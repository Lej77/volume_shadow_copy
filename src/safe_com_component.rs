pub use winapi::um::unknwnbase::IUnknown;

use std::{
    any::type_name,
    borrow::Borrow,
    fmt,
    ops::Deref,
    ptr::{self, NonNull},
    rc::Rc,
};

use once_cell::unsync::OnceCell;
use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::REFIID,
        minwindef::ULONG,
        winerror::{E_NOINTERFACE, S_OK},
    },
    um::winnt::HRESULT,
    Interface,
};

/// # Safety
///
/// Assert that the current type can be managed by the `IUnknown` interface.
pub unsafe trait AsIUnknown {
    fn as_unknown(&self) -> &IUnknown;
}

/// Forward calls to an `IUnknown` implementations.
///
/// # Safety
///
/// Assert that the current type can be managed by the `IUnknown` interface.
pub unsafe trait CustomIUnknown {
    /// Check if a COM object implements a specific interface.
    unsafe fn query_interface(&self, riid: REFIID, object: *mut *mut c_void) -> HRESULT;
    /// Increment reference count.
    unsafe fn add_ref(&self) -> ULONG;
    /// Decrement reference count.
    unsafe fn release(&self) -> ULONG;
}
unsafe impl<T> CustomIUnknown for T
where
    T: AsIUnknown,
{
    unsafe fn query_interface(&self, riid: REFIID, object: *mut *mut c_void) -> HRESULT {
        self.as_unknown().QueryInterface(riid, object)
    }
    unsafe fn add_ref(&self) -> ULONG {
        self.as_unknown().AddRef()
    }
    unsafe fn release(&self) -> ULONG {
        self.as_unknown().Release()
    }
}

/// # Safety
///
/// The IID returned by the `Interface` trait must be for an interface that
/// will be used correctly when cast into `Self`.
///
/// This means that if the specified interface id is successfully queried then
/// the returned interface can be safely used as the `Self` type.
pub unsafe trait CorrectInterface: Interface {}

/// Impl `AsIUnknown`
///
/// # Safety
///
/// The returned `IUnknown` interface must track the reference count for the
/// `Self` type.
///
/// The IID returned by the `Interface` trait must be for an interface that
/// will be used correctly when cast into `Self`.
macro_rules! _unsafe_impl_as_IUnknown {
    ($type:ty) => {
        unsafe impl $crate::safe_com_component::AsIUnknown for $type {
            fn as_unknown(&self) -> &$crate::safe_com_component::IUnknown {
                self
            }
        }
        unsafe impl $crate::safe_com_component::CorrectInterface for $type {}
    };
}
pub(crate) use _unsafe_impl_as_IUnknown as unsafe_impl_as_IUnknown;

pub struct SafeCOMComponent<T: CustomIUnknown> {
    /// The methods on the pointed to struct relies on the structs location
    /// so never try to move it (note also that most of the struct layout is
    /// likely hidden from us).
    comp: NonNull<T>,
    /// Track the reference count so we never overflow the native component's
    /// reference count. We can't just prevent the user from increasing the
    /// reference count since it must be increased when casting to a different
    /// interface.
    ///
    /// This can be more conservative than necessary since sometimes an interface
    /// that was queried from a component uses a different reference count than
    /// the component it was queried from.
    ///
    /// Note that the components reference count should be an `u32`, source:
    /// "The internal reference counter that AddRef maintains should be a 32-bit
    /// unsigned integer." from:
    /// https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref#remarks
    ///
    /// If the component isn't using an `u32` internally then this wrapper can
    /// still be unsound.
    ///
    /// Uses `OnceCell` to prevent an extra allocation if the the component's
    /// reference count is never larger than 1.
    ref_count: OnceCell<Rc<()>>,
}
impl<T: CustomIUnknown> SafeCOMComponent<T> {
    /// # Safety
    ///
    /// - The provided pointer must be a valid COM component.
    /// - The COM component's reference count should be `1`.
    pub unsafe fn new(comp: *mut T) -> Self {
        Self {
            comp: NonNull::new(comp).expect("component pointer was null"),
            ref_count: OnceCell::new(),
        }
    }
    /// Panics if another reference count increment could cause the reference
    /// count to overflow.
    fn check_if_overflowing_reference_count(&self) {
        let count = self.reference_count();
        debug_assert_ne!(count, 0);
        if count as u32 == u32::MAX {
            panic!(
                "tried to clone a COM component whose reference count was already {}, \
                    this panicked to prevent possible overflow of the component's reference \
                    count which should be a 32-bit unsigned integer.",
                count
            );
        }
    }
    fn get_rc(&self) -> &Rc<()> {
        self.ref_count.get_or_init(|| Rc::new(()))
    }
    pub fn reference_count(&self) -> usize {
        self.ref_count
            .get()
            .map(Rc::strong_count)
            // If we haven't created an `Rc` then the reference count has never
            // been incremented, so it must be `1`:
            .unwrap_or(1)
    }
    /// Queries a COM object for a pointer to one of its interface.
    #[doc(alias = "QueryInterface")]
    pub fn query_interface<I: CustomIUnknown + CorrectInterface>(
        &self,
    ) -> Option<SafeCOMComponent<I>> {
        // If `QueryInterface` is successful then it will increment the reference
        // count:
        // https://docs.microsoft.com/en-us/windows/win32/learnwin32/asking-an-object-for-an-interface
        // https://stackoverflow.com/questions/7376286/does-iunknownqueryinterface-increment-the-reference-count

        // Run arbitrary code to get uuid of interface:
        let iid = I::uuidof();
        // Ensure we don't overflow the internal reference count if we are successful:
        self.check_if_overflowing_reference_count();
        // Try to get the interface:
        let mut interface = ptr::null_mut();
        // Safety: the implementation of `AsIUnknown` for `I` promises to
        // behave correctly for an interface is queried using its IDD.
        let hr = unsafe { <T as CustomIUnknown>::query_interface(self, &iid, &mut interface) };
        if hr == S_OK {
            let comp = NonNull::new(interface as *mut I)
                .expect("returned interface pointer was null even though the return value as S_OK");

            // This will safely handle decrementing the interface's reference count
            // when dropped:
            let mut comp = SafeCOMComponent {
                comp,
                ref_count: OnceCell::new(),
            };
            // Increment our separate reference count (this might panic, who knows?)
            let ref_count = Rc::clone(self.get_rc());
            // Ensure our reference count is stored in the returned SafeCOMComponent:
            comp.ref_count = OnceCell::from(ref_count);
            Some(comp)
        } else if hr == E_NOINTERFACE {
            None
        } else {
            panic!("`QueryInterface` returned unexpected error code: {:#X}", hr);
        }
    }
}
impl<T: CustomIUnknown> Clone for SafeCOMComponent<T> {
    #[doc(alias = "AddRef")]
    fn clone(&self) -> Self {
        self.check_if_overflowing_reference_count();
        let ref_count = Rc::clone(self.get_rc());
        // Safety: we have tried to ensure that the internal reference count
        // never overflows and the wrapped pointer must still be valid so this
        // should be safe. Note that if the reference count is stored in something
        // smaller than an u32 this can still be unsound.
        unsafe {
            <T as CustomIUnknown>::add_ref(self);
        }
        Self {
            comp: self.comp,
            ref_count: OnceCell::from(ref_count),
        }
    }
}
impl<T: CustomIUnknown> Deref for SafeCOMComponent<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.comp.as_ref() }
    }
}
impl<T: CustomIUnknown> AsRef<T> for SafeCOMComponent<T> {
    fn as_ref(&self) -> &T {
        self
    }
}
impl<T: CustomIUnknown> Borrow<T> for SafeCOMComponent<T> {
    fn borrow(&self) -> &T {
        self
    }
}
impl<T: CustomIUnknown> Drop for SafeCOMComponent<T> {
    #[doc(alias = "Release")]
    fn drop(&mut self) {
        unsafe {
            <T as CustomIUnknown>::release(self);
        }
        // Rc is dropped after the component's reference count is decremented
        // so our reference count will never be less than the component's.
    }
}
impl<T: CustomIUnknown> fmt::Debug for SafeCOMComponent<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComComponent")
            .field("com_object", &type_name::<T>())
            .field("reference_count", &self.reference_count())
            .finish()
    }
}

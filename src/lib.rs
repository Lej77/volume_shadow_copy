//! Volume Shadow Copy Service API wrapper. Can be used to make a VSS
//! "requester" that uses VSS to make or restore backups.
//!
//! # Why it is useful
//!
//! There are many advantages to using the Volume Shadow Copy Service to preform
//! backups instead of just reading files directly. For example it ensures that
//! files are not modified while the they are being read and it can also notify
//! registered VSS "writers" that a backup is being preformed so that they can
//! prepares their data as appropriate, such as completing all open
//! transactions, rolling transaction logs, and flushing caches.
//!
//! Using the Volume Shadow Copy Service should also allow copying files that
//! have a [share mode] that doesn't [allow them to be read]. Even if a file can
//! be read, opening it can still cause issues since another process could try
//! to open it with a [share mode that doesn't allow another reader] and
//! therefore fail their open operation [causing issues for that other process].
//!
//! [share mode]:
//!     https://doc.rust-lang.org/std/os/windows/fs/trait.OpenOptionsExt.html#tymethod.share_mode
//! [allow them to be read]:
//!     https://stackoverflow.com/questions/3560651/whats-the-least-invasive-way-to-read-a-locked-file-in-c-sharp-perhaps-in-unsaf
//! [share mode that doesn't allow another reader]:
//!     https://stackoverflow.com/questions/11855245/unable-to-read-same-file-in-parallel
//! [causing issues for that other process]:
//!     https://stackoverflow.com/questions/10473442/why-cant-i-open-a-file-for-reading-if-theoretically-i-should-be-allowed
//!
//! # References
//!
//! This command-line tool could maybe be used to manage volume shadow copies:
//! [VShadow Tool and Sample - Win32 apps | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows/win32/vss/vshadow-tool-and-sample)
//!
//! High level overview of Volume Shadow Copy Service: [Volume Shadow Copy
//! Service | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows-server/storage/file-server/volume-shadow-copy-service)
//!
//! High level overview of Volume Shadow Copy Service: [Volume Shadow Copy
//! Service - Win32 apps | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows/win32/vss/volume-shadow-copy-service-portal)
//!
//! Hight level overview of a "requestor": [Requesters - Win32 apps | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows/win32/vss/requestors)
//!
//! Overview of what operations to preform when making a backup: [Overview of
//! Processing a Backup Under VSS - Win32 apps | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows/win32/vss/overview-of-processing-a-backup-under-vss)
//!
//! Documentation for "vsbackup.h" header that defines the API that "requestors"
//! uses: [Vsbackup.h header - Win32 apps | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows/win32/api/vsbackup/)
//!
//! List with relevant header files: [System Services - Win32 apps | Microsoft
//! Docs](https://docs.microsoft.com/en-us/windows/win32/api/_base/)
//!
//! Specification  for the Shadow Copy Management Protocol: [[MS-SCMP]: Shadow
//! Copy Management Protocol | Microsoft
//! Docs](https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-scmp/a1ab0e30-2dc1-49bb-8c46-4616ea09cc54)
//!
//! # License
//!
//! This project is released under either:
//!
//! - [MIT
//!   License](https://github.com/Lej77/volume_shadow_copy/blob/master/LICENSE-MIT)
//! - [Apache License (Version
//!   2.0)](https://github.com/Lej77/volume_shadow_copy/blob/master/LICENSE-APACHE)
//!
//! at your choosing.
//!
//! ## Copied content
//!
//! The above license might not apply to content copied from other sources. This
//! includes some documentation comments in the code as well as error info
//! inside the `errors.md` file used to generate [error types](errors). The
//! example code in `examples/make-backup-snapshot.rs` was inspired by code from
//! the [backup program `restic`] and so might be affected by its license.
//!
//! [backup program `restic`]:
//!     https://github.com/restic/restic/blob/db8a95899114ef5131818462d057cac202189b3a/internal/fs/vss_windows.go#L763-L777

#![warn(clippy::all)]
#![warn(rustdoc::broken_intra_doc_links)]
// TODO: enable these warnings.
// #![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms),
        allow(unused_extern_crates, unused_variables)
    )
))]

pub mod vsadmin;
pub mod vsbackup;
pub mod vsmgmt;
pub mod vsprov;
pub mod vss;
pub mod vswriter;

mod safe_com_component;
use safe_com_component::{
    unsafe_impl_as_IUnknown, CorrectInterface, CustomIUnknown, SafeCOMComponent,
};

use std::{
    error::Error as StdError,
    fmt,
    mem::{transmute, ManuallyDrop},
    ops::Deref,
    ptr::NonNull,
};

use widestring::U16CStr;
use winapi::{
    ctypes::c_void,
    shared::{winerror::S_OK, wtypes::BSTR, wtypesbase::OLECHAR},
    um::{combaseapi::CoTaskMemFree, oleauto::SysFreeString, vss::VSS_PWSZ, winnt::HRESULT},
};
use winstr::{BStr, BString};

pub use widestring;
pub use winapi::um::vss::VSS_ID;
pub use winstr;

/// An owned null-terminated wide character string returned from a VSS function.
pub struct VssU16CString(VSS_PWSZ);
impl VssU16CString {
    /// Take ownership of the provided string.
    ///
    /// # Safety
    ///
    /// - It must be safe to pass the provided pointer to the
    ///   `winapi::um::combaseapi::CoTaskMemFree` function.
    /// - The pointer must be valid.
    ///     - The pointer must therefore not be null.
    /// - The data that is pointed to must be nul-terminated.
    pub unsafe fn from_ptr(string: VSS_PWSZ) -> Self {
        Self(string)
    }
    /// Take ownership of the provided string.
    ///
    /// # Safety
    ///
    /// - It must be safe to pass the provided pointer to the
    ///   `winapi::um::combaseapi::CoTaskMemFree` function.
    /// - The pointer must be valid.
    ///     - A null pointer is okay though in which case `None` is returned.
    /// - The data that is pointed to must be nul-terminated.
    pub unsafe fn from_nullable_ptr(string: VSS_PWSZ) -> Option<Self> {
        if string.is_null() {
            None
        } else {
            Some(Self::from_ptr(string))
        }
    }
}
impl Deref for VssU16CString {
    type Target = U16CStr;
    fn deref(&self) -> &Self::Target {
        unsafe { U16CStr::from_ptr_str(self.0) }
    }
}
impl Drop for VssU16CString {
    fn drop(&mut self) {
        // Safety: the `new` method promised that this would be safe.
        unsafe { CoTaskMemFree(self.0 as *mut c_void) }
    }
}

/// Error returned by [`take_ownership_of_bstr`]. This will free the wrapped
/// `BSTR`. Use the [`take_back_bstr`](TakeBStringError::take_back_bstr)
/// method to prevent freeing the `BSTR`.
pub struct TakeBStringError {
    bstr: BSTR,
}
impl TakeBStringError {
    /// Take back ownership of the `BSTR`. If this isn't called then the wrapped
    /// `BSTR` will be freed when this error is dropped.
    pub fn take_back_bstr(self) -> BSTR {
        let this = ManuallyDrop::new(self);
        this.bstr
    }
}
impl Drop for TakeBStringError {
    fn drop(&mut self) {
        if !self.bstr.is_null() {
            unsafe {
                SysFreeString(self.bstr);
            }
        }
    }
}
impl fmt::Debug for TakeBStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length of BSTR was close to overflowing")
    }
}
impl fmt::Display for TakeBStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
impl StdError for TakeBStringError {}

/// Wrap a raw `BSTR` in a `BString` type, taking ownership of the pointed to memory.
/// Returns `None` if the provided pointer is `null`.
///
/// Note that even if this method returns `Err(_)` it still takes ownership of the
/// memory and will free it unless the error's [`take_back_bstr`](TakeBStringError::take_back_bstr)
/// method is used.
///
/// # Safety
///
/// Must uphold the safety requirements of [`BStr::from_bstr_unbounded`] and also
/// must be safe to free via [`SysFreeString`](winapi::um::oleauto::SysFreeString).
pub unsafe fn take_ownership_of_bstr(bstr: BSTR) -> Result<Option<BString>, TakeBStringError> {
    let bstr = if let Some(v) = BStr::from_bstr_unbounded(bstr) {
        v
    } else if bstr.is_null() {
        return Ok(None);
    } else {
        return Err(TakeBStringError { bstr });
    };
    // We rely on the following internal definitions of winstr. We are pinned to a
    // specific winstr version to ensure this is safe.
    //
    // Internal details of the `winstr` crate that we rely upon:
    // ```
    // use winapi::shared::wtypesbase::OLECHAR;
    // use winapi::um::oleauto::SysFreeString;
    // use std::ptr::NonNull;
    //
    // #[repr(transparent)] pub struct BStr(OLECHAR);
    // #[repr(transparent)] pub struct BString(NonNull<OLECHAR>);
    //
    // impl Drop for BString {
    //     fn drop(&mut self) { unsafe { SysFreeString(self.0.as_ptr()) }; }
    // }
    // ```
    let bstr = transmute::<&BStr, &OLECHAR>(bstr);
    let bstr = NonNull::<OLECHAR>::from(bstr);
    Ok(Some(transmute::<NonNull<OLECHAR>, BString>(bstr)))
}

pub mod errors {
    //! Errors that enumerate expected error conditions for different methods.
    use std::{error::Error as StdError, fmt};

    use winapi::um::winnt::HRESULT;

    mod all_errors {
        //! Used in code generated by build script to access errors.
        pub use winapi::{shared::winerror::*, um::vsserror::*};
    }
    include!(concat!(env!("OUT_DIR"), "/errors.rs"));
}

/// Convert a `HRESULT` into a `Result`. If the value is `S_OK` then returns `Ok(())`,
/// otherwise returns `Err(code)` where code is the provided `HRESULT`.
fn check_com(hr: HRESULT) -> Result<(), HRESULT> {
    if hr == S_OK {
        Ok(())
    } else {
        Err(hr)
    }
}

/// Initializes the COM library for use by the calling thread.
///
/// Note: this uses the default `COINIT_MULTITHREADED` flag to initialize the
/// COM library.
///
/// See the wrapped methods docs at:
/// [CoInitializeEx function (combaseapi.h) - Win32 apps | Microsoft Docs](https://docs.microsoft.com/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex?redirectedfrom=MSDN)
#[doc(alias = "CoInitializeEx")]
pub fn initialize_com() -> Result<(), errors::CoInitializeExError> {
    check_com(unsafe {
        winapi::um::combaseapi::CoInitializeEx(
            std::ptr::null_mut(),
            winapi::um::objbase::COINIT_MULTITHREADED,
        )
    })?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
// Cast interfaces
////////////////////////////////////////////////////////////////////////////////

mod sealed {
    use super::*;

    /// This trait is sealed.
    pub trait QueryInterfaceInternal {
        fn query_interface<T: CustomIUnknown + CorrectInterface>(
            &self,
        ) -> Option<SafeCOMComponent<T>>;
    }
    pub trait InterfaceInternal {
        type Inner: CustomIUnknown + CorrectInterface;
        fn from_safe_com_component(com: SafeCOMComponent<Self::Inner>) -> Self;
    }
}

/// A type that represents an interface that can be queried from another interface.
///
/// This trait is sealed and can't be implemented.
pub trait Interface: sealed::InterfaceInternal {}

/// Allow querying for an interface. If the interface exists then the type's
/// internal reference count is incremented and the new interface is returned.
///
/// This trait is sealed and can't be implemented.
pub trait QueryInterface: sealed::QueryInterfaceInternal {
    fn query<T: Interface>(&self) -> Option<T> {
        self.query_interface::<T::Inner>()
            .map(T::from_safe_com_component)
    }
}

/// Implement `QueryInterface` for a type to allow casting it to other interfaces.
///
/// If `=> InnerType` isn't specified then it will only be possible to query the
/// provided for other interfaces but never to cast other interfaces into the
/// current type.
macro_rules! _impl_query_interface {
    (
        $type:ident
        $(<$($life:lifetime),* $(,)?>)?
        $(=>
        $inner:ty)?
    ) => {
        impl$(<$($life),*>)? $crate::sealed::QueryInterfaceInternal for $type$(<$($life),*>)? {
            fn query_interface<T: $crate::safe_com_component::CustomIUnknown + $crate::safe_com_component::CorrectInterface>(
                &self
            ) -> Option<$crate::safe_com_component::SafeCOMComponent<T>> {
                self.0.query_interface::<T>()
            }
        }
        impl$(<$($life),*>)? $crate::QueryInterface for $type$(<$($life),*>)? {}

        $crate::impl_query_interface!(@if ($(true $inner)?) {
            impl$(<$($life),*>)? $crate::sealed::InterfaceInternal for $type$(<$($life),*>)? {
                type Inner = $($inner)?;
                fn from_safe_com_component(com: SafeCOMComponent<Self::Inner>) -> Self {
                    Self(
                        com,
                        $({
                            $(let _: &$life ();)*
                            PhantomData
                        })?
                    )
                }
            }
            impl$(<$($life),*>)? $crate::Interface for $type$(<$($life),*>)? {}
        } else {});
    };
    // Utilities:
    (@if (true $($condition:tt)* ) { $($true:tt)* } else { $($false:tt)* }) => { $($true)* };
    (@if ( $($condition:tt)* ) { $($true:tt)* } else { $($false:tt)* }) => { $($false)* };
}
pub(crate) use _impl_query_interface as impl_query_interface;

////////////////////////////////////////////////////////////////////////////////
// Helper macros for COM
////////////////////////////////////////////////////////////////////////////////

/// Create a transparent new type over another type and provide an unsafe method to
/// go from a reference of the inner type to a reference of the new type.
///
/// Useful to ensure that the wrapped COM object is in a valid state.
macro_rules! _transparent_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident
        $(<$($life:lifetime),* $(,)?>)?
        (
            $(#[$inner_attr:meta])*
            $inner_vis:vis $inner:ty
            $(,
                $(#[$extra_attr:meta])*
                $extra_vis:vis $extra:ty
            )*
            $(,)?
        );
    ) => {
        $(#[$attr])*
        #[repr(transparent)]
        $vis struct $name
        $(<$($life),*>)?
        (
            $(#[$inner_attr])*
            $inner_vis $inner
            $(,
                $(#[$extra_attr])*
                $extra_vis $extra
            )*
        );

        // `const_assert` from `static-assertions` crate to check that size of extra fields are zero:
        #[allow(unknown_lints)]
        const _: [(); 0 - !{ const ASSERT: bool = $name::__EXTRA_SIZE == 0; ASSERT } as usize] = [];

        impl$(<$($life),*>)? $name$(<$($life),*>)? {
            const __EXTRA_SIZE: usize = {
                0 $(+ std::mem::size_of::<$extra>())*
            };
            /// Create this wrapper from its unsafe equivalent.
            ///
            /// # Safety
            ///
            /// The inner type must be in a valid state (not `Released`). (Also if
            /// this struct has any lifetimes then those must be correct as well.)
            pub unsafe fn from_inner_ref<'__a>(inner: &'__a $inner) -> &'__a Self {
                // Safety: this type is a transparent wrapper around the inner type.
                &* (inner as *const $inner as *const Self)
            }
            /// Get the inner unsafe equivalent of this component.
            ///
            /// # Safety
            ///
            /// Don't do anything to invalidate the wrapped value. (So don't call
            /// `Release` to decrement the reference count but incrementing the
            /// reference count can also cause unsafety since that could allow
            /// safe code to overflow the interface's internal reference count.
            /// This library keeps its own separate reference count to ensure
            /// this doesn't happen so if the interface's reference count is
            /// incremented elsewhere then that protection won't work anymore.)
            pub unsafe fn into_inner_ref<'__a>(this: &'__a Self) -> &'__a $inner {
                &this.0
            }
        }
    };
}
pub(crate) use _transparent_wrapper as transparent_wrapper;

/// Implement `AsRef` and `Borrow` traits to match the type that this type `Deref`
/// into.
macro_rules! _impl_as_ref_and_borrow {
    (
        $source:ident
        $(<$($source_life:lifetime),* $(,)?>)?
        =>
        $target:ident
        $(<$($target_life:lifetime),* $(,)?>)?
    ) => {
        impl$(<$($source_life),*>)? AsRef<$target $(<$($target_life),*>)?> for $source $(<$($source_life),*>)? {
            fn as_ref(&self) -> &$target $(<$($target_life),*>)? {
                // Will deref as needed:
                self
            }
        }
        impl<$($($source_life,)*)? T> AsRef<T> for $source $(<$($source_life),*>)?
        where
            $target $(<$($target_life),*>)?: AsRef<T>,
        {
            fn as_ref(&self) -> &T {
                <$target as AsRef<T>>::as_ref(&self)
            }
        }
        impl<$($($source_life,)*)? T> ::core::borrow::Borrow<T> for $source $(<$($source_life),*>)?
        where
            $target $(<$($target_life),*>)?: ::core::borrow::Borrow<T>,
        {
            fn borrow(&self) -> &T {
                <$target $(<$($target_life),*>)? as ::core::borrow::Borrow<T>>::borrow(&self)
            }
        }
        impl<$($($source_life,)*)? '__a, T> ::core::borrow::Borrow<T> for &'__a $source $(<$($source_life),*>)?
        where
            $target $(<$($target_life),*>)?: ::core::borrow::Borrow<T>,
        {
            fn borrow(&self) -> &T {
                <$target $(<$($target_life),*>)? as ::core::borrow::Borrow<T>>::borrow(&self)
            }
        }
    };
}
pub(crate) use _impl_as_ref_and_borrow as impl_as_ref_and_borrow;

/// Implement `Deref` for `owned` into the specified `$ref` type.
///
/// # Safety
///
/// The COM object inside the `owned` type must be in a valid state (not released).
macro_rules! _unsafe_deref_to_ref {
    (
        $source:ident
        $(<$($source_life:lifetime),* $(,)?>)?
        =>
        $target:ident
        $(<$($target_life:lifetime),* $(,)?>)?
    ) => {
        impl$(<$($source_life),*>)? ::core::ops::Deref for $source $(<$($source_life),*>)? {
            type Target = $target $(<$($target_life),*>)?;
            fn deref(&self) -> &Self::Target {
                // Safety: the inner type is not released.
                unsafe { Self::Target::from_inner_ref(&self.0) }
            }
        }
        $crate::impl_as_ref_and_borrow!(
            $source
            $(<$($source_life,)*>)?
            =>
            $target
            $(<$($target_life,)*>)?
        );
    };
    (
        $source:ident
        $(<$($source_life:lifetime),* $(,)?>)?
        =>
        $target:ident
        $(<$($target_life:lifetime),* $(,)?>)?
        $(
            =>
            $next:ident
            $(<$($next_life:lifetime),* $(,)?>)?
        )+
    ) => {
        unsafe_deref_to_ref!($source $(<$($source_life),*>)? => $target $(<$($target_life),*>)?);
        unsafe_deref_to_ref!($target $(<$($target_life),*>)? => $( $next $(<$($next_life),*>)? )+);
    };
}
pub(crate) use _unsafe_deref_to_ref as unsafe_deref_to_ref;

////////////////////////////////////////////////////////////////////////////////
// Raw bitflags
////////////////////////////////////////////////////////////////////////////////

/// Implemented for typed bitflags APIs to allow dealing with flags that aren't
/// defined.
pub trait AsRawBitFlags {
    /// The raw representation of the bitflags.
    type Raw: Copy;
    /// Create a typed API from a raw value.
    fn from_raw(raw: Self::Raw) -> Self;
    /// Get the raw value that represents the typed APIs flags.
    fn into_raw(this: Self) -> Self::Raw;
}

/// A wrapper that allows lossless handling of bitflags while still exposing a
/// typed API.
pub struct RawBitFlags<T: AsRawBitFlags> {
    raw: <T as AsRawBitFlags>::Raw,
}
impl<T> RawBitFlags<T>
where
    T: AsRawBitFlags,
{
    pub fn new(flags: T) -> Self {
        Self {
            raw: T::into_raw(flags),
        }
    }
    pub fn from_raw(raw: T::Raw) -> Self {
        Self { raw }
    }
    /// An untyped version of the flags.
    pub fn raw(self) -> T::Raw {
        self.raw
    }
    /// A typed version of the flags. Might truncate some flags if they aren't
    /// defined.
    pub fn flags(self) -> T {
        T::from_raw(self.raw)
    }
}
impl<T> Clone for RawBitFlags<T>
where
    T: AsRawBitFlags,
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for RawBitFlags<T> where T: AsRawBitFlags {}
impl<T> fmt::Debug for RawBitFlags<T>
where
    T: AsRawBitFlags + fmt::Debug,
    T::Raw: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(RawBitFlags))
            .field("raw", &self.raw)
            .field("flags", &self.flags())
            .finish()
    }
}
impl<T> Default for RawBitFlags<T>
where
    T: AsRawBitFlags + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
impl<T> From<T> for RawBitFlags<T>
where
    T: AsRawBitFlags,
{
    fn from(flags: T) -> Self {
        Self::new(flags)
    }
}

/// Forwards to `bitflags::bitflags` macro so does the same as that one but also
/// implements the `AsRawBitFlags` so that the generated type can be used with
/// our `RawBitFlags` type.
macro_rules! _raw_bitflags {
    (@impl_raw
        $(#[$attr:meta])*
        $vis:vis struct $name:ident: $repr:ty { $($rest:tt)* }
    ) => {
        impl $crate::AsRawBitFlags for $name {
            type Raw = $repr;
            fn from_raw(raw: Self::Raw) -> Self {
                Self::from_bits_truncate(raw)
            }
            fn into_raw(this: Self) -> Self::Raw {
                Self::bits(&this)
            }
        }
    };
    ($($t:tt)*) => {
        bitflags::bitflags! {
            $($t)*
        }
        $crate::raw_bitflags!(@impl_raw $($t)*);
    };
}
pub(crate) use _raw_bitflags as raw_bitflags;

////////////////////////////////////////////////////////////////////////////////
// Macros for enums
////////////////////////////////////////////////////////////////////////////////

/// Implement `From` to convert between an enum and an integer type.
macro_rules! _with_from {
    // Define the enum and convert from the an integer type to the typed enum.
    (@from_raw
        raw_type = { $as_type:ty },
        fallback = { $($fallback_variant:ident)? },
        default = { $($default_variant:ident)? },
        all = {
            $(#[$attr:meta])*
            $vis:vis enum $name:ident {
                $(
                    $(#[$variant_attr:meta])*
                    $variant:ident = $value:pat
                ),* $(,)?
            }
        },
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $(
                $(#[$variant_attr])*
                $variant,
            )*
        }
        $crate::with_from!(@if ($(true $default_variant)?) {
            impl ::core::default::Default for $name {
                fn default() -> Self {
                    $(Self::$default_variant)?
                }
            }
        } else {});
        $crate::with_from!(@if ($(true $fallback_variant)?) {
            // Fallback variant specified:
            impl ::core::convert::From<$as_type> for $name {
                fn from(value: $as_type) -> Self {
                    match value {
                        $(
                            $value => Self::$variant,
                        )*
                        _ => $(Self::$fallback_variant)?,
                    }
                }
            }
        } else {
            // No default variant so implement TryFrom:
            impl ::core::convert::TryFrom<$as_type> for $name {
                type Error = ();
                fn try_from(value: $as_type) -> ::core::result::Result<Self, Self::Error> {
                    match value {
                        $(
                            $value => Ok(Self::$variant),
                        )*
                        _ => Err(()),
                    }
                }
            }
        });
    };
    // Convert from the typed enum to an integer type.
    (@to_raw
        raw_type = { $as_type:ty },
        all = {
            $(#[$attr:meta])*
            $vis:vis enum $name:ident {
                $(
                    $(#[$variant_attr:meta])*
                    $variant:ident = $value:expr
                ),* $(,)?
            }
        },
    ) => {
        impl ::core::convert::From<$name> for $as_type {
            fn from(value: $name) -> Self {
                match value {
                    $(
                        $name::$variant => $value,
                    )*
                }
            }
        }
    };
    // Utilities:
    (@if (true $($condition:tt)* ) { $($true:tt)* } else { $($false:tt)* }) => { $($true)* };
    (@if ( $($condition:tt)* ) { $($true:tt)* } else { $($false:tt)* }) => { $($false)* };
    // Entry point:
    (
        [
            raw = $as_type:ty
            $(,fallback = $fallback:ident)?
            $(,default = $default:ident)?
            $(,)?
        ],
        $($t:tt)*
    ) => {
        $crate::with_from!(@from_raw
            raw_type = { $as_type },
            fallback = { $($fallback)? },
            default = { $($default)? },
            all = { $($t)* },
        );
        $crate::with_from!(@to_raw
            raw_type = { $as_type },
            all = { $($t)* },
        );
    };
}
pub(crate) use _with_from as with_from;

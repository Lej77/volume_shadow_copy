//! Bindings for the `Vss.h` header.
//!
//! This is required when developing a VSS "writer" or a VSS "requester".
//!
//! # References
//!
//! [Vss.h header - Win32 apps | Microsoft Docs](https://docs.microsoft.com/en-us/windows/win32/api/vss/)

use std::{
    borrow::Borrow,
    convert::{TryFrom, TryInto},
    error::Error as StdError,
    fmt,
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::{self, null_mut},
};

use widestring::U16CStr;
use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::GUID,
        minwindef::ULONG,
        winerror::{S_FALSE, S_OK},
    },
    um::{
        combaseapi::CoTaskMemFree,
        vsbackup,
        vss::{self, VSS_ID},
        vsserror,
        winbase::INFINITE,
        winnt::{HRESULT, LONG},
    },
};

use super::{
    check_com, errors::*, impl_query_interface, raw_bitflags, transparent_wrapper,
    unsafe_deref_to_ref, unsafe_impl_as_IUnknown, with_from, RawBitFlags, SafeCOMComponent,
};

////////////////////////////////////////////////////////////////////////////////
// AsyncError
////////////////////////////////////////////////////////////////////////////////

pub type IVssAsyncResult<E> = Result<VssAsync<E>, E>;

/// An error from the [`VssAsync`] interface. Can be caused by the interface
/// itself or because of the method that created and returned the [`VssAsync`]
/// interface.
pub struct VssAsyncError<A, E> {
    hresult: HRESULT,
    error_types: PhantomData<(A, E)>,
}
impl<A, E> VssAsyncError<A, E>
where
    A: From<HRESULT>,
{
    /// Interpret the error code as a failure of the method of the [`VssAsync`]
    /// interface itself.
    pub fn async_error(self) -> A {
        self.hresult.into()
    }
}
impl<A, E> VssAsyncError<A, E>
where
    E: From<HRESULT>,
{
    /// Interpret the error code as the original method that created the [`VssAsync`]
    /// interface would have.
    pub fn underlying_error(self) -> E {
        self.hresult.into()
    }
}
impl<A, E> Clone for VssAsyncError<A, E> {
    fn clone(&self) -> Self { *self }
}
impl<A, E> Copy for VssAsyncError<A, E> {}
impl<A, E> fmt::Debug for VssAsyncError<A, E>
where
    A: From<HRESULT> + fmt::Debug,
    E: From<HRESULT> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(stringify!(VssAsyncError))
            .field("hresult", &self.hresult)
            .field("async_error", &self.async_error())
            .field("underlying_error", &self.underlying_error())
            .finish()
    }
}
impl<A, E> fmt::Display for VssAsyncError<A, E>
where
    A: From<HRESULT> + fmt::Display,
    E: From<HRESULT> + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (HRESULT: {:#X}): caused by an async error `{}` or an underlying error `{}`",
            stringify!(VssAsyncError),
            self.hresult,
            self.async_error(),
            self.underlying_error(),
        )
    }
}
impl<A, E> StdError for VssAsyncError<A, E>
where
    A: From<HRESULT> + fmt::Debug + fmt::Display,
    E: From<HRESULT> + fmt::Debug + fmt::Display,
{
}
impl<A, E> core::convert::From<HRESULT> for VssAsyncError<A, E> {
    fn from(value: HRESULT) -> Self {
        Self {
            hresult: value,
            error_types: PhantomData,
        }
    }
}
impl<A, E> core::convert::From<VssAsyncError<A, E>> for HRESULT {
    fn from(value: VssAsyncError<A, E>) -> Self {
        value.hresult
    }
}

////////////////////////////////////////////////////////////////////////////////
// IVssAsync
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vss::IVssAsync);

#[must_use = "this type represents an async operation which should probably be waited on"]
#[doc(alias = "IVssAsync")]
#[derive(Debug, Clone)]
pub struct VssAsync<E>(SafeCOMComponent<vss::IVssAsync>, PhantomData<E>);
impl<E> VssAsync<E> {
    pub(crate) fn new(com: SafeCOMComponent<vss::IVssAsync>) -> Self {
        Self(com, PhantomData)
    }
}
/// Change the error type of the operation.
impl<E> VssAsync<E> {
    /// Return operation errors as a raw error code instead of converting them
    /// into a more typed API. This can be useful to store several different
    /// kinds of `IVssAsync` in the same `Vec` for example.
    pub fn untyped_errors(self) -> VssAsync<HRESULT> {
        VssAsync(self.0, PhantomData)
    }
    /// Change the error type.
    pub fn errors_as<E2>(self) -> VssAsync<E2> {
        VssAsync(self.0, PhantomData)
    }
}
impl<E> VssAsync<E>
where
    E: From<HRESULT>,
{
    /// Waits until an incomplete asynchronous operation finishes.
    #[doc(alias = "Wait")]
    pub fn wait(&self, milliseconds: Option<u32>) -> Result<(), VssAsyncError<WaitError, E>> {
        let milliseconds = milliseconds.unwrap_or(INFINITE);
        check_com(unsafe { self.0.Wait(milliseconds) })?;
        Ok(())
    }
    /// Queries the status of an asynchronous operation.
    #[doc(alias = "QueryStatus")]
    pub fn query_status(&self) -> Result<AsyncStatus, VssAsyncError<QueryStatusError, E>> {
        let mut result: HRESULT = S_OK;
        check_com(unsafe { self.0.QueryStatus(&mut result, null_mut()) })?;
        Ok(AsyncStatus::try_from(result).map_err(|_| result)?)
    }
    /// Cancel an incomplete asynchronous operation.
    #[doc(alias = "Cancel")]
    pub fn cancel(&self) -> Result<(), VssAsyncError<CancelError, E>> {
        check_com(unsafe { self.0.Cancel() })?;
        Ok(())
    }
}

with_from!(
    [raw = HRESULT],
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum AsyncStatus {
        /// The asynchronous operation was canceled by a previous call to
        /// [`VssAsync::cancel`].
        #[doc(alias = "VSS_S_ASYNC_CANCELLED")]
        Canceled = vsserror::VSS_S_ASYNC_CANCELLED,
        /// The asynchronous operation was completed successfully.
        #[doc(alias = "VSS_S_ASYNC_FINISHED")]
        Finished = vsserror::VSS_S_ASYNC_FINISHED,
        /// The asynchronous operation is still running.
        #[doc(alias = "VSS_S_ASYNC_PENDING")]
        Pending = vsserror::VSS_S_ASYNC_PENDING,
    }
);

////////////////////////////////////////////////////////////////////////////////
// IVssEnumObject
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vss::IVssEnumObject);

/// Contains methods to iterate over and perform other operations on a list of
/// enumerated objects.
///
/// The [`IBackupComponents::query`] method returns an `EnumObject` object.
///
/// [`IBackupComponents::query`]: crate::vsbackup::IBackupComponents::query
#[doc(alias = "IVssEnumObject")]
#[derive(Debug, Clone)]
pub struct EnumObject(pub(crate) SafeCOMComponent<vss::IVssEnumObject>);
impl_query_interface!(EnumObject => vss::IVssEnumObject);
transparent_wrapper!(
    #[doc(alias = "IVssEnumObject")]
    pub struct IEnumObject(vss::IVssEnumObject);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(EnumObject => IEnumObject);

impl IEnumObject {
    /// Creates a copy of the specified list of enumerated elements by creating
    /// a copy of the `EnumObject` enumerator object.
    #[doc(alias = "Clone")]
    pub fn clone_enumerator(&self) -> Result<EnumObject, EnumObjectCloneError> {
        let mut enumerator: *mut vss::IVssEnumObject = null_mut();
        check_com(unsafe { self.0.Clone(&mut enumerator) })?;
        Ok(EnumObject(unsafe { SafeCOMComponent::new(enumerator) }))
    }
    /// Returns the specified number of objects from the specified list of
    /// enumerated objects.
    ///
    /// # Leaks memory
    ///
    /// The provided buffer should not contain already initialized objects since
    /// their memory might be leaked (leaking is still memory safe so this method
    /// isn't `unsafe`). Use the [`ObjectProperties::free`] method to free all
    /// memory for the initialized  `ObjectProperties` to ensure no memory is
    /// leaked.
    #[doc(alias = "Next")]
    pub fn next(
        &self,
        buffer: &mut [ObjectProperties],
    ) -> Result<EnumObjectNextResult, EnumObjectNextError> {
        let wanted = buffer.len().try_into().unwrap_or(ULONG::MAX);
        let mut fetched: ULONG = 0;

        // Note that `ObjectProperties` is a transparent wrapper around `vss::VSS_OBJECT_PROP`.
        let buffer_ptr = buffer.as_mut_ptr() as *mut vss::VSS_OBJECT_PROP;

        let hr = unsafe { self.0.Next(wanted, buffer_ptr, &mut fetched) };
        let done = hr == S_FALSE;
        if done || hr == S_OK {
            Ok(EnumObjectNextResult {
                fetched: fetched as usize,
                done,
            })
        } else {
            Err(hr.into())
        }
    }
    /// Resets the enumerator so that [`IEnumObject::next`] starts at the first
    /// enumerated object.
    #[doc(alias = "Reset")]
    pub fn reset(&self) -> Result<(), EnumObjectResetError> {
        check_com(unsafe { self.0.Reset() })?;
        Ok(())
    }
    /// Skips the specified number of objects.
    ///
    /// Returns `true` if an attempt was made to access a location beyond the
    /// end of the list of items; otherwise returns `false`.
    #[doc(alias = "Skip")]
    pub fn skip(&self, element_count: u32) -> Result<bool, EnumObjectSkipError> {
        let hr = unsafe { self.0.Skip(element_count) };
        let too_far = hr == S_FALSE;
        if hr == S_OK || too_far {
            Ok(too_far)
        } else {
            Err(hr.into())
        }
    }
}
impl IEnumObject {
    /// Return an iterator that reads from this object.
    ///
    /// See the [`EnumObjectIterator::new`] method for more information.
    pub fn iter(&self, buffer_size: usize) -> EnumObjectIterator<&'_ Self> {
        EnumObjectIterator::new(self, buffer_size)
    }
    /// Return an iterator that reads from this object.
    ///
    /// See the [`EnumObjectIterator::new`] method for more information.
    pub fn into_iter(self, buffer_size: usize) -> EnumObjectIterator<Self> {
        EnumObjectIterator::new(self, buffer_size)
    }
}

enum EnumObjectIteratorBuffer {
    /// Optimization to not require allocation when buffer is small.
    Inlined {
        buffer: [ObjectProperties; Self::INLINED_SIZE],
        wanted_size: usize,
    },
    Heap(Vec<ObjectProperties>),
}
impl EnumObjectIteratorBuffer {
    fn new(buffer_size: usize) -> Self {
        assert_ne!(
            buffer_size, 0,
            "the EnumObjectIterator can't have a buffer size of zero"
        );

        if buffer_size > Self::INLINED_SIZE {
            let mut b = Vec::with_capacity(buffer_size);
            b.resize_with(buffer_size, Default::default);
            Self::Heap(b)
        } else {
            Self::Inlined {
                buffer: Default::default(),
                wanted_size: buffer_size,
            }
        }
    }
    fn as_mut_slice(&mut self) -> &mut [ObjectProperties] {
        match self {
            Self::Inlined {
                buffer,
                wanted_size,
            } => &mut buffer[..*wanted_size],
            Self::Heap(v) => v,
        }
    }
}
impl EnumObjectIteratorBuffer {
    pub const INLINED_SIZE: usize = 2;
}
pub struct EnumObjectIterator<T> {
    enumerator: T,
    buffer: EnumObjectIteratorBuffer,
    position: usize,
    length: usize,
}
impl<T> EnumObjectIterator<T> {
    /// Return an iterator that reads from the enumerator.
    ///
    /// Be aware that if the iterator is dropped and the `buffer_size` isn't `1`
    /// then some elements might have been stored inside the iterator which will
    /// therefore be skipped if a new iterator is created after.
    ///
    /// # Panics
    ///
    /// If the specified buffer size is 0 then this function will panic.
    pub fn new(enumerator: T, buffer_size: usize) -> Self {
        Self {
            enumerator,
            buffer: EnumObjectIteratorBuffer::new(buffer_size),
            position: 0,
            length: 0,
        }
    }
}

impl<T> Iterator for EnumObjectIterator<T>
where
    T: Borrow<IEnumObject>,
{
    type Item = Result<ObjectProperties, EnumObjectNextError>;

    fn next(&mut self) -> Option<Self::Item> {
        let buffer = self.buffer.as_mut_slice();
        if self.position < self.length {
            let value = buffer[self.position].take();
            self.position += 1;
            Some(Ok(value))
        } else if self.position > 0 && self.length < buffer.len() {
            // We have read to buffer at least once but the latest read
            // couldn't fill the buffer so we must be done:
            None
        } else {
            debug_assert_ne!(buffer.len(), 0);
            let info = match self.enumerator.borrow().next(buffer) {
                Ok(info) => info,
                Err(e) => return Some(Err(e)),
            };
            self.length = info.fetched;
            self.position = 1;
            let value = buffer[..self.length].get_mut(0)?.take();
            Some(Ok(value))
        }
    }
}

/// Info returned by the [`IEnumObject::next`] method.
pub struct EnumObjectNextResult {
    /// The number of elements that were written to the provided buffer.
    pub fetched: usize,
    /// `true` if the end of the enumeration list has been reached; otherwise `false`.
    pub done: bool,
}

#[doc(alias = "VSS_OBJECT_UNION")]
pub enum ObjectUnion {
    Snapshot(SnapshotProperties),
    Provider(ProviderProperties),
}
#[doc(alias = "VSS_OBJECT_UNION")]
pub enum ObjectUnionRef<'a> {
    Snapshot(&'a SnapshotProperties),
    Provider(&'a ProviderProperties),
}
#[doc(alias = "VSS_OBJECT_UNION")]
pub enum ObjectUnionMut<'a> {
    Snapshot(&'a mut SnapshotProperties),
    Provider(&'a mut ProviderProperties),
}

/// Contains the properties of a shadow copy or shadow copy set.
#[doc(alias = "VSS_SNAPSHOT_PROP")]
#[repr(transparent)]
pub struct SnapshotProperties(pub(crate) vss::VSS_SNAPSHOT_PROP);
/// Field getters.
impl SnapshotProperties {
    /// A VSS_ID (GUID) uniquely identifying the shadow copy identifier.
    #[doc(alias = "m_SnapshotId")]
    pub fn snapshot_id(&self) -> VSS_ID {
        self.0.m_SnapshotId
    }
    /// A VSS_ID (GUID) uniquely identifying the shadow copy set containing the
    /// shadow copy.
    #[doc(alias = "m_SnapshotSetId")]
    pub fn snapshot_set_id(&self) -> VSS_ID {
        self.0.m_SnapshotSetId
    }
    /// Number of volumes included with the shadow copy in the shadow copy set
    /// when it was created. Because it is possible for applications to release
    /// individual shadow copies without releasing the shadow copy set, at any
    /// given time the number of shadow copies in the shadow copy set may be less
    /// than `snapshots_count`.
    ///
    /// The maximum number of shadow-copied volumes permitted in a shadow copy
    /// set is 64.
    #[doc(alias = "m_lSnapshotsCount")]
    pub fn snapshots_count(&self) -> i32 {
        self.0.m_lSnapshotsCount
    }
    /// The name of the device object for the shadow copy of the volume. The
    /// device object can be thought of as the root of a shadow copy of a volume.
    /// Requesters will use this device name when accessing files on a shadow-copied
    /// volume that it needs to work with.
    ///
    /// The device name does not contain a trailing "\".
    #[doc(alias = "m_pwszSnapshotDeviceObject")]
    pub fn snapshot_device_object(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0.m_pwszSnapshotDeviceObject) }
    }
    /// The name of the volume that had been shadow copied.
    #[doc(alias = "m_pwszOriginalVolumeName")]
    pub fn original_volume_name(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0.m_pwszOriginalVolumeName) }
    }
    /// The name of the machine containing the original volume.
    #[doc(alias = "m_pwszOriginatingMachine")]
    pub fn originating_machine(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0.m_pwszOriginatingMachine) }
    }
    /// The name of the machine running the Volume Shadow Copy Service that
    /// created the shadow copy.
    #[doc(alias = "m_pwszServiceMachine")]
    pub fn service_machine(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0.m_pwszServiceMachine) }
    }
    /// The name of the shadow copy when it is exposed. This is a drive letter
    /// or mounted folder (if the shadow copy is exposed as a local volume), or
    /// a share name. Corresponds to the `wszExpose` parameter of the
    /// `IVssBackupComponents::ExposeSnapshot` method.
    #[doc(alias = "m_pwszExposedName")]
    pub fn exposed_name(&self) -> Option<&U16CStr> {
        if self.0.m_pwszExposedName.is_null() {
            None
        } else {
            Some(unsafe { U16CStr::from_ptr_str(self.0.m_pwszExposedName) })
        }
    }
    /// String indicating the portion of the shadow copy of a volume made available
    /// if it is exposed as a share. Corresponds to the `wszPathFromRoot` parameter
    /// of the `IVssBackupComponents::ExposeSnapshot` method.
    #[doc(alias = "m_pwszExposedPath")]
    pub fn exposed_path(&self) -> Option<&U16CStr> {
        if self.0.m_pwszExposedPath.is_null() {
            None
        } else {
            Some(unsafe { U16CStr::from_ptr_str(self.0.m_pwszExposedPath) })
        }
    }
    /// A VSS_ID (GUID) uniquely identifying the provider used to create this
    /// shadow copy.
    #[doc(alias = "m_ProviderId")]
    pub fn provider_id(&self) -> VSS_ID {
        self.0.m_ProviderId
    }
    /// The attributes of the shadow copy expressed as a bit mask (or bitwise OR)
    /// of members of the [`VolumeSnapshotAttributes`] enumeration.
    #[doc(alias = "m_lSnapshotAttributes")]
    pub fn snapshot_attributes(&self) -> RawBitFlags<VolumeSnapshotAttributes> {
        RawBitFlags::from_raw(self.0.m_lSnapshotAttributes as _)
    }
    /// Time stamp indicating when the shadow copy was created. The exact time is
    /// determined by the provider. See [VSS_TIMESTAMP] for information about the
    /// time-stamp format.
    ///
    /// [VSS_TIMESTAMP]: https://docs.microsoft.com/en-us/windows/desktop/VSS/volume-shadow-copy-api-data-types
    #[doc(alias = "m_tsCreationTimestamp")]
    pub fn creation_timestamp(&self) -> i64 {
        self.0.m_tsCreationTimestamp
    }
    /// Current shadow copy creation status.
    #[doc(alias = "m_eStatus")]
    pub fn status(&self) -> SnapshotState {
        self.0.m_eStatus.into()
    }
}
impl SnapshotProperties {
    /// Free all memory used by the content of the struct.
    ///
    /// # Safety
    ///
    /// Don't access any fields of the struct that contain strings or byte arrays
    /// after this function has been called.
    pub unsafe fn unchecked_free(&mut self) {
        // Safety: the docs for `VSS_OBJECT_PROP` specify that the
        // `VssFreeSnapshotProperties` function can be used to free the contents
        // of the `VSS_SNAPSHOT_PROP` variant instead of manually freeing each
        // string and byte array.
        // See remarks at: https://docs.microsoft.com/en-us/windows/win32/api/vss/ns-vss-vss_object_prop
        // It is also mentioned for the docs of `VSS_SNAPSHOT_PROP` in the remarks, see:
        // https://docs.microsoft.com/en-us/windows/win32/api/vss/ns-vss-vss_snapshot_prop
        // And in the remarks for the `IVssEnumObject::Next` method, see:
        // https://docs.microsoft.com/en-us/windows/win32/api/vss/nf-vss-ivssenumobject-next
        vsbackup::VssFreeSnapshotProperties(&mut self.0);
    }
}
impl Drop for SnapshotProperties {
    fn drop(&mut self) {
        // Safety: we will never access the contents of this struct after this point.
        unsafe {
            self.unchecked_free();
        }
    }
}

/// Specifies shadow copy provider properties.
#[doc(alias = "VSS_PROVIDER_PROP")]
#[repr(transparent)]
pub struct ProviderProperties(vss::VSS_PROVIDER_PROP);
/// Field getters.
impl ProviderProperties {
    /// Identifies the provider who supports shadow copies of this class.
    #[doc(alias = "m_ProviderId")]
    pub fn provider_id(&self) -> VSS_ID {
        self.0.m_ProviderId
    }
    /// The provider name.
    #[doc(alias = "m_pwszProviderName")]
    pub fn provider_name(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0.m_pwszProviderName) }
    }
    /// The provider type.
    #[doc(alias = "m_eProviderType")]
    pub fn provider_type(&self) -> ProviderType {
        self.0.m_eProviderType.into()
    }
    /// The provider version in readable format.
    #[doc(alias = "m_pwszProviderVersion")]
    pub fn provider_version(&self) -> &U16CStr {
        unsafe { U16CStr::from_ptr_str(self.0.m_pwszProviderVersion) }
    }
    /// A `VSS_ID` (GUID) uniquely identifying the version of a provider.
    #[doc(alias = "m_ProviderVersionId")]
    pub fn provider_version_id(&self) -> VSS_ID {
        self.0.m_ProviderVersionId
    }
    /// Class identifier of the component registered in the local machine's
    /// COM catalog.
    #[doc(alias = "m_ClassId")]
    pub fn class_id(&self) -> GUID {
        self.0.m_ClassId
    }
}
impl ProviderProperties {
    /// Free all memory used by the content of the struct.
    ///
    /// # Safety
    ///
    /// Don't access any fields of the struct that contain strings or byte arrays
    /// after this function has been called.
    pub unsafe fn unchecked_free(&mut self) {
        // Safety: the docs for `VSS_OBJECT_PROP` specify that this
        // is how we are supposed to free the contents of the variant:
        // https://docs.microsoft.com/en-us/windows/win32/api/vss/ns-vss-vss_object_prop
        // This is also mentioned in the remarks for the `IVssEnumObject::Next` method, see:
        // https://docs.microsoft.com/en-us/windows/win32/api/vss/nf-vss-ivssenumobject-next
        if !self.0.m_pwszProviderName.is_null() {
            CoTaskMemFree(self.0.m_pwszProviderName as *mut c_void);
        }
        if !self.0.m_pwszProviderVersion.is_null() {
            CoTaskMemFree(self.0.m_pwszProviderVersion as *mut c_void);
        }
    }
}
impl Drop for ProviderProperties {
    fn drop(&mut self) {
        // Safety: we will never access the contents of this struct after this point.
        unsafe {
            self.unchecked_free();
        }
    }
}

/// Defines the properties of a provider, volume, shadow copy, or shadow copy set.
#[doc(alias = "VSS_OBJECT_PROP")]
// Make it transparent so that a user can provide a buffer that we can fill with
// this type:
#[repr(transparent)]
// Default: zeroed so `ObjectType` will be `Unknown` since `vss::VSS_OBJECT_UNKNOWN`
// is the zero value:
#[derive(Default)]
pub struct ObjectProperties(vss::VSS_OBJECT_PROP);
impl ObjectProperties {
    pub fn object_type(&self) -> ObjectType {
        self.0.Type.into()
    }
    /// Copy the data of the current struct and change the current type to
    /// the [`ObjectType::Unknown`] variant so that the wrapped union of the
    /// original struct can't be accessed again.
    pub fn take(&mut self) -> Self {
        let cloned: Self = unsafe { ptr::read(&*self) };
        let cloned = ManuallyDrop::new(cloned);
        // Set the union's discriminator to `Unknown` so that the original struct
        // can't be used to access the union data anymore:
        self.0.Type = ObjectType::Unknown.into();
        ManuallyDrop::into_inner(cloned)
    }
    pub fn into_object(self) -> Option<ObjectUnion> {
        // Safety: this ensures it is safe to use `ptr::read` to copy/take the
        // contents of the union.
        let this = ManuallyDrop::new(self);
        // Safety: the union's variant is dependency on the object type is specified at:
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-scmp/f63af19f-bc5c-4a20-afaf-4f6e0f7c1045
        Some(match this.object_type() {
            ObjectType::Snapshot => ObjectUnion::Snapshot(unsafe {
                ptr::read(
                    &*((&this.0.Obj) as *const vss::VSS_OBJECT_UNION
                        as *const vss::VSS_SNAPSHOT_PROP
                        as *const SnapshotProperties),
                )
            }),
            ObjectType::Provider => ObjectUnion::Provider(unsafe {
                ptr::read(
                    &*((&this.0.Obj) as *const vss::VSS_OBJECT_UNION
                        as *const vss::VSS_PROVIDER_PROP
                        as *const ProviderProperties),
                )
            }),
            _ => {
                return None;
            }
        })
    }
    pub fn as_object(&self) -> Option<ObjectUnionRef<'_>> {
        // Safety: the union's variant is dependency on the object type is specified at:
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-scmp/f63af19f-bc5c-4a20-afaf-4f6e0f7c1045
        Some(match self.object_type() {
            ObjectType::Snapshot => ObjectUnionRef::Snapshot(unsafe {
                &*((&self.0.Obj) as *const vss::VSS_OBJECT_UNION as *const vss::VSS_SNAPSHOT_PROP
                    as *const SnapshotProperties)
            }),
            ObjectType::Provider => ObjectUnionRef::Provider(unsafe {
                &*((&self.0.Obj) as *const vss::VSS_OBJECT_UNION as *const vss::VSS_PROVIDER_PROP
                    as *const ProviderProperties)
            }),
            _ => {
                return None;
            }
        })
    }
    pub fn as_object_mut(&mut self) -> Option<ObjectUnionMut<'_>> {
        // Safety: the union's variant is dependency on the object type is specified at:
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-scmp/f63af19f-bc5c-4a20-afaf-4f6e0f7c1045
        Some(match self.object_type() {
            ObjectType::Snapshot => ObjectUnionMut::Snapshot(unsafe {
                &mut *((&mut self.0.Obj) as *mut vss::VSS_OBJECT_UNION
                    as *mut vss::VSS_SNAPSHOT_PROP
                    as *mut SnapshotProperties)
            }),
            ObjectType::Provider => ObjectUnionMut::Provider(unsafe {
                &mut *((&mut self.0.Obj) as *mut vss::VSS_OBJECT_UNION
                    as *mut vss::VSS_PROVIDER_PROP
                    as *mut ProviderProperties)
            }),
            _ => {
                return None;
            }
        })
    }
    /// Free all memory used by the current variant and change the current type to
    /// the [`ObjectType::Unknown`] variant.
    pub fn free(&mut self) {
        struct DropGuard<T, F: FnMut(&mut T)>(T, F);
        impl<T, F: FnMut(&mut T)> Drop for DropGuard<T, F> {
            fn drop(&mut self) {
                (self.1)(&mut self.0)
            }
        }
        let guard = DropGuard(self, |this| {
            // Set the union's discriminator to `Unknown` so that we never free memory
            // twice:
            this.0.Type = ObjectType::Unknown.into();
        });
        // Safety: the union's data won't be accessible after the guard drops.
        unsafe {
            guard.0.unchecked_free();
        }
        drop(guard);
    }
    /// Free all memory used by the content of the wrapped union.
    ///
    /// # Safety
    ///
    /// Don't access any fields of the wrapped union that contain strings or byte
    /// arrays after this function has been called.
    pub unsafe fn unchecked_free(&mut self) {
        match self.as_object_mut() {
            Some(ObjectUnionMut::Provider(v)) => v.unchecked_free(),
            Some(ObjectUnionMut::Snapshot(v)) => v.unchecked_free(),
            None => {}
        }
    }
}
impl Drop for ObjectProperties {
    fn drop(&mut self) {
        // Safety: we will never access the contents of this struct after this point.
        unsafe {
            self.unchecked_free();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Enumerations
////////////////////////////////////////////////////////////////////////////////

raw_bitflags! {
    /// Defines shadow copy LUN flags.
    #[doc(alias = "VSS_HARDWARE_OPTIONS")]
    #[derive(Default)]
    pub struct HardwareOptions: vss::VSS_HARDWARE_OPTIONS {
        #[doc(alias = "VSS_BREAKEX_FLAG_MASK_LUNS")]
        const BREAKEX_FLAG_MASK_LUNS = vss::VSS_BREAKEX_FLAG_MASK_LUNS;
        #[doc(alias = "VSS_BREAKEX_FLAG_MAKE_READ_WRITE")]
        const BREAKEX_FLAG_MAKE_READ_WRITE = vss::VSS_BREAKEX_FLAG_MAKE_READ_WRITE;
        #[doc(alias = "VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL")]
        const BREAKEX_FLAG_REVERT_IDENTITY_ALL = vss::VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL;
        #[doc(alias = "VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE")]
        const BREAKEX_FLAG_REVERT_IDENTITY_NONE = vss::VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE;
        #[doc(alias = "VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE")]
        const ONLUNSTATECHANGE_NOTIFY_READ_WRITE = vss::VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE;
        #[doc(alias = "VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY")]
        const ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY = vss::VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY;
        #[doc(alias = "VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY")]
        const ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY = vss::VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY;
        #[doc(alias = "VSS_ONLUNSTATECHANGE_DO_MASK_LUNS")]
        const ONLUNSTATECHANGE_DO_MASK_LUNS = vss::VSS_ONLUNSTATECHANGE_DO_MASK_LUNS;
    }
}

raw_bitflags! {
    /// Indicates which volume control or file I/O operations are disabled for
    /// the volume that has been shadow copied.
    ///
    /// Used with the [`VolumeSnapshottedInfo`] struct.
    ///
    /// [`VolumeSnapshottedInfo`]: crate::vsbackup::VolumeSnapshottedInfo
    #[doc(alias = "VSS_SNAPSHOT_COMPATIBILITY")]
    #[derive(Default)]
    pub struct SnapshotCapability: LONG {
        /// The provider managing the shadow copies for a specified volume does not
        /// support defragmentation operations on that volume.
        #[doc(alias = "VSS_SC_DISABLE_DEFRAG")]
        const DISABLE_DEFRAG = vss::VSS_SC_DISABLE_DEFRAG as LONG;
        /// The provider managing the shadow copies for a specified volume does not
        /// support content index operations on that volume.
        #[doc(alias = "VSS_SC_DISABLE_CONTENTINDEX")]
        const DISABLE_CONTENTINDEX = vss::VSS_SC_DISABLE_CONTENTINDEX as LONG;
    }
}

raw_bitflags! {
    /// Used by a requester to specify how a resynchronization operation is to
    /// be performed.
    #[doc(alias = "VSS_RECOVERY_OPTIONS")]
    #[derive(Default)]
    pub struct RecoveryOptions: vss::VSS_RECOVERY_OPTIONS {
        #[doc(alias = "VSS_RECOVERY_REVERT_IDENTITY_ALL")]
        const REVERT_IDENTITY_ALL = vss::VSS_RECOVERY_REVERT_IDENTITY_ALL;
        #[doc(alias = "VSS_RECOVERY_NO_VOLUME_CHECK")]
        const NO_VOLUME_CHECK = vss::VSS_RECOVERY_NO_VOLUME_CHECK;
    }
}

raw_bitflags! {
    /// used by writers to indicate their support of certain backup
    /// operations—such as incremental or differential backup—on the basis of
    /// file sets (a specified file or files).
    ///
    /// File sets stored in the Writer Metadata Document are tagged with a bit
    /// mask (or bitwise OR) of values indicating the following:
    ///
    /// - Whether the writer and the requester have to evaluate a given file set
    ///   for participation in the specified type of backup operations
    /// - Whether backing up the specified file will require a shadow copy
    #[doc(alias = "VSS_FILE_SPEC_BACKUP_TYPE")]
    #[derive(Default)]
    pub struct FileSpecificationBackupType : vss::VSS_FILE_SPEC_BACKUP_TYPE {
        #[doc(alias = "VSS_FSBT_FULL_BACKUP_REQUIRED")]
        const FULL_BACKUP_REQUIRED = vss::VSS_FSBT_FULL_BACKUP_REQUIRED;
        #[doc(alias = "VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED")]
        const DIFFERENTIAL_BACKUP_REQUIRED = vss::VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED;
        #[doc(alias = "VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED")]
        const INCREMENTAL_BACKUP_REQUIRED = vss::VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED;
        #[doc(alias = "VSS_FSBT_LOG_BACKUP_REQUIRED")]
        const LOG_BACKUP_REQUIRED = vss::VSS_FSBT_LOG_BACKUP_REQUIRED;
        #[doc(alias = "VSS_FSBT_FULL_SNAPSHOT_REQUIRED")]
        const FULL_SNAPSHOT_REQUIRED = vss::VSS_FSBT_FULL_SNAPSHOT_REQUIRED;
        #[doc(alias = "VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED")]
        const DIFFERENTIAL_SNAPSHOT_REQUIRED = vss::VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED;
        #[doc(alias = "VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED")]
        const INCREMENTAL_SNAPSHOT_REQUIRED = vss::VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED;
        #[doc(alias = "VSS_FSBT_LOG_SNAPSHOT_REQUIRED")]
        const LOG_SNAPSHOT_REQUIRED = vss::VSS_FSBT_LOG_SNAPSHOT_REQUIRED;
        #[doc(alias = "VSS_FSBT_CREATED_DURING_BACKUP")]
        const CREATED_DURING_BACKUP = vss::VSS_FSBT_CREATED_DURING_BACKUP;
        #[doc(alias = "VSS_FSBT_ALL_BACKUP_REQUIRED")]
        const ALL_BACKUP_REQUIRED = vss::VSS_FSBT_ALL_BACKUP_REQUIRED;
        #[doc(alias = "VSS_FSBT_ALL_SNAPSHOT_REQUIRED")]
        const ALL_SNAPSHOT_REQUIRED = vss::VSS_FSBT_ALL_SNAPSHOT_REQUIRED;
    }
}

raw_bitflags! {
    /// Modifications to a [`SnapshotContext`].
    #[doc(alias = "VSS_VOLUME_SNAPSHOT_ATTRIBUTES")]
    #[derive(Default)]
    pub struct VolumeSnapshotAttributes: vss::VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
        #[doc(alias = "VSS_VOLSNAP_ATTR_PERSISTENT")]
        const PERSISTENT = vss::VSS_VOLSNAP_ATTR_PERSISTENT;
        #[doc(alias = "VSS_VOLSNAP_ATTR_NO_AUTORECOVERY")]
        const NO_AUTORECOVERY = vss::VSS_VOLSNAP_ATTR_NO_AUTORECOVERY;
        #[doc(alias = "VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE")]
        const CLIENT_ACCESSIBLE = vss::VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE;
        #[doc(alias = "VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE")]
        const NO_AUTO_RELEASE = vss::VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE;
        #[doc(alias = "VSS_VOLSNAP_ATTR_NO_WRITERS")]
        const NO_WRITERS = vss::VSS_VOLSNAP_ATTR_NO_WRITERS;
        #[doc(alias = "VSS_VOLSNAP_ATTR_TRANSPORTABLE")]
        const TRANSPORTABLE = vss::VSS_VOLSNAP_ATTR_TRANSPORTABLE;
        #[doc(alias = "VSS_VOLSNAP_ATTR_NOT_SURFACED")]
        const NOT_SURFACED = vss::VSS_VOLSNAP_ATTR_NOT_SURFACED;
        #[doc(alias = "VSS_VOLSNAP_ATTR_NOT_TRANSACTED")]
        const NOT_TRANSACTED = vss::VSS_VOLSNAP_ATTR_NOT_TRANSACTED;
        #[doc(alias = "VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED")]
        const HARDWARE_ASSISTED = vss::VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED;
        #[doc(alias = "VSS_VOLSNAP_ATTR_DIFFERENTIAL")]
        const DIFFERENTIAL = vss::VSS_VOLSNAP_ATTR_DIFFERENTIAL;
        #[doc(alias = "VSS_VOLSNAP_ATTR_PLEX")]
        const PLEX = vss::VSS_VOLSNAP_ATTR_PLEX;
        #[doc(alias = "VSS_VOLSNAP_ATTR_IMPORTED")]
        const IMPORTED = vss::VSS_VOLSNAP_ATTR_IMPORTED;
        #[doc(alias = "VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY")]
        const EXPOSED_LOCALLY = vss::VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY;
        #[doc(alias = "VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY")]
        const EXPOSED_REMOTELY = vss::VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY;
        #[doc(alias = "VSS_VOLSNAP_ATTR_AUTORECOVER")]
        const AUTORECOVER = vss::VSS_VOLSNAP_ATTR_AUTORECOVER;
        #[doc(alias = "VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY")]
        const ROLLBACK_RECOVERY = vss::VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY;
        #[doc(alias = "VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT")]
        const DELAYED_POSTSNAPSHOT = vss::VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT;
        #[doc(alias = "VSS_VOLSNAP_ATTR_TXF_RECOVERY")]
        const TXF_RECOVERY = vss::VSS_VOLSNAP_ATTR_TXF_RECOVERY;
        #[doc(alias = "VSS_VOLSNAP_ATTR_FILE_SHARE")]
        const FILE_SHARE = vss::VSS_VOLSNAP_ATTR_FILE_SHARE;
    }
}

raw_bitflags! {
    /// Used by a writer to indicate the types of backup operations it can
    /// participate in. The supported kinds of backup are expressed as a bit
    /// mask (or bitwise OR) of this type's values.
    #[doc(alias = "VSS_BACKUP_SCHEMA")]
    #[derive(Default)]
    pub struct BackupSchema: vss::VSS_BACKUP_SCHEMA {
        #[doc(alias = "VSS_BS_UNDEFINED")]
        const UNDEFINED = vss::VSS_BS_UNDEFINED;
        #[doc(alias = "VSS_BS_DIFFERENTIAL")]
        const DIFFERENTIAL = vss::VSS_BS_DIFFERENTIAL;
        #[doc(alias = "VSS_BS_INCREMENTAL")]
        const INCREMENTAL = vss::VSS_BS_INCREMENTAL;
        #[doc(alias = "VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL")]
        const EXCLUSIVE_INCREMENTAL_DIFFERENTIAL = vss::VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL;
        #[doc(alias = "VSS_BS_LOG")]
        const LOG = vss::VSS_BS_LOG;
        #[doc(alias = "VSS_BS_COPY")]
        const COPY = vss::VSS_BS_COPY;
        #[doc(alias = "VSS_BS_TIMESTAMPED")]
        const TIMESTAMPED = vss::VSS_BS_TIMESTAMPED;
        #[doc(alias = "VSS_BS_LAST_MODIFY")]
        const LAST_MODIFY = vss::VSS_BS_LAST_MODIFY;
        #[doc(alias = "VSS_BS_LSN")]
        const LSN = vss::VSS_BS_LSN;
        #[doc(alias = "VSS_BS_WRITER_SUPPORTS_NEW_TARGET")]
        const WRITER_SUPPORTS_NEW_TARGET = vss::VSS_BS_WRITER_SUPPORTS_NEW_TARGET;
        #[doc(alias = "VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE")]
        const WRITER_SUPPORTS_RESTORE_WITH_MOVE = vss::VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE;
        #[doc(alias = "VSS_BS_INDEPENDENT_SYSTEM_STATE")]
        const INDEPENDENT_SYSTEM_STATE = vss::VSS_BS_INDEPENDENT_SYSTEM_STATE;
        #[doc(alias = "VSS_BS_ROLLFORWARD_RESTORE")]
        const ROLLFORWARD_RESTORE = vss::VSS_BS_ROLLFORWARD_RESTORE;
        #[doc(alias = "VSS_BS_RESTORE_RENAME")]
        const RESTORE_RENAME = vss::VSS_BS_RESTORE_RENAME;
        #[doc(alias = "VSS_BS_AUTHORITATIVE_RESTORE")]
        const AUTHORITATIVE_RESTORE = vss::VSS_BS_AUTHORITATIVE_RESTORE;
        #[doc(alias = "VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES")]
        const WRITER_SUPPORTS_PARALLEL_RESTORES = vss::VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES;
    }
}

with_from!(
    [raw = vss::VSS_SNAPSHOT_CONTEXT, default = Backup],
    /// Specifies how a shadow copy is to be created, queried, or deleted and
    /// the degree of writer involvement.
    #[doc(alias = "VSS_SNAPSHOT_CONTEXT")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SnapshotContext {
        #[doc(alias = "VSS_CTX_BACKUP")]
        Backup = vss::VSS_CTX_BACKUP,
        #[doc(alias = "VSS_CTX_FILE_SHARE_BACKUP")]
        FileShareBackup = vss::VSS_CTX_FILE_SHARE_BACKUP,
        #[doc(alias = "VSS_CTX_NAS_ROLLBACK")]
        NasRollback = vss::VSS_CTX_NAS_ROLLBACK,
        #[doc(alias = "VSS_CTX_APP_ROLLBACK")]
        AppRollback = vss::VSS_CTX_APP_ROLLBACK,
        #[doc(alias = "VSS_CTX_CLIENT_ACCESSIBLE")]
        ClientAccessible = vss::VSS_CTX_CLIENT_ACCESSIBLE,
        #[doc(alias = "VSS_CTX_CLIENT_ACCESSIBLE_WRITERS")]
        ClientAccessibleWriters = vss::VSS_CTX_CLIENT_ACCESSIBLE_WRITERS,
        #[doc(alias = "VSS_CTX_ALL")]
        All = vss::VSS_CTX_ALL,
    }
);

with_from!(
    [raw = vss::VSS_BACKUP_TYPE, fallback = Undefined],
    /// Indicates the type of backup to be performed using VSS writer/requester
    /// coordination.
    #[doc(alias = "VSS_BACKUP_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum BackupType {
        #[doc(alias = "VSS_BT_UNDEFINED")]
        Undefined = vss::VSS_BT_UNDEFINED,
        #[doc(alias = "VSS_BT_FULL")]
        Full = vss::VSS_BT_FULL,
        #[doc(alias = "VSS_BT_INCREMENTAL")]
        Incremental = vss::VSS_BT_INCREMENTAL,
        #[doc(alias = "VSS_BT_DIFFERENTIAL")]
        Differential = vss::VSS_BT_DIFFERENTIAL,
        #[doc(alias = "VSS_BT_LOG")]
        Log = vss::VSS_BT_LOG,
        #[doc(alias = "VSS_BT_COPY")]
        Copy = vss::VSS_BT_COPY,
        #[doc(alias = "VSS_BT_OTHER")]
        Other = vss::VSS_BT_OTHER,
    }
);

with_from!(
    [raw = vss::VSS_OBJECT_TYPE, fallback = Unknown],
    /// Used by requesters to identify an object as a shadow copy set, shadow copy,
    /// or provider.
    #[doc(alias = "VSS_OBJECT_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ObjectType {
        #[doc(alias = "VSS_OBJECT_UNKNOWN")]
        Unknown = vss::VSS_OBJECT_UNKNOWN,
        #[doc(alias = "VSS_OBJECT_NONE")]
        None = vss::VSS_OBJECT_NONE,
        #[doc(alias = "VSS_OBJECT_SNAPSHOT_SET")]
        SnapshotSet = vss::VSS_OBJECT_SNAPSHOT_SET,
        #[doc(alias = "VSS_OBJECT_SNAPSHOT")]
        Snapshot = vss::VSS_OBJECT_SNAPSHOT,
        #[doc(alias = "VSS_OBJECT_PROVIDER")]
        Provider = vss::VSS_OBJECT_PROVIDER,
        #[doc(alias = "VSS_OBJECT_TYPE_COUNT")]
        TypeCount = vss::VSS_OBJECT_TYPE_COUNT,
    }
);

with_from!(
    [raw = vss::VSS_WRITER_STATE, fallback = Unknown],
    /// Indicates the current state of the writer.
    ///
    /// # Remarks
    ///
    /// A requester determines the state of a writer through
    /// `IVssBackupComponents::GetWriterStatus`.
    #[doc(alias = "VSS_WRITER_STATE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum WriterState {
        #[doc(alias = "VSS_WS_UNKNOWN")]
        Unknown = vss::VSS_WS_UNKNOWN,
        #[doc(alias = "VSS_WS_STABLE")]
        Stable = vss::VSS_WS_STABLE,
        #[doc(alias = "VSS_WS_WAITING_FOR_FREEZE")]
        WaitingForFreeze = vss::VSS_WS_WAITING_FOR_FREEZE,
        #[doc(alias = "VSS_WS_WAITING_FOR_THAW")]
        WaitingForThaw = vss::VSS_WS_WAITING_FOR_THAW,
        #[doc(alias = "VSS_WS_WAITING_FOR_POST_SNAPSHOT")]
        WaitingForPostSnapshot = vss::VSS_WS_WAITING_FOR_POST_SNAPSHOT,
        #[doc(alias = "VSS_WS_WAITING_FOR_BACKUP_COMPLETE")]
        WaitingForBackupComplete = vss::VSS_WS_WAITING_FOR_BACKUP_COMPLETE,
        #[doc(alias = "VSS_WS_FAILED_AT_IDENTIFY")]
        FailedAtIdentify = vss::VSS_WS_FAILED_AT_IDENTIFY,
        #[doc(alias = "VSS_WS_FAILED_AT_PREPARE_BACKUP")]
        FailedAtPrepareBackup = vss::VSS_WS_FAILED_AT_PREPARE_BACKUP,
        #[doc(alias = "VSS_WS_FAILED_AT_PREPARE_SNAPSHOT")]
        FailedAtPrepareSnapshot = vss::VSS_WS_FAILED_AT_PREPARE_SNAPSHOT,
        #[doc(alias = "VSS_WS_FAILED_AT_FREEZE")]
        FailedAtFreeze = vss::VSS_WS_FAILED_AT_FREEZE,
        #[doc(alias = "VSS_WS_FAILED_AT_THAW")]
        FailedAtThaw = vss::VSS_WS_FAILED_AT_THAW,
        #[doc(alias = "VSS_WS_FAILED_AT_POST_SNAPSHOT")]
        FailedAtPostSnapshot = vss::VSS_WS_FAILED_AT_POST_SNAPSHOT,
        #[doc(alias = "VSS_WS_FAILED_AT_BACKUP_COMPLETE")]
        FailedAtBackupComplete = vss::VSS_WS_FAILED_AT_BACKUP_COMPLETE,
        #[doc(alias = "VSS_WS_FAILED_AT_PRE_RESTORE")]
        FailedAtPreRestore = vss::VSS_WS_FAILED_AT_PRE_RESTORE,
        #[doc(alias = "VSS_WS_FAILED_AT_POST_RESTORE")]
        FailedAtPostRestore = vss::VSS_WS_FAILED_AT_POST_RESTORE,
        #[doc(alias = "VSS_WS_FAILED_AT_BACKUPSHUTDOWN")]
        FailedAtBackupShutdown = vss::VSS_WS_FAILED_AT_BACKUPSHUTDOWN,
        #[doc(alias = "VSS_WS_COUNT")]
        COUNT = vss::VSS_WS_COUNT,
    }
);

with_from!(
    [raw = vss::VSS_ROLLFORWARD_TYPE, fallback = Undefined],
    /// Used by a requester to indicate the type of roll-forward operation it is
    /// about to perform.
    #[doc(alias = "VSS_ROLLFORWARD_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum RollForwardType {
        #[doc(alias = "VSS_RF_UNDEFINED")]
        Undefined = vss::VSS_RF_UNDEFINED,
        #[doc(alias = "VSS_RF_NONE")]
        None = vss::VSS_RF_NONE,
        #[doc(alias = "VSS_RF_ALL")]
        All = vss::VSS_RF_ALL,
        #[doc(alias = "VSS_RF_PARTIAL")]
        Partial = vss::VSS_RF_PARTIAL,
    }
);

with_from!(
    [raw = vss::VSS_PROVIDER_TYPE, fallback = Unknown],
    #[doc(alias = "VSS_PROVIDER_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ProviderType {
        /// The provider type is unknown.
        ///
        /// This indicates an error in the application or the VSS service, or
        /// that no provider is available.
        #[doc(alias = "VSS_PROV_UNKNOWN")]
        Unknown = vss::VSS_PROV_UNKNOWN,
        /// The default provider that ships with Windows.
        #[doc(alias = "VSS_PROV_SYSTEM")]
        System = vss::VSS_PROV_SYSTEM,
        /// A software provider.
        #[doc(alias = "VSS_PROV_SOFTWARE")]
        Software = vss::VSS_PROV_SOFTWARE,
        /// A hardware provider.
        #[doc(alias = "VSS_PROV_HARDWARE")]
        Hardware = vss::VSS_PROV_HARDWARE,
        /// A file share provider.
        ///
        /// `Windows 7, Windows Server 2008 R2, Windows Vista, Windows Server 2008,
        /// Windows XP and Windows Server 2003`:  This enumeration value is not
        /// supported until Windows 8 and Windows Server 2012.
        #[doc(alias = "VSS_PROV_FILESHARE")]
        FileShare = vss::VSS_PROV_FILESHARE,
    }
);

with_from!(
    [raw = vss::VSS_SNAPSHOT_STATE, fallback = Unknown],
    /// Returned by a provider to specify the state of a given shadow copy
    /// operation.
    #[doc(alias = "VSS_SNAPSHOT_STATE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SnapshotState {
        #[doc(alias = "VSS_SS_UNKNOWN")]
        Unknown = vss::VSS_SS_UNKNOWN,
        #[doc(alias = "VSS_SS_PREPARING")]
        Preparing = vss::VSS_SS_PREPARING,
        #[doc(alias = "VSS_SS_PROCESSING_PREPARE")]
        ProcessingPrepare = vss::VSS_SS_PROCESSING_PREPARE,
        #[doc(alias = "VSS_SS_PREPARED")]
        Prepared = vss::VSS_SS_PREPARED,
        #[doc(alias = "VSS_SS_PROCESSING_PRECOMMIT")]
        ProcessingPreCommit = vss::VSS_SS_PROCESSING_PRECOMMIT,
        #[doc(alias = "VSS_SS_PRECOMMITTED")]
        PreCommitted = vss::VSS_SS_PRECOMMITTED,
        #[doc(alias = "VSS_SS_PROCESSING_COMMIT")]
        ProcessingCommit = vss::VSS_SS_PROCESSING_COMMIT,
        #[doc(alias = "VSS_SS_COMMITTED")]
        Committed = vss::VSS_SS_COMMITTED,
        #[doc(alias = "VSS_SS_PROCESSING_POSTCOMMIT")]
        ProcessingPostCommit = vss::VSS_SS_PROCESSING_POSTCOMMIT,
        #[doc(alias = "VSS_SS_PROCESSING_PREFINALCOMMIT")]
        ProcessingPreFinalCommit = vss::VSS_SS_PROCESSING_PREFINALCOMMIT,
        #[doc(alias = "VSS_SS_PREFINALCOMMITTED")]
        PreFinalCommitted = vss::VSS_SS_PREFINALCOMMITTED,
        #[doc(alias = "VSS_SS_PROCESSING_POSTFINALCOMMIT")]
        ProcessingPostFinalCommit = vss::VSS_SS_PROCESSING_POSTFINALCOMMIT,
        #[doc(alias = "VSS_SS_CREATED")]
        Created = vss::VSS_SS_CREATED,
        #[doc(alias = "VSS_SS_ABORTED")]
        Aborted = vss::VSS_SS_ABORTED,
        #[doc(alias = "VSS_SS_DELETED")]
        Deleted = vss::VSS_SS_DELETED,
        #[doc(alias = "VSS_SS_POSTCOMMITTED")]
        PostCommitted = vss::VSS_SS_POSTCOMMITTED,
        #[doc(alias = "VSS_SS_COUNT")]
        Count = vss::VSS_SS_COUNT,
    }
);

with_from!(
    [
        raw = vss::VSS_RESTORE_TYPE,
        fallback = Undefined,
        default = Undefined
    ],
    /// Used by a requester to indicate the type of restore operation it is
    /// about to perform.
    #[doc(alias = "VSS_RESTORE_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum RestoreType {
        #[doc(alias = "VSS_RTYPE_UNDEFINED")]
        Undefined = vss::VSS_RTYPE_UNDEFINED,
        #[doc(alias = "VSS_RTYPE_BY_COPY")]
        ByCopy = vss::VSS_RTYPE_BY_COPY,
        #[doc(alias = "VSS_RTYPE_IMPORT")]
        Import = vss::VSS_RTYPE_IMPORT,
        #[doc(alias = "VSS_RTYPE_OTHER")]
        Other = vss::VSS_RTYPE_OTHER,
    }
);

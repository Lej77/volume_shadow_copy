//! Bindings for the `VSwriter.h` header.
//!
//! This is required when developing a VSS "writer" or a VSS "requester".
//!
//! # References
//!
//! [Vswriter.h header - Win32 apps | Microsoft Docs](https://docs.microsoft.com/en-us/windows/win32/api/vswriter/)

use std::ptr::{null, null_mut};

use widestring::U16CStr;
use winapi::{
    shared::{
        minwindef::{DWORD, UINT},
        winerror::{HRESULT, S_OK},
        wtypes::BSTR,
    },
    um::{
        vss::{self, VSS_ID},
        vswriter,
    },
};
use winstr::BString;

use super::{
    check_com, errors::*, impl_query_interface, raw_bitflags, take_ownership_of_bstr,
    transparent_wrapper, unsafe_deref_to_ref, unsafe_impl_as_IUnknown,
    vss::FileSpecificationBackupType, with_from, RawBitFlags, SafeCOMComponent,
};

////////////////////////////////////////////////////////////////////////////////
// IVssWMDependency
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vswriter::IVssWMDependency);

/// Used by applications when backing up or restoring a component that has an
/// explicit writer-component dependency on a component managed by another writer.
/// (Dependencies must be between writers, not within writers.)
#[doc(alias = "IVssWMDependency")]
#[derive(Debug, Clone)]
pub struct WMDependency(pub(crate) SafeCOMComponent<vswriter::IVssWMDependency>);
impl_query_interface!(WMDependency => vswriter::IVssWMDependency);
transparent_wrapper!(
    #[doc(alias = "IVssWMDependency")]
    pub struct IWMDependency(vswriter::IVssWMDependency);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(WMDependency => IWMDependency);

impl IWMDependency {
    /// Retrieves the name of a component that the current component depends on
    /// in an explicit writer-component dependency.
    #[doc(alias = "GetComponentName")]
    pub fn get_component_name(&self) -> Result<BString, GetComponentNameError> {
        let mut component_name: BSTR = null_mut();
        let hr = unsafe { self.0.GetComponentName(&mut component_name) };
        let component_name = unsafe { take_ownership_of_bstr(component_name) };
        check_com(hr)?;
        Ok(component_name
            .unwrap()
            .expect("The component name returned from GetComponentName shouldn't be null"))
    }
    /// Retrieves the logical path of a component that the current component
    /// depends on in explicit writer-component dependency.
    #[doc(alias = "GetLogicalPath")]
    pub fn get_logical_path(&self) -> Result<BString, GetLogicalPathError> {
        let mut logical_path: BSTR = null_mut();
        let hr = unsafe { self.0.GetLogicalPath(&mut logical_path) };
        let component_name = unsafe { take_ownership_of_bstr(logical_path) };
        check_com(hr)?;
        Ok(component_name
            .unwrap()
            .expect("The logical path returned from GetLogicalPath shouldn't be null"))
    }
    /// Retrieves the class ID of a writer containing a component that the current
    /// component depends on in an explicit writer-component dependency.
    #[doc(alias = "GetWriterId")]
    pub fn get_writer_id(&self) -> Result<VSS_ID, GetWriterIdError> {
        let mut writer_id: VSS_ID = Default::default();
        check_com(unsafe { self.0.GetWriterId(&mut writer_id) })?;
        Ok(writer_id)
    }
}

////////////////////////////////////////////////////////////////////////////////
// IVssWMFiledesc
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vswriter::IVssWMFiledesc);

/// Returned to a calling application by a number of query methods. It provides
/// detailed information about a file or set of files (a file set).
#[doc(alias = "IVssWMFiledesc")]
#[derive(Debug, Clone)]
pub struct WMFileDescriptor(pub(crate) SafeCOMComponent<vswriter::IVssWMFiledesc>);
impl_query_interface!(WMFileDescriptor => vswriter::IVssWMFiledesc);
transparent_wrapper!(
    #[doc(alias = "IVssWMFiledesc")]
    pub struct IWMFileDescriptor(vswriter::IVssWMFiledesc);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(WMFileDescriptor => IWMFileDescriptor);

impl IWMFileDescriptor {
    /// Obtains an alternate location for a file set.
    #[doc(alias = "GetAlternateLocation")]
    pub fn get_alternate_location(&self) -> Result<Option<BString>, GetAlternateLocationError> {
        let mut alternate_location: BSTR = null_mut();
        let result = check_com(unsafe { self.0.GetAlternateLocation(&mut alternate_location) });
        let alternate_location = unsafe { take_ownership_of_bstr(alternate_location) };
        result?;
        Ok(alternate_location.unwrap())
    }
    /// Returns the file backup specification for the files specified by the current
    /// file descriptor as a bit mask (or bitwise OR) of values. This information
    /// indicates if the files are to be evaluated by their writer for participation
    /// in various specific types of backup operations (or if they will participate
    /// in an incremental or differential backups).
    #[doc(alias = "GetBackupTypeMask")]
    pub fn get_backup_type_mask(
        &self,
    ) -> Result<RawBitFlags<FileSpecificationBackupType>, GetBackupTypeMaskError> {
        let mut type_mask: vss::VSS_FILE_SPEC_BACKUP_TYPE = 0;
        check_com(unsafe { self.0.GetBackupTypeMask(&mut type_mask) })?;
        Ok(RawBitFlags::from_raw(type_mask))
    }
    /// Returns the file specification used to obtain the list of files that the
    /// current `IWMFileDescriptor` object is a member of.
    ///
    /// A querying method used a path and this file specification to return the
    /// current `IWMFileDescriptor` object.
    ///
    /// A file specification cannot contain directory specifications (for example,
    /// no backslashes) but can contain the ? and * wildcard characters.
    #[doc(alias = "GetFilespec")]
    pub fn get_file_specification(&self) -> Result<BString, GetFilespecError> {
        let mut file_spec: BSTR = null_mut();
        let result = check_com(unsafe { self.0.GetFilespec(&mut file_spec) });
        let file_spec = unsafe { take_ownership_of_bstr(file_spec) };
        result?;
        Ok(file_spec
            .unwrap()
            .expect("The file specification returned by GetFilespec should be null"))
    }
    /// Obtains the fully qualified directory path or the UNC path of the remote
    /// file share to obtain the list of files described in the current
    /// `IWMFileDescriptor` object.
    ///
    /// A querying method used this path and a file specification to return the
    /// current `IWMFileDescriptor` object.
    ///
    /// The path can be a long or short file name and can use the prefix "\?".
    /// For more information, see [Naming a File].
    ///
    /// Users of this method need to check to determine whether this path ends
    /// with a backslash ("\").
    ///
    /// [Naming a File]: https://docs.microsoft.com/en-us/windows/desktop/FileIO/naming-a-file
    #[doc(alias = "GetPath")]
    pub fn get_path(&self) -> Result<BString, GetPathError> {
        let mut path: BSTR = null_mut();
        let result = check_com(unsafe { self.0.GetPath(&mut path) });
        let path = unsafe { take_ownership_of_bstr(path) };
        result?;
        Ok(path
            .unwrap()
            .expect("The path returned by GetPath shouldn't be null"))
    }
    /// Indicates whether the list of files described in a `IWMFileDescriptor`
    /// object with a root directory returned by [`IWMFileDescriptor::get_path`]
    /// contains only files in that directory or whether the file list contains
    /// files from the directory hierarchy as well.
    ///
    /// The returned value is `true` if the path is treated as a hierarchy of
    /// directories to be traversed recursively, or `false` if not.
    ///
    /// For information on traversing over mounted folders, see
    /// [Working with Mounted Folders and Reparse Points](https://docs.microsoft.com/en-us/windows/desktop/VSS/working-with-reparse-and-mount-points).
    #[doc(alias = "GetRecursive")]
    pub fn get_recursive(&self) -> Result<bool, GetRecursiveError> {
        let mut recursive = false;
        check_com(unsafe { self.0.GetRecursive(&mut recursive) })?;
        Ok(recursive)
    }
}

////////////////////////////////////////////////////////////////////////////////
// IVssWriterComponents
////////////////////////////////////////////////////////////////////////////////

transparent_wrapper!(
    #[doc(alias = "IVssWriterComponents")]
    pub struct IWriterComponents(vswriter::IVssWriterComponents);
);

impl IWriterComponents {
    /// Returns an [`Component`] interface to one of a given writer's components
    /// explicitly stored in the Backup Components Document.
    #[doc(alias = "GetComponent")]
    pub fn get_component(
        &self,
        component_index: u32,
    ) -> Result<Component, WriterComponentsGetComponentError> {
        let mut component: *mut vswriter::IVssComponent = null_mut();
        check_com(unsafe { self.0.GetComponent(component_index, &mut component) })?;
        Ok(Component(unsafe { SafeCOMComponent::new(component) }))
    }
    /// Returns the number of a given writer's components explicitly stored in
    /// the Backup Components Document.
    #[doc(alias = "GetComponentCount")]
    pub fn get_component_count(&self) -> Result<u32, GetComponentCountError> {
        let mut components: UINT = 0;
        check_com(unsafe { self.0.GetComponentCount(&mut components) })?;
        Ok(components)
    }
    /// Gets the instance and class identifier of the writer responsible for the
    /// components.
    #[doc(alias = "GetWriterInfo")]
    pub fn get_writer_info(&self) -> Result<WriterInfo, GetWriterInfoError> {
        let mut info = WriterInfo {
            instance_id: Default::default(),
            writer_id: Default::default(),
        };
        check_com(unsafe {
            self.0
                .GetWriterInfo(&mut info.instance_id, &mut info.writer_id)
        })?;
        Ok(info)
    }
}

/// Info returned by the [`IWriterComponents::get_writer_info`] method.
pub struct WriterInfo {
    /// Identifier of the writer instance.
    pub instance_id: VSS_ID,
    /// Identifier of the writer class.
    pub writer_id: VSS_ID,
}

////////////////////////////////////////////////////////////////////////////////
// IVssComponent
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vswriter::IVssComponent);

#[doc(alias = "IVssComponent")]
#[derive(Debug, Clone)]
pub struct Component(SafeCOMComponent<vswriter::IVssComponent>);
impl_query_interface!(Component => vswriter::IVssComponent);
transparent_wrapper!(
    #[doc(alias = "IVssComponent")]
    pub struct IComponent(vswriter::IVssComponent);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(Component => IComponent);

////////////////////////////////////////////////////////////////////////////////
// IVssComponentEx
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vswriter::IVssComponentEx);

#[doc(alias = "IVssComponentEx")]
#[derive(Debug, Clone)]
pub struct ComponentEx(SafeCOMComponent<vswriter::IVssComponentEx>);
impl_query_interface!(ComponentEx => vswriter::IVssComponentEx);
transparent_wrapper!(
    #[doc(alias = "IVssComponentEx")]
    pub struct IComponentEx(vswriter::IVssComponentEx);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(ComponentEx => IComponentEx => IComponent);

////////////////////////////////////////////////////////////////////////////////
// IVssComponentEx2
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vswriter::IVssComponentEx2);

#[doc(alias = "IVssComponentEx2")]
#[derive(Debug, Clone)]
pub struct ComponentEx2(SafeCOMComponent<vswriter::IVssComponentEx2>);
impl_query_interface!(ComponentEx2 => vswriter::IVssComponentEx2);
transparent_wrapper!(
    /// Defines additional methods for reporting and retrieving component-level
    /// writer errors.
    #[doc(alias = "IVssComponentEx2")]
    pub struct IComponentEx2(vswriter::IVssComponentEx2);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(ComponentEx2 => IComponentEx2 => IComponentEx);

impl IComponentEx2 {
    /// VSS requesters call this method to retrieve component-level errors
    /// reported by writers.
    #[doc(alias = "GetFailure")]
    pub fn get_failure(&self) -> Result<GetFailureInfo, GetFailureError> {
        let mut failure: HRESULT = 0;
        let mut application_return_code: HRESULT = 0;
        let mut application_message: BSTR = null_mut();
        let mut reserved: DWORD = Default::default();
        let result = check_com(unsafe {
            self.0.GetFailure(
                &mut failure,
                &mut application_return_code,
                &mut application_message,
                &mut reserved,
            )
        });
        let application_message = unsafe { take_ownership_of_bstr(application_message) };
        result?;
        Ok(GetFailureInfo {
            failure: Some(failure).filter(|&hr| hr != S_OK).map(Into::into),
            application_return_code: Some(application_return_code).filter(|&hr| hr != S_OK),
            application_message: application_message.unwrap(),
        })
    }
    /// VSS writers call this method to report errors at the component level.
    #[doc(alias = "SetFailure")]
    pub fn set_failure(
        &self,
        failure: Option<ReportableWriterFailureError>,
        application_return_code: Option<HRESULT>,
        application_message: Option<&U16CStr>,
    ) -> Result<(), IComponentEx2SetFailureError> {
        check_com(unsafe {
            self.0.SetFailure(
                failure.map(HRESULT::from).unwrap_or(S_OK),
                application_return_code.unwrap_or(S_OK),
                application_message.map(|s| s.as_ptr()).unwrap_or(null()),
                0,
            )
        })?;
        Ok(())
    }
}

/// Information returned by the [`IComponentEx2::get_failure`] method.
#[derive(Clone)]
pub struct GetFailureInfo {
    /// The failure code that the writer passed for the `failure` parameter of
    /// the [`IComponentEx2::set_failure`] method.
    pub failure: Option<IVssComponentEx2WriterFailureError>,
    /// the return code that the writer passed for the `application_return_code`
    /// parameter of the [`set_failure`] method.
    ///
    /// [`set_failure`]: IComponentEx2::set_failure
    pub application_return_code: Option<HRESULT>,
    /// the application failure message that the writer passed for the
    /// `application_message` parameter of the [`set_failure`] method.
    ///
    /// [`set_failure`]: IComponentEx2::set_failure
    pub application_message: Option<BString>,
}

////////////////////////////////////////////////////////////////////////////////
// IVssCreateWriterMetadata
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe impl crate::safe_com_component::CorrectInterface for vswriter::IVssCreateWriterMetadata {}

transparent_wrapper!(
    #[doc(alias = "IVssCreateWriterMetadata")]
    pub struct ICreateWriterMetadata(vswriter::IVssCreateWriterMetadata);
);

////////////////////////////////////////////////////////////////////////////////
// Enumerations
////////////////////////////////////////////////////////////////////////////////

with_from!(
    [raw = vswriter::VSS_COMPONENT_TYPE, fallback = Undefined],
    /// Used by both the requester and the writer to specify the type of component
    /// being used with a shadow copy backup operation.
    ///
    /// # Remarks
    ///
    /// A writer sets a component's type when it adds the component to its Writer
    /// Metadata Document using `IVssCreateWriterMetadata::AddComponent`.
    ///
    /// Writers and requesters can find the type information of components selected
    /// for inclusion in a Backup Components Document through calls to
    /// `IVssComponent::GetComponentType` to return a component type directly.
    ///
    /// A requester can obtain the type of any component in a given writer's Writer
    /// Metadata Document by doing the following:
    ///
    /// 1. Using [`IVssExamineWriterMetadata::GetComponent`] to obtain a [`IVssWMComponent`] interface
    /// 2. Using [`IVssWMComponent::GetComponentInfo`] to return a [`VSS_COMPONENTINFO`] structure
    /// 3. Examining the [`Type`] member of the [`VSS_COMPONENTINFO`] object
    ///
    /// [`IVssExamineWriterMetadata::GetComponent`]: crate::vsbackup::IExamineWriterMetadata::get_component
    /// [`IVssWMComponent`]: crate::vsbackup::WMComponent
    /// [`IVssWMComponent::GetComponentInfo`]: crate::vsbackup::WMComponent::get_component_info
    /// [`VSS_COMPONENTINFO`]: crate::vsbackup::ComponentInfo
    /// [`Type`]: crate::vsbackup::ComponentInfo::component_type
    #[doc(alias = "VSS_COMPONENT_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum VssComponentType {
        /// Undefined component type.
        ///
        /// This value indicates an application error.
        #[doc(alias = "VSS_CT_UNDEFINED")]
        Undefined = vswriter::VSS_CT_UNDEFINED,
        /// Database component.
        #[doc(alias = "VSS_CT_DATABASE")]
        Database = vswriter::VSS_CT_DATABASE,
        /// File group component. This is any component other than a database.
        #[doc(alias = "VSS_CT_FILEGROUP")]
        FileGroup = vswriter::VSS_CT_FILEGROUP,
    }
);

with_from!(
    [
        raw = vswriter::VSS_FILE_RESTORE_STATUS,
        fallback = Undefined
    ],
    /// This enumeration defines the set of statuses of a file restore operation
    /// performed on the files managed by a selected component or component set
    /// (see [Working with Selectability and Logical Paths] for information on selecting components).
    ///
    /// [Working with Selectability and Logical Paths]: https://docs.microsoft.com/en-us/windows/desktop/VSS/working-with-selectability-and-logical-paths
    #[doc(alias = "VSS_FILE_RESTORE_STATUS")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FileRestoreStatus {
        #[doc(alias = "VSS_RS_UNDEFINED")]
        Undefined = vswriter::VSS_RS_UNDEFINED,
        #[doc(alias = "VSS_RS_NONE")]
        None = vswriter::VSS_RS_NONE,
        #[doc(alias = "VSS_RS_ALL")]
        All = vswriter::VSS_RS_ALL,
        #[doc(alias = "VSS_RS_FAILED")]
        Failed = vswriter::VSS_RS_FAILED,
    }
);

with_from!(
    [raw = vswriter::VSS_USAGE_TYPE, fallback = Undefined],
    /// Specifies how the host system uses the data managed by a writer involved
    /// in a VSS operation.
    #[doc(alias = "VSS_USAGE_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum UsageType {
        /// The usage type is not known.
        ///
        /// This indicates an error on the part of the writer.
        #[doc(alias = "VSS_UT_UNDEFINED")]
        Undefined = vswriter::VSS_UT_UNDEFINED,
        /// The data stored by the writer is part of the bootable system state.
        #[doc(alias = "VSS_UT_BOOTABLESYSTEMSTATE")]
        BootableSystemState = vswriter::VSS_UT_BOOTABLESYSTEMSTATE,
        /// The writer either stores data used by a system service or is a system
        /// service itself.
        #[doc(alias = "VSS_UT_SYSTEMSERVICE")]
        SystemService = vswriter::VSS_UT_SYSTEMSERVICE,
        /// The data is user data.
        #[doc(alias = "VSS_UT_USERDATA")]
        UserData = vswriter::VSS_UT_USERDATA,
        /// Unclassified data.
        #[doc(alias = "VSS_UT_OTHER")]
        Other = vswriter::VSS_UT_OTHER,
    }
);

with_from!(
    [raw = vswriter::VSS_SOURCE_TYPE, fallback = Undefined],
    /// Specifies the type of data that a writer manages.
    #[doc(alias = "VSS_SOURCE_TYPE")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SourceType {
        /// The source of the data is not known.
        ///
        /// This indicates a writer error, and the requester should report it.
        #[doc(alias = "VSS_ST_UNDEFINED")]
        Undefined = vswriter::VSS_ST_UNDEFINED,
        /// The source of the data is a database that supports transactions, such
        /// as Microsoft SQL Server.
        #[doc(alias = "VSS_ST_TRANSACTEDDB")]
        TransactedDb = vswriter::VSS_ST_TRANSACTEDDB,
        /// The source of the data is a database that does not support transactions.
        #[doc(alias = "VSS_ST_NONTRANSACTEDDB")]
        NonTransactedDb = vswriter::VSS_ST_NONTRANSACTEDDB,
        /// Unclassified source type—data will be in a file group.
        ///
        /// This is the default source type.
        #[doc(alias = "VSS_ST_OTHER")]
        Other = vswriter::VSS_ST_OTHER,
    }
);

raw_bitflags! {
    /// The `VssComponentFlags` enumeration is used by writers to indicate support
    /// for auto-recovery. These values are used in the [`component_flags`] member
    /// of the [`VssComponentInfo`] structure and the `dwComponentFlags` parameter
    /// of the `IVssCreateWriterMetadata::AddComponent` method.
    ///
    /// [`VssComponentInfo`]: crate::vsbackup::ComponentInfo
    /// [`component_flags`]: crate::vsbackup::ComponentInfo::component_flags
    #[doc(alias = "VSS_COMPONENT_FLAGS")]
    #[derive(Default)]
    pub struct VssComponentFlags: DWORD {
        /// The writer will need write access to this component after the shadow
        /// copy has been created.
        ///
        /// This flag can be used together with the `VSS_VOLSNAP_ATTR_TRANSPORTABLE`
        /// value of the `_VSS_VOLUME_SNAPSHOT_ATTRIBUTES` enumeration if the VSS
        /// hardware provider supports LUN masking. `Windows Vista and Windows Server
        /// 2003 with SP1`: This flag is incompatible with
        /// `VSS_VOLSNAP_ATTR_TRANSPORTABLE`.
        ///
        /// This flag is not supported for express writers.
        #[doc(alias = "VSS_CF_BACKUP_RECOVERY")]
        const BACKUP_RECOVERY = vswriter::VSS_CF_BACKUP_RECOVERY;
        /// If this is a rollback shadow copy
        /// (see the `VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY` value of the
        /// `_VSS_VOLUME_SNAPSHOT_ATTRIBUTES` enumeration), the writer for this
        /// component will need write access to this component after the shadow
        /// copy has been created.
        ///
        /// This flag can be used together with the `VSS_VOLSNAP_ATTR_TRANSPORTABLE`
        /// value of the `_VSS_VOLUME_SNAPSHOT_ATTRIBUTES` enumeration if the VSS
        /// hardware provider supports LUN masking. `Windows Vista and Windows Server
        /// 2003 with SP1`: This flag is incompatible with
        /// `VSS_VOLSNAP_ATTR_TRANSPORTABLE`.
        ///
        /// This flag is not supported for express writers.
        #[doc(alias = "VSS_CF_APP_ROLLBACK_RECOVERY")]
        const APP_ROLLBACK_RECOVERY = vswriter::VSS_CF_APP_ROLLBACK_RECOVERY;
        /// This component is not part of system state.
        ///
        /// `Windows Server 2003 with SP1`: This value is not supported until
        /// Windows Vista.
        #[doc(alias = "VSS_CF_NOT_SYSTEM_STATE")]
        const NOT_SYSTEM_STATE = vswriter::VSS_CF_NOT_SYSTEM_STATE;
    }
}

with_from!(
    [raw = vswriter::VSS_RESTOREMETHOD_ENUM, fallback = Undefined],
    /// Used by a writer at backup time to specify through its Writer Metadata
    /// Document the default file restore method to be used with all the files
    /// in all the components it manages.
    #[doc(alias = "VSS_RESTOREMETHOD_ENUM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum RestoreMethod {
        #[doc(alias = "VSS_RME_UNDEFINED")]
        Undefined = vswriter::VSS_RME_UNDEFINED,
        #[doc(alias = "VSS_RME_RESTORE_IF_NOT_THERE")]
        RestoreIfNotThere = vswriter::VSS_RME_RESTORE_IF_NOT_THERE,
        #[doc(alias = "VSS_RME_RESTORE_IF_CAN_REPLACE")]
        RestoreIfCanReplace = vswriter::VSS_RME_RESTORE_IF_CAN_REPLACE,
        #[doc(alias = "VSS_RME_STOP_RESTORE_START")]
        StopRestoreStart = vswriter::VSS_RME_STOP_RESTORE_START,
        #[doc(alias = "VSS_RME_RESTORE_TO_ALTERNATE_LOCATION")]
        RestoreToAlternateLocation = vswriter::VSS_RME_RESTORE_TO_ALTERNATE_LOCATION,
        #[doc(alias = "VSS_RME_RESTORE_AT_REBOOT")]
        RestoreAtReboot = vswriter::VSS_RME_RESTORE_AT_REBOOT,
        #[doc(alias = "VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE")]
        RestoreAtRebootIfCannotReplace = vswriter::VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE,
        #[doc(alias = "VSS_RME_CUSTOM")]
        Custom = vswriter::VSS_RME_CUSTOM,
        #[doc(alias = "VSS_RME_RESTORE_STOP_START")]
        RestoreStopStart = vswriter::VSS_RME_RESTORE_STOP_START,
    }
);

with_from!(
    [raw = vswriter::VSS_WRITERRESTORE_ENUM, fallback = Undefined],
    /// Used by a writer to indicate to a requester the conditions under which it
    /// will handle events generated during a restore operation.
    #[doc(alias = "VSS_WRITERRESTORE_ENUM")]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum WriterRestore {
        #[doc(alias = "VSS_WRE_UNDEFINED")]
        Undefined = vswriter::VSS_WRE_UNDEFINED,
        #[doc(alias = "VSS_WRE_NEVER")]
        Never = vswriter::VSS_WRE_NEVER,
        #[doc(alias = "VSS_WRE_IF_REPLACE_FAILS")]
        IfReplaceFails = vswriter::VSS_WRE_IF_REPLACE_FAILS,
        #[doc(alias = "VSS_WRE_ALWAYS")]
        Always = vswriter::VSS_WRE_ALWAYS,
    }
);

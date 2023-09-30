//! Bindings for the `vsbackup.h` header.
//!
//! This is required when developing a VSS "requester" but not when developing a
//! VSS "writer".
//!
//! # References
//!
//! [Vsbackup.h header - Win32 apps | Microsoft Docs](https://docs.microsoft.com/en-us/windows/win32/api/vsbackup/)
//!
//! [winapi::um::vsbackup - Rust](https://docs.rs/winapi/0.3.9/winapi/um/vsbackup/)

use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    mem::ManuallyDrop,
    ops::Deref,
    ptr::{null, null_mut},
    slice,
};

use widestring::U16CStr;
use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::REFIID,
        minwindef::{BOOL, DWORD, FALSE, TRUE, UINT, ULONG},
        winerror::{S_FALSE, S_OK},
        wtypes::BSTR,
    },
    um::{
        cguid::GUID_NULL,
        vsbackup::{self, PVSSCOMPONENTINFO, VSS_COMPONENTINFO},
        vss::{self, VSS_ID, VSS_PWSZ, VSS_SNAPSHOT_CONTEXT},
        vswriter,
        winnt::HRESULT,
        winnt::LONG,
    },
};
use winstr::{BStr, BString};

use super::{
    check_com,
    errors::*,
    impl_as_ref_and_borrow, impl_query_interface,
    safe_com_component::CustomIUnknown,
    take_ownership_of_bstr, transparent_wrapper, unsafe_deref_to_ref, unsafe_impl_as_IUnknown,
    vss::{
        BackupSchema, BackupType, EnumObject, HardwareOptions, IVssAsyncResult, ObjectType,
        RecoveryOptions, RestoreType, RollForwardType, SnapshotCapability, SnapshotContext,
        SnapshotProperties, VolumeSnapshotAttributes, VssAsync, WriterState,
    },
    vswriter::{
        FileRestoreStatus, IWriterComponents, RestoreMethod, SourceType, UsageType,
        VssComponentFlags, VssComponentType, WMDependency, WMFileDescriptor, WriterRestore,
    },
    RawBitFlags, SafeCOMComponent, VssU16CString,
};

////////////////////////////////////////////////////////////////////////////////
// IVssBackupComponents
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssBackupComponents);

#[doc(alias = "IVssBackupComponents")]
#[derive(Debug, Clone)]
pub struct BackupComponents(SafeCOMComponent<vsbackup::IVssBackupComponents>);
impl_query_interface!(BackupComponents => vsbackup::IVssBackupComponents);
transparent_wrapper!(
    #[doc(alias = "IVssBackupComponents")]
    pub struct IBackupComponents(vsbackup::IVssBackupComponents);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(BackupComponents => IBackupComponents);

impl BackupComponents {
    #[doc(alias = "CreateVssBackupComponents")]
    pub fn new() -> Result<Self, CreateVssBackupComponentsError> {
        unsafe {
            let mut comp = null_mut::<vsbackup::IVssBackupComponents>();
            check_com(vsbackup::CreateVssBackupComponents(&mut comp))?;
            Ok(Self(SafeCOMComponent::new(comp)))
        }
    }
}
impl IBackupComponents {
    /// The `abort_backup` method notifies VSS that a backup operation was terminated.
    ///
    /// This method must be called if a backup operation terminates after the
    /// creation of a shadow copy set with [`IBackupComponents::start_snapshot_set`]
    /// and before [`IBackupComponents::do_snapshot_set`] returns.
    ///
    /// If `abort_backup` is called and no shadow copy or backup operations are
    /// underway, it is ignored.
    ///
    /// # Remarks
    ///
    /// `abort_backup` generates an `Abort` event, which is handled by each instance
    /// of each writer through the `CVssWriter::OnAbort` method.
    #[doc(alias = "AbortBackup")]
    pub fn abort_backup(&self) -> Result<(), AbortBackupError> {
        check_com(unsafe { self.0.AbortBackup() })?;
        Ok(())
    }
    /// Used by a requester to indicate that an alternate location mapping was
    /// used to restore all the members of a file set in a given component.
    #[doc(alias = "AddAlternativeLocationMapping")]
    pub fn add_alternative_location_mapping(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        path: &U16CStr,
        file_specification: &U16CStr,
        recursive: bool,
        destination: &U16CStr,
    ) -> Result<(), AddAlternativeLocationMappingError> {
        check_com(unsafe {
            self.0.AddAlternativeLocationMapping(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                path.as_ptr(),
                file_specification.as_ptr(),
                recursive,
                destination.as_ptr(),
            )
        })?;
        Ok(())
    }
    #[doc(alias = "AddComponent")]
    pub fn add_component(
        &self,
        instance_id: VSS_ID,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
    ) -> Result<(), AddComponentError> {
        check_com(unsafe {
            self.0.AddComponent(
                instance_id,
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Used by a requester during a restore operation to indicate that the backup
    /// application plans to restore files to a new location.
    #[doc(alias = "AddNewTarget")]
    pub fn add_new_target(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        path: &U16CStr,
        file_name: &U16CStr,
        recursive: bool,
        alternate_path: &U16CStr,
    ) -> Result<(), AddNewTargetError> {
        check_com(unsafe {
            self.0.AddNewTarget(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                path.as_ptr(),
                file_name.as_ptr(),
                recursive,
                alternate_path.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Indicates that a subcomponent member of a component set, which had been
    /// marked as nonselectable for backup but is marked selectable for restore,
    /// is to be restored irrespective of whether any other member of the component
    /// set will be restored.
    #[doc(alias = "AddRestoreSubcomponent")]
    pub fn add_restore_subcomponent(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        sub_component_logical_path: &U16CStr,
        sub_component_name: &U16CStr,
    ) -> Result<(), AddRestoreSubcomponentError> {
        check_com(unsafe {
            self.0.AddRestoreSubcomponent(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                sub_component_logical_path.as_ptr(),
                sub_component_name.as_ptr(),
                false,
            )
        })?;
        Ok(())
    }
    /// Adds an original volume or original remote file share to the shadow copy
    /// set.
    #[doc(alias = "AddToSnapshotSet")]
    pub fn add_to_snapshot_set(
        &self,
        volume_name: &U16CStr,
        provider_id: Option<VSS_ID>,
    ) -> Result<VSS_ID, AddToSnapshotSetError> {
        let mut snapshot_id: VSS_ID = Default::default();
        check_com(unsafe {
            self.0.AddToSnapshotSet(
                volume_name.as_ptr() as *mut _,
                provider_id.unwrap_or(GUID_NULL),
                &mut snapshot_id,
            )
        })?;
        Ok(snapshot_id)
    }
    /// Cause VSS to generate a `BackupComplete` event, which signals writers that
    /// the backup process has completed.
    #[doc(alias = "BackupComplete")]
    pub fn backup_complete(&self) -> IVssAsyncResult<BackupCompleteError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.BackupComplete(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Cause the existence of a shadow copy set to be "forgotten" by VSS.
    #[doc(alias = "BreakSnapshotSet")]
    pub fn break_snapshot_set(&self, snapshot_set_id: VSS_ID) -> Result<(), BreakSnapshotSetError> {
        check_com(unsafe { self.0.BreakSnapshotSet(snapshot_set_id) })?;
        Ok(())
    }
    /// Delete one or more shadow copies or a shadow copy set.
    #[doc(alias = "DeleteSnapshots")]
    pub fn delete_snapshots(
        &self,
        source_object_id: VSS_ID,
        source_object_type: ObjectType,
        force_delete: bool,
    ) -> DeleteSnapshotsInfo {
        let mut deleted_snapshots = 0;
        let mut nondeleted_snapshot_id = Default::default();
        let error = check_com(unsafe {
            self.0.DeleteSnapshots(
                source_object_id,
                source_object_type.into(),
                if force_delete { TRUE } else { FALSE },
                &mut deleted_snapshots,
                &mut nondeleted_snapshot_id,
            )
        })
        .err();
        DeleteSnapshotsInfo {
            deleted_snapshots: deleted_snapshots as _,
            nondeleted_snapshot_id: Some(nondeleted_snapshot_id).filter(|_| error.is_some()),
            error: error.map(Into::into),
        }
    }
    /// Prevent a specific class of writers from receiving any events.
    #[doc(alias = "DisableWriterClasses")]
    pub fn disable_writer_classes(
        &self,
        writer_class_ids: &[VSS_ID],
    ) -> Result<(), DisableWriterClassesError> {
        let len = writer_class_ids.len();
        // The documentation says 1 or more entries, so to be safe we assert that:
        assert_ne!(len, 0);
        check_com(unsafe {
            self.0
                .DisableWriterClasses(writer_class_ids.as_ptr(), len.try_into().unwrap())
        })?;
        Ok(())
    }
    /// Disables a specified writer instance or instances.
    #[doc(alias = "DisableWriterInstances")]
    pub fn disable_writer_instances(
        &self,
        writer_instance_ids: &[VSS_ID],
    ) -> Result<(), DisableWriterInstancesError> {
        let len = writer_instance_ids.len();
        // The documentation says 1 or more entries, so to be safe we assert that:
        assert_ne!(len, 0);
        check_com(unsafe {
            self.0
                .DisableWriterInstances(writer_instance_ids.as_ptr(), len.try_into().unwrap())
        })?;
        Ok(())
    }
    /// Commits all shadow copies in this set simultaneously.
    #[doc(alias = "DoSnapshotSet")]
    pub fn do_snapshot_set(&self) -> IVssAsyncResult<DoSnapshotSetError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.DoSnapshotSet(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Prevent a specific class of writers from receiving any events.
    #[doc(alias = "EnableWriterClasses")]
    pub fn enable_writer_classes(
        &self,
        writer_class_ids: &[VSS_ID],
    ) -> Result<(), EnableWriterClassesError> {
        let len = writer_class_ids.len();
        // The documentation says 1 or more entries, so to be safe we assert that:
        assert_ne!(len, 0);
        check_com(unsafe {
            self.0
                .EnableWriterClasses(writer_class_ids.as_ptr(), len.try_into().unwrap())
        })?;
        Ok(())
    }
    /// Exposes a shadow copy as a drive letter, mounted folder, or file share.
    ///
    /// Returns the exposed name of the shadow copy. This is either a share name,
    /// a drive letter followed by a colon, or a mounted folder.
    #[doc(alias = "ExposeSnapshot")]
    pub fn expose_snapshot(
        &self,
        snapshot_id: VSS_ID,
        path_from_root: Option<&U16CStr>,
        attributes: RawBitFlags<VolumeSnapshotAttributes>,
        expose: Option<&U16CStr>,
    ) -> Result<VssU16CString, ExposeSnapshotError> {
        let mut exposed: VSS_PWSZ = null_mut();
        let result = check_com(unsafe {
            self.0.ExposeSnapshot(
                snapshot_id,
                path_from_root.map(|s| s.as_ptr()).unwrap_or(null()) as *mut _,
                attributes.raw() as _,
                expose.map(|s| s.as_ptr()).unwrap_or(null()) as *mut _,
                // TODO: the `winapi` binding for this argument doesn't match the
                // documentation, fortunately its just a pointer anyway so it
                // should have no effect.
                &mut exposed as *mut VSS_PWSZ as *mut _,
            )
        });
        let exposed = unsafe { VssU16CString::from_nullable_ptr(exposed) };
        result?;
        Ok(exposed.expect("the returned string from `ExposeSnapshot` shouldn't be null"))
    }
    /// Frees system resources allocated when
    /// [`IBackupComponents::gather_writer_metadata`] was called.
    #[doc(alias = "FreeWriterMetadata")]
    pub fn free_writer_metadata(&self) -> Result<(), FreeWriterMetadataError> {
        check_com(unsafe { self.0.FreeWriterMetadata() })?;
        Ok(())
    }
    #[doc(alias = "FreeWriterStatus")]
    pub fn free_writer_status(&self) -> Result<(), FreeWriterStatusError> {
        // TODO: is it safe to call this method before the `IVssAsync` operation
        // from `GatherWriterStatus` completes. In other words, are we freeing
        // resources that are in use or will the interface handle that?
        check_com(unsafe { self.0.FreeWriterStatus() })?;
        Ok(())
    }
    /// Prompt each writer to send the metadata they have collected.
    ///
    /// # Remarks
    ///
    /// Should be called only once during the lifetime of a given
    /// `IVssBackupComponents` object.
    #[doc(alias = "GatherWriterMetadata")]
    pub fn gather_writer_metadata(&self) -> IVssAsyncResult<GatherWriterMetadataError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.GatherWriterMetadata(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Prompt each writer to send a status message.
    ///
    /// # Remarks
    ///
    /// The caller of this method should also call
    /// [`IBackupComponents::free_writer_status`] after receiving the status
    /// of each writer.
    #[doc(alias = "GatherWriterStatus")]
    pub fn gather_writer_status(&self) -> IVssAsyncResult<GatherWriterStatusError> {
        // TODO: maybe wrap the returned `IVssAsync` operation in a wrapper
        // that calls `free_writer_status` when the operation completes to
        // make it harder to leak memory?
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.GatherWriterStatus(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Gets the properties of the specified shadow copy.
    #[doc(alias = "GetSnapshotProperties")]
    pub fn get_snapshot_properties(
        &self,
        snapshot_id: VSS_ID,
    ) -> Result<SnapshotProperties, GetSnapshotPropertiesError> {
        let mut prop: vss::VSS_SNAPSHOT_PROP = Default::default();
        check_com(unsafe { self.0.GetSnapshotProperties(snapshot_id, &mut prop) })?;
        Ok(SnapshotProperties(prop))
    }
    /// Returns information about those components of a given writer that have
    /// been stored in a requester's Backup Components Document.
    #[doc(alias = "GetWriterComponents")]
    pub fn get_writer_components(
        &self,
        writer_index: u32,
    ) -> Result<WriterComponentsExt, GetWriterComponentsError> {
        let mut writer: *mut vsbackup::IVssWriterComponentsExt = null_mut();
        check_com(unsafe { self.0.GetWriterComponents(writer_index, &mut writer) })?;
        Ok(WriterComponentsExt(unsafe {
            SafeCOMComponent::new(writer)
        }))
    }
    /// Returns the number of writers whose components have been added to a
    /// requester's Backup Components Document.
    #[doc(alias = "GetWriterComponentsCount")]
    pub fn get_writer_components_count(&self) -> Result<u32, GetWriterComponentsCountError> {
        let mut components: UINT = 0;
        check_com(unsafe { self.0.GetWriterComponentsCount(&mut components) })?;
        Ok(components)
    }
    /// The GetWriterMetadata method returns the metadata for a specific writer
    /// running on the system.
    ///
    /// # Parameters
    ///
    /// ## writer_index
    ///
    /// Index of the writer whose metadata is to be retrieved. The value of this
    /// parameter is an integer from 0 to n–1 inclusive, where n is the total
    /// number of writers on the current system. The value of n is returned by
    /// [`IBackupComponents::get_writer_metadata_count`].
    ///
    /// ## writer_instance
    ///
    /// The instance identifier of the writer that collected the metadata.
    ///
    /// # Remarks
    ///
    /// A requester must call the asynchronous operation [`IBackupComponents::gather_writer_metadata`]
    /// and wait for it to complete prior to calling `get_writer_metadata`.
    ///
    /// Although [IBackupComponents::gather_writer_metadata`] must be called
    /// prior to either a restore or backup operation, `get_writer_metadata` is
    /// not typically called for restores.
    ///
    /// Component information retrieved (during backup operations) using
    /// [`IExamineWriterMetadata::get_component`], where the
    /// [`ExamineWriterMetadata`] interface has been returned by `get_writer_metadata`,
    /// comes from the Writer Metadata Document of a live writer process.
    ///
    /// This is in contrast to the information returned by [`IBackupComponents::get_writer_components`]
    /// (during restore operations), which was stored in the Backup Components
    /// Document by calls to [`IBackupComponents::add_component`].
    #[doc(alias = "GetWriterMetadata")]
    pub fn get_writer_metadata(
        &self,
        writer_index: u32,
        writer_instance: VSS_ID,
    ) -> Result<ExamineWriterMetadata, GetWriterMetadataError> {
        let mut metadata = null_mut::<vsbackup::IVssExamineWriterMetadata>();
        check_com(unsafe {
            self.0.GetWriterMetadata(
                writer_index,
                &writer_instance as *const _ as *mut _,
                &mut metadata,
            )
        })?;
        Ok(ExamineWriterMetadata(unsafe {
            SafeCOMComponent::new(metadata)
        }))
    }
    /// Returns the number of writers with metadata.
    #[doc(alias = "GetWriterMetadataCount")]
    pub fn get_writer_metadata_count(&self) -> Result<u32, GetWriterMetadataCountError> {
        let mut writers: UINT = 0;
        check_com(unsafe { self.0.GetWriterMetadataCount(&mut writers) })?;
        Ok(writers)
    }
    /// Returns the status of the specified writer.
    #[doc(alias = "GetWriterStatus")]
    pub fn get_writer_status(
        &self,
        writer_index: u32,
    ) -> Result<GetWriterStatusInfo, GetWriterStatusError> {
        let mut instance_id: VSS_ID = Default::default();
        let mut writer_id: VSS_ID = Default::default();
        let mut writer: BSTR = null_mut();
        let mut status: vss::VSS_WRITER_STATE = Default::default();
        let mut writer_failure: HRESULT = Default::default();
        let hr = unsafe {
            self.0.GetWriterStatus(
                writer_index,
                &mut instance_id,
                &mut writer_id,
                &mut writer,
                &mut status,
                &mut writer_failure,
            )
        };
        let writer = unsafe { take_ownership_of_bstr(writer) };
        check_com(hr)?;
        Ok(GetWriterStatusInfo {
            instance_id,
            writer_id,
            writer: writer
                .unwrap()
                .expect("The writer string returned by GetWriterStatus shouldn't be null"),
            status: status.into(),
            writer_failure: Some(writer_failure.into()).filter(|_| writer_failure != S_OK),
        })
    }
    /// Returns the number of writers with status.
    #[doc(alias = "GetWriterStatusCount")]
    pub fn get_writer_status_count(&self) -> Result<u32, GetWriterStatusCountError> {
        let mut writers: UINT = 0;
        check_com(unsafe { self.0.GetWriterStatusCount(&mut writers) })?;
        Ok(writers)
    }
    /// imports shadow copies transported from a different machine.
    ///
    /// Note: This method is supported only on Windows Server operating systems
    /// and for Volume Shadow Copy Service hardware providers.
    #[doc(alias = "ImportSnapshots")]
    pub fn import_snapshots(&self) -> IVssAsyncResult<ImportSnapshotsError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.ImportSnapshots(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Initializes the backup components metadata in preparation for backup.
    #[doc(alias = "InitializeForBackup")]
    pub fn initialize_for_backup(
        &self,
        xml: Option<&BStr>,
    ) -> Result<(), InitializeForBackupError> {
        // TODO: is the xml string reference used after this function returns?
        check_com(unsafe {
            self.0
                .InitializeForBackup(xml.map(|v| v.as_bstr()).unwrap_or(null_mut()))
        })?;
        Ok(())
    }
    /// initializes the `IBackupComponents` interface in preparation for a restore
    /// operation.
    #[doc(alias = "InitializeForRestore")]
    pub fn initialize_for_restore(&self, xml: &BStr) -> Result<(), InitializeForRestoreError> {
        check_com(unsafe { self.0.InitializeForRestore(xml.as_bstr()) })?;
        Ok(())
    }
    /// Determines whether the specified provider supports shadow copies on the
    /// specified volume or remote file share.
    #[doc(alias = "IsVolumeSupported")]
    pub fn is_volume_supported(
        &self,
        provider_id: Option<VSS_ID>,
        volume_name: &U16CStr,
    ) -> Result<bool, IsVolumeSupportedError> {
        let mut supported_by_this_provider: BOOL = FALSE;
        check_com(unsafe {
            self.0.IsVolumeSupported(
                provider_id.unwrap_or(GUID_NULL),
                volume_name.as_ptr() as *mut _,
                &mut supported_by_this_provider,
            )
        })?;
        Ok(supported_by_this_provider == TRUE)
    }
    /// Cause VSS to generate a `PostRestore` event, signaling writers that the
    /// current restore operation has finished.
    #[doc(alias = "PostRestore")]
    pub fn post_restore(&self) -> IVssAsyncResult<PostRestoreError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.PostRestore(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Cause VSS to generate a PrepareForBackup event, signaling writers to
    /// prepare for an upcoming backup operation. This makes a requester's
    /// Backup Components Document available to writers.
    #[doc(alias = "PrepareForBackup")]
    pub fn prepare_for_backup(&self) -> IVssAsyncResult<PrepareForBackupError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.PrepareForBackup(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    #[doc(alias = "PreRestore")]
    pub fn pre_restore(&self) -> IVssAsyncResult<PreRestoreError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.PreRestore(&mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Query providers on the system and/or the completed shadow copies in the
    /// system that reside in the current context. The method can be called only
    /// during backup operations.
    #[doc(alias = "Query")]
    pub fn query(&self, returned_objects_type: ObjectType) -> Result<EnumObject, QueryError> {
        let mut enumerator = null_mut::<vss::IVssEnumObject>();
        check_com(unsafe {
            self.0.Query(
                GUID_NULL,
                ObjectType::None.into(),
                returned_objects_type.into(),
                &mut enumerator,
            )
        })?;
        Ok(EnumObject(unsafe { SafeCOMComponent::new(enumerator) }))
    }
    /// Used to determine the status of the revert operation.
    #[doc(alias = "QueryRevertStatus")]
    pub fn query_revert_status(&self, volume: &U16CStr) -> IVssAsyncResult<QueryRevertStatusError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe {
            self.0
                .QueryRevertStatus(volume.as_ptr() as *mut _, &mut task)
        })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Reverts a volume to a previous shadow copy. Only shadow copies created
    /// with persistent contexts are supported.
    ///
    /// Note: This method is only supported on Windows Server operating systems.
    ///
    /// # Remarks
    ///
    /// This operation cannot be canceled, or undone once completed. If the
    /// computer is rebooted during the revert operation, the revert process
    /// will continue when the system is restarted.
    #[doc(alias = "RevertToSnapshot")]
    pub fn revert_to_snapshot(
        &self,
        snapshot_id: VSS_ID,
        force_dismount: bool,
    ) -> Result<(), RevertToSnapshotError> {
        check_com(unsafe {
            self.0
                .RevertToSnapshot(snapshot_id, if force_dismount { TRUE } else { FALSE })
        })?;
        Ok(())
    }
    /// Saves the Backup Components Document containing a requester's state
    /// information to a specified string. This XML document, which contains
    /// the Backup Components Document, should always be securely saved as part
    /// of a backup operation.
    #[doc(alias = "SaveAsXML")]
    pub fn save_as_xml(&self) -> Result<BString, IBackupComponentsSaveAsXMLError> {
        let mut xml: BSTR = null_mut();
        let hr = unsafe { self.0.SaveAsXML(&mut xml) };

        // TODO: the ownership of the returned string isn't documented well, but
        // we assume it is the same as other methods the get a BSTR returned to
        // them.
        let xml = unsafe { take_ownership_of_bstr(xml) };

        check_com(hr)?;
        Ok(xml
            .unwrap()
            .expect("the xml string returned from SaveAsXML shouldn't be null"))
    }
    /// Used by a requester during incremental or differential restore operations
    /// to indicate to writers that a given component will require additional
    /// restore operations to completely retrieve it.
    #[doc(alias = "SetAdditionalRestores")]
    pub fn set_additional_restores(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        additional_restores: bool,
    ) -> Result<(), SetAdditionalRestoresError> {
        check_com(unsafe {
            self.0.SetAdditionalRestores(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                additional_restores,
            )
        })?;
        Ok(())
    }
    /// Sets a string of private, or writer-dependent, backup parameters for a
    /// component.
    #[doc(alias = "SetBackupOptions")]
    pub fn set_backup_options(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        backup_options: &U16CStr,
    ) -> Result<(), SetBackupOptionsError> {
        check_com(unsafe {
            self.0.SetBackupOptions(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                backup_options.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Defines an overall configuration for a backup operation.
    #[doc(alias = "SetBackupState")]
    pub fn set_backup_state(
        &self,
        select_components: bool,
        backup_bootable_system_state: bool,
        backup_type: BackupType,
        partial_file_support: bool,
    ) -> Result<(), SetBackupStateError> {
        check_com(unsafe {
            self.0.SetBackupState(
                select_components,
                backup_bootable_system_state,
                backup_type.into(),
                partial_file_support,
            )
        })?;
        Ok(())
    }
    /// Indicates whether the backup of the specified component of a specific
    /// writer was successful.
    #[doc(alias = "SetBackupSucceeded")]
    pub fn set_backup_succeeded(
        &self,
        instance_id: VSS_ID,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        succeeded: bool,
    ) -> Result<(), SetBackupSucceededError> {
        check_com(unsafe {
            self.0.SetBackupSucceeded(
                instance_id,
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                succeeded,
            )
        })?;
        Ok(())
    }
    /// Sets the context for subsequent shadow copy-related operations.
    ///
    /// Note that both arguments implement `Default` with sensible values.
    #[doc(alias = "SetContext")]
    pub fn set_context(
        &self,
        context: SnapshotContext,
        attributes: RawBitFlags<VolumeSnapshotAttributes>,
    ) -> Result<(), SetContextError> {
        let context: LONG =
            (VSS_SNAPSHOT_CONTEXT::from(context) as LONG) | (attributes.raw() as LONG);
        check_com(unsafe { self.0.SetContext(context) })?;
        Ok(())
    }
    /// Indicates whether some, all, or no files were successfully restored.
    #[doc(alias = "SetFileRestoreStatus")]
    pub fn set_file_restore_status(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        status: FileRestoreStatus,
    ) -> Result<(), SetFileRestoreStatusError> {
        check_com(unsafe {
            self.0.SetFileRestoreStatus(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                status.into(),
            )
        })?;
        Ok(())
    }
    /// Sets the backup stamp of an earlier backup operation, upon which a
    /// differential or incremental backup operation will be based.
    ///
    /// The method can be called only during a backup operation.
    #[doc(alias = "SetPreviousBackupStamp")]
    pub fn set_previous_backup_stamp(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        previous_backup_stamp: &U16CStr,
    ) -> Result<(), SetPreviousBackupStampError> {
        check_com(unsafe {
            self.0.SetPreviousBackupStamp(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                previous_backup_stamp.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Used when a partial file operation requires a ranges file, and that file
    /// has been restored to a location other than its original one.
    #[doc(alias = "SetRangesFilePath")]
    pub fn set_ranges_file_path(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        partial_file_index: u32,
        ranges_file: &U16CStr,
    ) -> Result<(), SetRangesFilePathError> {
        check_com(unsafe {
            self.0.SetRangesFilePath(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                partial_file_index,
                ranges_file.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Sets a string of private, or writer-dependent, restore parameters for a
    /// writer component.
    #[doc(alias = "SetRestoreOptions")]
    pub fn set_restore_options(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        restore_options: &U16CStr,
    ) -> Result<(), SetRestoreOptionsError> {
        check_com(unsafe {
            self.0.SetRestoreOptions(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                restore_options.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Defines an overall configuration for a restore operation.
    #[doc(alias = "SetRestoreState")]
    pub fn set_restore_state(&self, restore_type: RestoreType) -> Result<(), SetRestoreStateError> {
        check_com(unsafe { self.0.SetRestoreState(restore_type.into()) })?;
        Ok(())
    }
    /// Indicates whether the specified selectable component is selected for
    /// restoration.
    #[doc(alias = "SetSelectedForRestore")]
    pub fn set_selected_for_restore(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        selected_for_restore: bool,
    ) -> Result<(), SetSelectedForRestoreError> {
        check_com(unsafe {
            self.0.SetSelectedForRestore(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                selected_for_restore,
            )
        })?;
        Ok(())
    }
    /// Creates a new, empty shadow copy set.
    #[doc(alias = "StartSnapshotSet")]
    pub fn start_snapshot_set(&self) -> Result<VSS_ID, StartSnapshotSetError> {
        let mut snapshot_set_id: VSS_ID = Default::default();
        check_com(unsafe { self.0.StartSnapshotSet(&mut snapshot_set_id) })?;
        Ok(snapshot_set_id)
    }
}

/// Info returned by [`IBackupComponents::delete_snapshots`].
#[derive(Clone, Copy)]
pub struct DeleteSnapshotsInfo {
    /// Number of deleted shadow copies.
    pub deleted_snapshots: u32,
    /// If an error occurs, the value of this parameter is the identifier of the
    /// first shadow copy that could not be deleted.
    pub nondeleted_snapshot_id: Option<VSS_ID>,
    /// An error if something went wrong.
    pub error: Option<DeleteSnapshotsError>,
}

////////////////////////////////////////////////////////////////////////////////
// IVssBackupComponentsEx
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssBackupComponentsEx);

#[doc(alias = "IVssBackupComponentsEx")]
#[derive(Debug, Clone)]
pub struct BackupComponentsEx(SafeCOMComponent<vsbackup::IVssBackupComponentsEx>);
impl_query_interface!(BackupComponentsEx => vsbackup::IVssBackupComponentsEx);
transparent_wrapper!(
    #[doc(alias = "IVssBackupComponentsEx")]
    pub struct IBackupComponentsEx(vsbackup::IVssBackupComponentsEx);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(BackupComponentsEx => IBackupComponentsEx => IBackupComponents);

impl IBackupComponentsEx {
    /// Returns the metadata for a specific writer instance running on the system.
    #[doc(alias = "GetWriterMetadataEx")]
    pub fn get_writer_metadata_ex(
        &self,
        writer_index: u32,
    ) -> Result<GetWriterMetadataExInfo, GetWriterMetadataExError> {
        let mut instance_id: VSS_ID = Default::default();
        let mut metadata: *mut vsbackup::IVssExamineWriterMetadataEx = null_mut();
        check_com(unsafe {
            self.0
                .GetWriterMetadataEx(writer_index, &mut instance_id, &mut metadata)
        })?;
        Ok(GetWriterMetadataExInfo {
            instance_id,
            metadata: ExamineWriterMetadataEx(unsafe { SafeCOMComponent::new(metadata) }),
        })
    }
    /// Indicates whether the specified selectable component is selected for
    /// restoration to a specified writer instance.
    #[doc(alias = "SetSelectedForRestoreEx")]
    pub fn set_selected_for_restore_ex(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        selected_for_restore: bool,
        instance_id: Option<VSS_ID>,
    ) -> Result<(), SetSelectedForRestoreExError> {
        check_com(unsafe {
            self.0.SetSelectedForRestoreEx(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                selected_for_restore,
                instance_id.unwrap_or(GUID_NULL),
            )
        })?;
        Ok(())
    }
}
/// Data returned by the [`IBackupComponentsEx::get_writer_metadata_ex`] method.
pub struct GetWriterMetadataExInfo {
    /// Identifier of the writer that collected the metadata.
    pub instance_id: VSS_ID,
    /// Object that contains the metadata.
    pub metadata: ExamineWriterMetadataEx,
}

/// Information returned by the [`IBackupComponents::get_writer_status`]
/// method.
#[derive(Clone)]
pub struct GetWriterStatusInfo {
    pub instance_id: VSS_ID,
    pub writer_id: VSS_ID,
    pub writer: BString,
    pub status: WriterState,
    pub writer_failure: Option<WriterFailureError>,
}

////////////////////////////////////////////////////////////////////////////////
// IVssBackupComponentsEx2
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssBackupComponentsEx2);

#[doc(alias = "IVssBackupComponentsEx2")]
#[derive(Debug, Clone)]
pub struct BackupComponentsEx2(SafeCOMComponent<vsbackup::IVssBackupComponentsEx2>);
impl_query_interface!(BackupComponentsEx2 => vsbackup::IVssBackupComponentsEx2);
transparent_wrapper!(
    #[doc(alias = "IVssBackupComponentsEx2")]
    pub struct IBackupComponentsEx2(vsbackup::IVssBackupComponentsEx2);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(BackupComponentsEx2 => IBackupComponentsEx2 => IBackupComponentsEx);

impl IBackupComponentsEx2 {
    /// Breaks a shadow copy set according to requester-specified options.
    #[doc(alias = "BreakSnapshotSetEx")]
    pub fn break_snapshot_set_ex(
        &self,
        snapshot_set_id: VSS_ID,
        break_flags: RawBitFlags<HardwareOptions>,
    ) -> IVssAsyncResult<BreakSnapshotSetExError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe {
            self.0
                .BreakSnapshotSetEx(snapshot_set_id, break_flags.raw(), &mut task)
        })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
    /// Marks the restore of a component as authoritative for a replicated data
    /// store.
    #[doc(alias = "SetAuthoritativeRestore")]
    pub fn set_authoritative_restore(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        authoritative: bool,
    ) -> Result<(), SetAuthoritativeRestoreError> {
        check_com(unsafe {
            self.0.SetAuthoritativeRestore(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                authoritative,
            )
        })?;
        Ok(())
    }
    /// Assigns a new logical name to a component that is being restored.
    #[doc(alias = "SetRestoreName")]
    pub fn set_restore_name(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        restore_name: &U16CStr,
    ) -> Result<(), SetRestoreNameError> {
        check_com(unsafe {
            self.0.SetRestoreName(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                restore_name.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Assigns a new logical name to a component that is being restored.
    #[doc(alias = "SetRollForward")]
    pub fn set_roll_forward(
        &self,
        writer_id: VSS_ID,
        component_type: VssComponentType,
        logical_path: Option<&U16CStr>,
        component_name: &U16CStr,
        roll_forward_type: RollForwardType,
        roll_forward_point: &U16CStr,
    ) -> Result<(), SetRollForwardError> {
        check_com(unsafe {
            self.0.SetRollForward(
                writer_id,
                component_type.into(),
                logical_path.map(|s| s.as_ptr()).unwrap_or(null()),
                component_name.as_ptr(),
                roll_forward_type.into(),
                roll_forward_point.as_ptr(),
            )
        })?;
        Ok(())
    }
    /// Unexposes a shadow copy either by deleting the file share or by removing
    /// the drive letter or mounted folder.
    #[doc(alias = "UnexposeSnapshot")]
    pub fn unexpose_snapshot(&self, snapshot_id: VSS_ID) -> Result<(), UnexposeSnapshotError> {
        check_com(unsafe { self.0.UnexposeSnapshot(snapshot_id) })?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// IVssBackupComponentsEx3
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssBackupComponentsEx3);

#[doc(alias = "IVssBackupComponentsEx3")]
#[derive(Debug, Clone)]
pub struct BackupComponentsEx3(SafeCOMComponent<vsbackup::IVssBackupComponentsEx3>);
impl_query_interface!(BackupComponentsEx3 => vsbackup::IVssBackupComponentsEx3);
transparent_wrapper!(
    #[doc(alias = "IVssBackupComponentsEx3")]
    pub struct IBackupComponentsEx3(vsbackup::IVssBackupComponentsEx3);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(BackupComponentsEx3 => IBackupComponentsEx3 => IBackupComponentsEx2);

impl IBackupComponentsEx3 {
    /// Specifies the volumes to be included in a LUN resynchronization operation.
    /// This method is supported only on Windows server operating systems.
    #[doc(alias = "AddSnapshotToRecoverySet")]
    pub fn add_snapshot_to_recovery_set(
        &self,
        snapshot_id: VSS_ID,
        destination_volume: Option<&U16CStr>,
    ) -> Result<(), AddSnapshotToRecoverySetError> {
        check_com(unsafe {
            self.0.AddSnapshotToRecoverySet(
                snapshot_id,
                0,
                destination_volume
                    .map(|s| s.as_ptr() as *mut _)
                    .unwrap_or(null_mut()),
            )
        })?;
        Ok(())
    }
    /// Returns the requester's session identifier.
    #[doc(alias = "GetSessionId")]
    pub fn get_session_id(&self) -> Result<VSS_ID, GetSessionIdError> {
        let mut session_id = Default::default();
        check_com(unsafe { self.0.GetSessionId(&mut session_id) })?;
        Ok(session_id)
    }
    #[doc(alias = "GetWriterStatusEx")]
    pub fn get_writer_status_ex(
        &self,
        writer_index: u32,
        get_application_return_code: bool,
        get_application_message: bool,
    ) -> Result<GetWriterStatusExInfo, GetWriterStatusExError> {
        let mut instance_id: VSS_ID = Default::default();
        let mut writer_id: VSS_ID = Default::default();
        let mut writer: BSTR = null_mut();
        let mut status: vss::VSS_WRITER_STATE = Default::default();
        let mut writer_failure: HRESULT = Default::default();
        let mut application_return_code: HRESULT = Default::default();
        let mut application_message: BSTR = null_mut();
        let hr = unsafe {
            self.0.GetWriterStatusEx(
                writer_index,
                &mut instance_id,
                &mut writer_id,
                &mut writer,
                &mut status,
                &mut writer_failure,
                if get_application_return_code {
                    &mut application_return_code
                } else {
                    null_mut()
                },
                if get_application_message {
                    &mut application_message
                } else {
                    null_mut()
                },
            )
        };
        let writer = unsafe { take_ownership_of_bstr(writer) };
        let application_message = unsafe { take_ownership_of_bstr(application_message) };
        check_com(hr)?;
        Ok(GetWriterStatusExInfo {
            instance_id,
            writer_id,
            writer: writer
                .unwrap()
                .expect("GetWriterStatusEx's returned writer shouldn't be null"),
            status: status.into(),
            writer_failure: Some(writer_failure.into()).filter(|_| writer_failure != S_OK),
            application_return_code: Some(application_return_code)
                .filter(|_| get_application_return_code),
            application_message: application_message.unwrap(),
        })
    }
    /// Initiates a LUN resynchronization operation. This method is supported
    /// only on Windows server operating systems.
    #[doc(alias = "RecoverSet")]
    pub fn recover_set(
        &self,
        flags: RawBitFlags<RecoveryOptions>,
    ) -> IVssAsyncResult<RecoverSetError> {
        let mut task = null_mut::<vss::IVssAsync>();
        check_com(unsafe { self.0.RecoverSet(flags.raw(), &mut task) })?;
        Ok(VssAsync::new(unsafe { SafeCOMComponent::new(task) }))
    }
}

/// Information returned by the [`IBackupComponentsEx3::get_writer_status_ex`]
/// method.
#[derive(Clone)]
pub struct GetWriterStatusExInfo {
    pub instance_id: VSS_ID,
    pub writer_id: VSS_ID,
    pub writer: BString,
    pub status: WriterState,
    pub writer_failure: Option<WriterFailureExError>,
    pub application_return_code: Option<HRESULT>,
    pub application_message: Option<BString>,
}

////////////////////////////////////////////////////////////////////////////////
// IVssBackupComponentsEx4
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssBackupComponentsEx4);

#[doc(alias = "IVssBackupComponentsEx4")]
#[derive(Debug, Clone)]
pub struct BackupComponentsEx4(SafeCOMComponent<vsbackup::IVssBackupComponentsEx4>);
impl_query_interface!(BackupComponentsEx4 => vsbackup::IVssBackupComponentsEx4);
transparent_wrapper!(
    #[doc(alias = "IVssBackupComponentsEx4")]
    pub struct IBackupComponentsEx4(vsbackup::IVssBackupComponentsEx4);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(BackupComponentsEx4 => IBackupComponentsEx4 => IBackupComponentsEx3);

impl IBackupComponentsEx4 {
    /// Normalizes a local volume path or UNC share path so that it can be passed
    /// to the `IVssBackupComponents::AddToSnapshotSet` method.
    #[doc(alias = "GetRootAndLogicalPrefixPaths")]
    pub fn get_root_and_logical_prefix_paths(
        &self,
        file_path: &U16CStr,
        normalize_fqdn_for_root_path: bool,
    ) -> Result<GetRootAndLogicalPrefixPathsInfo, GetRootAndLogicalPrefixPathsError> {
        let mut root_path: VSS_PWSZ = null_mut();
        let mut logical_prefix: VSS_PWSZ = null_mut();
        let result = check_com(unsafe {
            self.0.GetRootAndLogicalPrefixPaths(
                file_path.as_ptr() as *mut _,
                &mut root_path,
                &mut logical_prefix,
                if normalize_fqdn_for_root_path {
                    TRUE
                } else {
                    FALSE
                },
            )
        });

        // TODO: this method `GetRootAndLogicalPrefixPaths` returns two strings
        // but the ownership over those strings aren't specified clearly.
        // Alternatives:
        // - They are borrowed from somewhere else.
        //      - Use lifetimes to enforce the correct usage.
        //      - Make the method unsafe and let the caller handle it.
        //      - Clone the interface that owns them.
        // - The caller gets ownership so free them, but how?
        //      - Maybe via the `CoTaskMemFree` method?
        //          - CoTaskMemFree: https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cotaskmemfree
        //          - Used by the `VSS_OBJECT_PROP` struct to free its strings (which are
        //            of the same type): https://docs.microsoft.com/en-us/windows/win32/api/vss/ns-vss-vss_object_prop
        //              - See the VSS_SNAPSHOT_PROP struct: https://docs.microsoft.com/en-us/windows/win32/api/vss/ns-vss-vss_snapshot_prop
        //              - See the VSS_PROVIDER_PROP struct: https://docs.microsoft.com/en-us/windows/win32/api/vss/ns-vss-vss_provider_prop
        //          - Used to free string returned from `IVssBackupComponents::ExposeSnapshot`.
        //              - See remarks at: https://docs.microsoft.com/en-us/windows/win32/api/vsbackup/nf-vsbackup-ivssbackupcomponents-exposesnapshot
        // -->      - We assume this!

        let root_path = unsafe { VssU16CString::from_nullable_ptr(root_path) };
        let logical_prefix = unsafe { VssU16CString::from_nullable_ptr(logical_prefix) };

        result?;
        Ok(GetRootAndLogicalPrefixPathsInfo {
            root_path: root_path
                .expect("GetRootAndLogicalPrefixPaths returned a root path that was null"),
            logical_prefix: logical_prefix
                .expect("GetRootAndLogicalPrefixPaths returned a logical prefix that was null"),
        })
    }
}

/// Info returned by the [`IBackupComponentsEx4::get_root_and_logical_prefix_paths`]
/// method.
pub struct GetRootAndLogicalPrefixPathsInfo {
    pub root_path: VssU16CString,
    pub logical_prefix: VssU16CString,
}

////////////////////////////////////////////////////////////////////////////////
// IVssExamineWriterMetadata
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssExamineWriterMetadata);

#[doc(alias = "IVssExamineWriterMetadata")]
#[derive(Debug, Clone)]
pub struct ExamineWriterMetadata(SafeCOMComponent<vsbackup::IVssExamineWriterMetadata>);
impl_query_interface!(ExamineWriterMetadata => vsbackup::IVssExamineWriterMetadata);
transparent_wrapper!(
    #[doc(alias = "IVssExamineWriterMetadata")]
    pub struct IExamineWriterMetadata(vsbackup::IVssExamineWriterMetadata);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(ExamineWriterMetadata => IExamineWriterMetadata);

impl ExamineWriterMetadata {
    /// Initialize an [`ExamineWriterMetadata`] interface with an XML string
    /// containing a Writer Metadata Document.
    ///
    /// Remarks
    ///
    /// To save a copy of a writer’s Writer Metadata Document into an XML string
    /// to pass in as a parameter, use the [`IExamineWriterMetadata::save_as_xml`]
    /// method.
    ///
    /// To retrieve the latest version of a writer’s Writer Metadata Document,
    /// use the [`IBackupComponents::get_writer_metadata`] method.
    ///
    /// To load a writer metadata document into an existing [`ExamineWriterMetadata`]
    /// object, use the [`IExamineWriterMetadata::load_from_xml`]
    /// method.
    // TODO: is the `xml` argument supposed to remain valid even after this
    // method returns?
    #[doc(alias = "CreateVssExamineWriterMetadata")]
    pub fn new(xml: &BStr) -> Result<Self, CreateVssExamineWriterMetadataError> {
        unsafe {
            let mut comp = null_mut::<vsbackup::IVssExamineWriterMetadata>();
            check_com(vsbackup::CreateVssExamineWriterMetadata(
                xml.as_bstr(),
                &mut comp,
            ))?;
            Ok(Self(SafeCOMComponent::new(comp)))
        }
    }
}
impl IExamineWriterMetadata {
    /// Obtains a specific alternate location mapping of a file set.
    #[doc(alias = "GetAlternateLocationMapping")]
    pub fn get_alternate_location_mapping(
        &self,
        mapping_index: u32,
    ) -> Result<WMFileDescriptor, GetAlternateLocationMappingError> {
        let mut file_descriptor: *mut vswriter::IVssWMFiledesc = null_mut();
        check_com(unsafe {
            self.0
                .GetAlternateLocationMapping(mapping_index, &mut file_descriptor)
        })?;
        Ok(WMFileDescriptor(unsafe {
            SafeCOMComponent::new(file_descriptor)
        }))
    }
    /// Used by a requester to determine from the Writer Metadata Document the
    /// types of backup operations that a given writer can participate in.
    #[doc(alias = "GetBackupSchema")]
    pub fn get_backup_schema(&self) -> Result<RawBitFlags<BackupSchema>, GetBackupSchemaError> {
        let mut schema_mask: DWORD = Default::default();
        check_com(unsafe { self.0.GetBackupSchema(&mut schema_mask) })?;
        Ok(RawBitFlags::from_raw(schema_mask))
    }
    /// Obtains a Writer Metadata Document for a specified backup component.
    ///
    /// # Parameters
    ///
    /// ## component_index
    ///
    /// Index for a component. The value of this parameter is an integer from `0`
    /// to `n–1` inclusive, where `n` is the total number of components supported
    /// by a given writer. The value of `n` is returned by
    /// [`IExamineWriterMetadata::get_file_counts`] as the
    /// [`GetFileCountsInfo::total_components`] value.
    #[doc(alias = "GetComponent")]
    pub fn get_component(
        &self,
        component_index: u32,
    ) -> Result<WMComponent, ExamineWriterMetadataGetComponentError> {
        let mut component = null_mut::<vsbackup::IVssWMComponent>();
        check_com(unsafe { self.0.GetComponent(component_index, &mut component) })?;
        Ok(WMComponent(unsafe { SafeCOMComponent::new(component) }))
    }
    /// Obtains information about files that have been explicitly excluded from
    /// backup for a given writer.
    #[doc(alias = "GetExcludeFile")]
    pub fn get_exclude_file(
        &self,
        file_index: u32,
    ) -> Result<WMFileDescriptor, GetExcludeFileError> {
        let mut file_descriptor: *mut vswriter::IVssWMFiledesc = null_mut();
        check_com(unsafe { self.0.GetExcludeFile(file_index, &mut file_descriptor) })?;
        Ok(WMFileDescriptor(unsafe {
            SafeCOMComponent::new(file_descriptor)
        }))
    }
    /// Obtains excluded files and the number of components that a writer manages.
    #[doc(alias = "GetFileCounts")]
    pub fn get_file_counts(&self) -> Result<GetFileCountsInfo, GetFileCountsError> {
        let mut included_files: u32 = 0;
        let mut info = GetFileCountsInfo {
            excluded_files: 0,
            total_components: 0,
        };
        check_com(unsafe {
            self.0.GetFileCounts(
                &mut included_files,
                &mut info.excluded_files,
                &mut info.total_components,
            )
        })?;
        Ok(info)
    }
    /// Obtains basic information about a specific writer instance.
    #[doc(alias = "GetIdentity")]
    pub fn get_identity(&self) -> Result<GetIdentityInfo, GetIdentityError> {
        let mut instance: VSS_ID = Default::default();
        let mut writer: VSS_ID = Default::default();
        let mut writer_name: BSTR = null_mut();
        let mut usage: vswriter::VSS_USAGE_TYPE = Default::default();
        let mut source: vswriter::VSS_SOURCE_TYPE = Default::default();
        let result = check_com(unsafe {
            self.0.GetIdentity(
                &mut instance,
                &mut writer,
                &mut writer_name,
                &mut usage,
                &mut source,
            )
        });
        let writer_name = unsafe { take_ownership_of_bstr(writer_name) };
        result?;
        Ok(GetIdentityInfo {
            instance,
            writer,
            writer_name: writer_name
                .unwrap()
                .expect("identity's writer_name shouldn't be null"),
            usage: usage.into(),
            source: source.into(),
        })
    }
    /// Returns information about how a writer wants its data to be restored.
    ///
    /// Returns `None` if a restore method does not exist.
    #[doc(alias = "GetRestoreMethod")]
    pub fn get_restore_method(
        &self,
    ) -> Result<Option<GetRestoreMethodInfo>, GetRestoreMethodError> {
        let mut method: vswriter::VSS_RESTOREMETHOD_ENUM = Default::default();
        let mut service: BSTR = null_mut();
        let mut user_procedure: BSTR = null_mut();
        let mut writer_restore: vswriter::VSS_WRITERRESTORE_ENUM = Default::default();
        let mut reboot_required: bool = Default::default();
        let mut mappings_count: u32 = Default::default();
        let hr = unsafe {
            self.0.GetRestoreMethod(
                &mut method,
                &mut service,
                &mut user_procedure,
                &mut writer_restore,
                &mut reboot_required,
                &mut mappings_count,
            )
        };
        let service = unsafe { take_ownership_of_bstr(service) };
        let user_procedure = unsafe { take_ownership_of_bstr(user_procedure) };
        if hr == S_FALSE {
            return Ok(None);
        }
        check_com(hr)?;
        Ok(Some(GetRestoreMethodInfo {
            method: method.into(),
            service: service.unwrap(),
            user_procedure: user_procedure.unwrap(),
            writer_restore: writer_restore.into(),
            reboot_required,
            mappings_count,
        }))
    }
    /// Loads an XML document that contains a writer's metadata.
    ///
    /// This method is used at restore time to load writer metadata that was saved
    /// by [`IExamineWriterMetadata::save_as_xml`] at the time of the backup
    /// operation.
    // TODO: is the `xml` argument supposed to remain valid even after this
    // method returns?
    #[doc(alias = "LoadFromXML")]
    pub fn load_from_xml(&self, xml: &BStr) -> Result<(), LoadFromXMLError> {
        // TODO: update `winapi` to have the correct signature.

        // The Rust bindings seem to have the wrong type here, they expect
        // `*mut *mut OLECHAR` but the official docs says that it should be only
        // `BSTR = *mut OLECHAR`. Since both expect a pointer it should be find
        // to give it just cast our pointer to what they want.

        check_com(unsafe { self.0.LoadFromXML(xml.as_bstr() as *mut _) })?;
        Ok(())
    }
    /// Saves the Writer Metadata Document that contains a writer's state
    /// information to a string. This string can be saved as part of a backup
    /// operation.
    #[doc(alias = "SaveAsXML")]
    pub fn save_as_xml(&self) -> Result<BString, ExamineWriterMetadataSaveAsXMLError> {
        let mut xml: BSTR = null_mut();
        let hr = unsafe { self.0.SaveAsXML(&mut xml) };
        let xml = unsafe { take_ownership_of_bstr(xml) };
        check_com(hr)?;
        Ok(xml
            .unwrap()
            .expect("the xml string returned by SaveAsXML shouldn't be null"))
    }
}

/// Info returned by the [`IExamineWriterMetadata::get_file_counts`] method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetFileCountsInfo {
    /// The number of file sets that are explicitly excluded from the backup.
    pub excluded_files: u32,
    /// The total number of components that are managed by the current writer.
    pub total_components: u32,
}

/// Info returned by the [`IExamineWriterMetadata::get_identity`] method.
pub struct GetIdentityInfo {
    /// Globally unique identifier (GUID) of the writer instance.
    pub instance: VSS_ID,
    /// Class identifier (GUID) of the writer class.
    pub writer: VSS_ID,
    /// String specifying the name of the writer.
    pub writer_name: BString,
    /// Indicates how the data managed by the writer is used on the host system.
    pub usage: UsageType,
    /// Indicates the type of data managed by the writer.
    pub source: SourceType,
}

/// Info returned by the [`IExamineWriterMetadata::get_restore_method`] method.
pub struct GetRestoreMethodInfo {
    /// Specifies file overwriting, the use of alternate locations specifying
    /// the method that will be used in the restore operation.
    pub method: RestoreMethod,
    /// If the value of [`method`](Self::method) is [`StopRestoreStart`] or
    /// [`RestoreStopStart`] then a string containing the name of the service
    /// that is started and stopped. Otherwise, the field will be `None`.
    ///
    /// [`StopRestoreStart`]: RestoreMethod::StopRestoreStart
    /// [`RestoreStopStart`]: RestoreMethod::RestoreStopStart
    pub service: Option<BString>,
    /// the URL of an HTML or XML document describing to the user how the restore
    /// is to be performed.
    pub user_procedure: Option<BString>,
    /// Specifies whether the writer will be involved in restoring its data.
    pub writer_restore: WriterRestore,
    /// Indicates whether a reboot will be required after the restore operation
    /// is complete. The field will be `true` if a reboot will be required, or
    /// `false` otherwise.
    pub reboot_required: bool,
    /// The number of alternate mappings associated with the writer.
    pub mappings_count: u32,
}

////////////////////////////////////////////////////////////////////////////////
// IVssExamineWriterMetadataEx
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssExamineWriterMetadataEx);

#[doc(alias = "IVssExamineWriterMetadataEx")]
#[derive(Debug, Clone)]
pub struct ExamineWriterMetadataEx(SafeCOMComponent<vsbackup::IVssExamineWriterMetadataEx>);
impl_query_interface!(ExamineWriterMetadataEx => vsbackup::IVssExamineWriterMetadataEx);
transparent_wrapper!(
    #[doc(alias = "IVssExamineWriterMetadataEx")]
    pub struct IExamineWriterMetadataEx(vsbackup::IVssExamineWriterMetadataEx);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(ExamineWriterMetadataEx => IExamineWriterMetadataEx => IExamineWriterMetadata);

impl IExamineWriterMetadataEx {
    /// Obtains the writer instance name and other basic information about a
    /// specific writer instance.
    #[doc(alias = "GetIdentityEx")]
    pub fn get_identity_ex(&self) -> Result<GetIdentityExInfo, GetIdentityExError> {
        let mut instance: VSS_ID = Default::default();
        let mut writer: VSS_ID = Default::default();
        let mut writer_name: BSTR = null_mut();
        let mut instance_name: BSTR = null_mut();
        let mut usage: vswriter::VSS_USAGE_TYPE = Default::default();
        let mut source: vswriter::VSS_SOURCE_TYPE = Default::default();
        let hr = unsafe {
            self.0.GetIdentityEx(
                &mut instance,
                &mut writer,
                &mut writer_name,
                &mut instance_name,
                &mut usage,
                &mut source,
            )
        };
        let writer_name = unsafe { take_ownership_of_bstr(writer_name) };
        let instance_name = unsafe { take_ownership_of_bstr(instance_name) };
        check_com(hr)?;
        Ok(GetIdentityExInfo {
            instance,
            writer,
            writer_name: writer_name
                .unwrap()
                .expect("the writer name returned by GetIdentityEx shouldn't be null"),
            instance_name: instance_name
                .unwrap()
                .expect("the instance name returned by GetIdentityEx shouldn't be null"),
            usage: usage.into(),
            source: source.into(),
        })
    }
}

/// Info returned by the [`IExamineWriterMetadataEx::get_identity_ex`] method.
pub struct GetIdentityExInfo {
    /// Globally unique identifier (GUID) of the writer instance.
    pub instance: VSS_ID,
    /// GUID of the writer class.
    pub writer: VSS_ID,
    /// String specifying the name of the writer.
    pub writer_name: BString,
    /// String specifying the writer instance name.
    pub instance_name: BString,
    /// Indicates how the data managed by the writer is used on the host system.
    pub usage: UsageType,
    /// Indicates the type of data managed by the writer.
    pub source: SourceType,
}

////////////////////////////////////////////////////////////////////////////////
// IVssExamineWriterMetadataEx2
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssExamineWriterMetadataEx2);

#[doc(alias = "IVssExamineWriterMetadataEx2")]
#[derive(Debug, Clone)]
pub struct ExamineWriterMetadataEx2(SafeCOMComponent<vsbackup::IVssExamineWriterMetadataEx2>);
impl_query_interface!(ExamineWriterMetadataEx2 => vsbackup::IVssExamineWriterMetadataEx2);
transparent_wrapper!(
    #[doc(alias = "IVssExamineWriterMetadataEx2")]
    pub struct IExamineWriterMetadataEx2(vsbackup::IVssExamineWriterMetadataEx2);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(ExamineWriterMetadataEx2 => IExamineWriterMetadataEx2 => IExamineWriterMetadataEx);

impl IExamineWriterMetadataEx2 {
    /// Obtains the number of file sets that have been explicitly excluded from
    /// a given shadow copy.
    #[doc(alias = "GetExcludeFromSnapshotCount")]
    pub fn get_exclude_from_snapshot_count(&self) -> Result<u32, GetExcludeFromSnapshotCountError> {
        let mut excluded_from_snapshot: UINT = 0;
        check_com(unsafe {
            self.0
                .GetExcludeFromSnapshotCount(&mut excluded_from_snapshot)
        })?;
        Ok(excluded_from_snapshot)
    }
    /// Obtains information about file sets that have been explicitly excluded
    /// from a given shadow copy.
    #[doc(alias = "GetExcludeFromSnapshotFile")]
    pub fn get_exclude_from_snapshot_file(
        &self,
        file_index: u32,
    ) -> Result<WMFileDescriptor, GetExcludeFromSnapshotFileError> {
        let mut file_description: *mut vswriter::IVssWMFiledesc = null_mut();
        check_com(unsafe {
            self.0
                .GetExcludeFromSnapshotFile(file_index, &mut file_description)
        })?;
        Ok(WMFileDescriptor(unsafe {
            SafeCOMComponent::new(file_description)
        }))
    }
    /// Obtains the version information for a writer application.
    #[doc(alias = "GetVersion")]
    pub fn get_version(&self) -> Result<VersionInfo, GetVersionError> {
        let mut info = VersionInfo { major: 0, minor: 0 };
        check_com(unsafe { self.0.GetVersion(&mut info.major, &mut info.minor) })?;
        Ok(info)
    }
}

/// Version information for a writer application. Returned by the
/// [`IExamineWriterMetadataEx2::get_version`] method.
pub struct VersionInfo {
    /// The major version of a writer application.
    pub major: u32,
    /// The minor version of a writer application.
    pub minor: u32,
}

////////////////////////////////////////////////////////////////////////////////
// IVssWMComponent
////////////////////////////////////////////////////////////////////////////////

// Safety: The type implements `Interface` correctly.
unsafe_impl_as_IUnknown!(vsbackup::IVssWMComponent);

/// Allows access to component information stored in a Writer Metadata Document.
///
/// Instances of IVssWMComponent are obtained by calling
/// [`IExamineWriterMetadata::get_component`].
#[doc(alias = "IVssWMComponent")]
#[derive(Debug, Clone)]
pub struct WMComponent(SafeCOMComponent<vsbackup::IVssWMComponent>);
impl_query_interface!(WMComponent => vsbackup::IVssWMComponent);

impl WMComponent {
    /// Obtains basic information about the specified writer metadata component.
    #[doc(alias = "GetComponentInfo")]
    pub fn get_component_info(&self) -> Result<ComponentInfo<'_>, GetComponentInfoError> {
        let mut info = null::<vsbackup::VSS_COMPONENTINFO>();
        check_com(unsafe { self.0.GetComponentInfo(&mut info) })?;
        Ok(ComponentInfo {
            owner: Some(Cow::Borrowed(self)),
            info,
        })
    }
    /// Obtains a [`WMFileDescriptor`] object containing information about the
    /// specified database backup component file.
    ///
    /// `db_file_index` is an index between `0` and `n-1`, where `n` is the number
    /// of database files as specified by the [`ComponentInfo::databases`]
    /// method.
    #[doc(alias = "GetDatabaseFile")]
    pub fn get_database_file(
        &self,
        db_file_index: u32,
    ) -> Result<WMFileDescriptor, GetDatabaseFileError> {
        let mut file_desc: *mut vswriter::IVssWMFiledesc = null_mut();
        check_com(unsafe { self.0.GetDatabaseFile(db_file_index, &mut file_desc) })?;
        Ok(WMFileDescriptor(unsafe {
            SafeCOMComponent::new(file_desc)
        }))
    }
    /// Obtains a file descriptor for the log file associated with the specified
    /// database backup component.
    ///
    /// `db_log_file_index` is an index between `0` and `n-1`, where `n` is the
    /// number of database log files as specified by the
    /// [`ComponentInfo::log_files`] method.
    #[doc(alias = "GetDatabaseLogFile")]
    pub fn get_database_log_file(
        &self,
        db_log_file_index: u32,
    ) -> Result<WMFileDescriptor, GetDatabaseLogFileError> {
        let mut file_desc: *mut vswriter::IVssWMFiledesc = null_mut();
        check_com(unsafe { self.0.GetDatabaseLogFile(db_log_file_index, &mut file_desc) })?;
        Ok(WMFileDescriptor(unsafe {
            SafeCOMComponent::new(file_desc)
        }))
    }
    /// Returns an instance of the `IVssWMDependency` interface containing accessors
    /// for obtaining information about explicit writer-component dependencies of
    /// one of the current components.
    ///
    /// `dependency_index` is an index between `0` and `n-1`, where `n` is the
    /// number of dependencies associated with this component as specified
    /// by the `ComponentInfo::dependencies` method.
    #[doc(alias = "GetDependency")]
    pub fn get_dependency(
        &self,
        dependency_index: u32,
    ) -> Result<WMDependency, GetDependencyError> {
        let mut dependency: *mut vswriter::IVssWMDependency = null_mut();
        check_com(unsafe { self.0.GetDependency(dependency_index, &mut dependency) })?;
        Ok(WMDependency(unsafe { SafeCOMComponent::new(dependency) }))
    }
    /// Obtains a file descriptor associated with a file group.
    ///
    /// `file_index` is an index between `0` and `n-1`, where `n` is the number
    /// of files in the file group as specified by the [`ComponentInfo::file_count`]
    /// method.
    #[doc(alias = "GetFile")]
    pub fn get_file(&self, file_index: u32) -> Result<WMFileDescriptor, GetFileError> {
        let mut file_desc: *mut vswriter::IVssWMFiledesc = null_mut();
        check_com(unsafe { self.0.GetFile(file_index, &mut file_desc) })?;
        Ok(WMFileDescriptor(unsafe {
            SafeCOMComponent::new(file_desc)
        }))
    }
}

////////////////////////////////////////////////////////////////////////////////
// VSS_COMPONENTINFO
////////////////////////////////////////////////////////////////////////////////

/// Contains information about a given component, and is returned to requesters
/// by the [`WMComponent`] interface by the [`WMComponent::get_component_info`]
/// method.
///
/// # Remarks
///
/// To obtain `VssComponentInfo` object for a given component, a requester must
/// first obtain the corresponding [`WMComponent`] object through a call to
/// [`IExamineWriterMetadata::get_component`]. A call to
/// [`WMComponent::get_component_info`] then allocates and returns a
/// `VSS_COMPONENTINFO` structure that this struct wraps.
#[doc(alias = "VSS_COMPONENTINFO")]
pub struct ComponentInfo<'a> {
    owner: Option<Cow<'a, WMComponent>>,
    info: PVSSCOMPONENTINFO,
}
/// Getters for the fields of the wrapped `VSS_COMPONENTINFO` struct.
impl<'a> ComponentInfo<'a> {
    /// Component type. See [`VssComponentType`].
    #[doc(alias = "type")]
    pub fn component_type(&self) -> VssComponentType {
        self.as_raw().type_.into()
    }
    /// A string containing the logical path of the component.
    ///
    /// There are no restrictions on the characters that can appear in a logical
    /// path.
    #[doc(alias = "bstrLogicalPath")]
    pub fn logical_path(&self) -> Option<&BStr> {
        let pointer = &self.as_raw().bstrLogicalPath;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { BStr::from_bstr(pointer) }.unwrap())
        }
    }
    /// A string containing the name of the component.
    #[doc(alias = "bstrComponentName")]
    pub fn component_name(&self) -> &BStr {
        unsafe { BStr::from_bstr(&self.as_raw().bstrComponentName) }.unwrap()
    }
    /// A string containing the description of the component.
    #[doc(alias = "bstrCaption")]
    pub fn caption(&self) -> Option<&BStr> {
        let pointer = &self.as_raw().bstrCaption;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { BStr::from_bstr(pointer) }.unwrap())
        }
    }
    /// A buffer containing the binary data for a displayable icon representing
    /// the component. The buffer contents should use the same format as the
    /// standard icon (.ico) files.
    ///
    /// If the writer that created the component did not choose to specify an icon
    /// then this will return `None`.
    #[doc(alias = "pbIcon")]
    #[doc(alias = "cbIcon")]
    pub fn icon(&self) -> Option<&[u8]> {
        let raw = self.as_raw();
        if raw.pbIcon.is_null() {
            return None;
        }
        Some(unsafe { slice::from_raw_parts(raw.pbIcon, usize::try_from(raw.cbIcon).unwrap()) })
    }
    /// Boolean that indicates whether there is private metadata associated with
    /// the restoration of the component. The Boolean is `true` if there is
    /// metadata and `false` if there is not.
    ///
    /// A writer indicates whether a component supports private metadata by
    /// setting this value when a component is added with
    /// `IVssCreateWriterMetadata::AddComponent`. Writers later add restore
    /// metadata with `IVssComponent::SetRestoreMetadata`. Requesters retrieve
    /// the information using `IVssComponent::GetRestoreMetadata`.
    #[doc(alias = "bRestoreMetadata")]
    pub fn restore_metadata(&self) -> bool {
        self.as_raw().bRestoreMetadata
    }
    /// Boolean that indicates (for component mode operations) if the component
    /// is selectable for backup. The value of `selectable` helps determine whether
    /// a requester has the option of including or excluding a given component
    /// in backup operations. The Boolean is `true` if the component is selectable
    /// for backup and `false` if it is not.
    ///
    /// There is no default value for a component's selectability for backup.
    /// A writer must always explicitly set the value when it adds the component
    /// to its Writer Metadata Document using `IVssCreateWriterMetadata::AddComponent`.
    ///
    /// In addition, the value of `selectable`, the component's logical path, and
    /// the component's relationship to other components as expressed in that path
    /// determine when and how a component is included in a backup operation:
    ///
    /// - For a nonselectable for backup component (`selectable` is `false`) with
    ///   no selectable for backup ancestors in the hierarchy of its logical path,
    ///   inclusion in the backup set is always mandatory and always implicit.
    ///   A requester explicitly adds the component to the backup set in the Backup
    ///   Components Document with [`IBackupComponents::add_component`].
    ///
    /// - For a selectable for backup component (`selectable` is `true`) with no
    ///   selectable for backup ancestor in the hierarchy of its logical paths,
    ///   inclusion in the backup set is always optional and always explicit.
    ///   A requester explicitly adds the component to the backup set in the
    ///   Backup Components Document with [`IBackupComponents::add_component`].
    ///
    ///   If such a component is included as an ancestor in the logical path of
    ///   other components, both those that are selectable for backup and those
    ///   that are not, it defines a component set containing these other components
    ///   as subcomponents. If a selectable for backup component is explicitly
    ///   included in a backup, these subcomponents are implicitly included in
    ///   the backup.
    ///
    /// - For a nonselectable for backup component (`selectable` is `false`) that
    ///   has a selectable for backup ancestor in the hierarchy of its logical
    ///   paths (and are therefore part of a component set defined by that
    ///   ancestor), inclusion in the backup set is always implicit and contingent
    ///   on the inclusion of a selectable for backup ancestor. A requester never
    ///   explicitly adds the component to the backup set in the Backup Components
    ///   Document; instead, it adds the selectable for backup ancestor to the
    ///   document using [`IBackupComponents::add_component`].
    ///
    /// - For a selectable for backup component (`selectable` is `true`) that has
    ///   a selectable for backup ancestor in the hierarchy of its logical paths
    ///   (and is therefore part of a component set defined by that ancestor),
    ///   inclusion in the backup set can be either optional and explicit, or if
    ///   the component is not explicitly selected, its inclusion may be implicit
    ///   and contingent on the inclusion of a selectable for backup ancestor.
    ///   If the inclusion of the component is explicit, a requester explicitly
    ///   adds the components to the backup set in the Backup Components Document
    ///   with [`IBackupComponents::add_component`].
    ///
    ///   If the inclusion is implicit, a requester does not add these components
    ///   to a backup set in the Backup Components Document.
    ///
    ///   If the inclusion of the component is explicit and the component defines
    ///   a component set, the members of that component set are implicitly selected.
    ///
    ///   A writer sets a component's selectability for backup (bSelectable) when
    ///   adding the component to the Writer Metadata Document by using
    ///   `IVssCreateWriterMetadata::AddComponent`.
    ///
    ///   See [Working with Selectability and Logical Paths](https://docs.microsoft.com/en-us/windows/desktop/VSS/working-with-selectability-and-logical-paths)
    ///   for more information.
    #[doc(alias = "bSelectable")]
    pub fn selectable(&self) -> bool {
        self.as_raw().bSelectable
    }
    /// Boolean that indicates (for component-mode operations) whether the
    /// component is selectable for restore. `selectable_for_restore` allows the
    /// requester to determine whether this component can be individually selected
    /// for restore if it had earlier been
    /// [implicitly included](https://docs.microsoft.com/en-us/windows/desktop/VSS/vssgloss-i)
    /// in the backup. The Boolean is `true` if the component is selectable for
    /// restore and `false` if it is not.
    ///
    /// By default, a component's selectability for restore is `false`. A writer
    /// can override this default when it adds the component to its Writer Metadata
    /// Document using `IVssCreateWriterMetadata::AddComponent`.
    ///
    /// If a component is explicitly added to the backup document (see
    /// [explicit component inclusion](https://docs.microsoft.com/en-us/windows/win32/vss/vssgloss-e)),
    /// then it can always be individually selected for restore; so this flag
    /// then has no meaning. If a component is implicitly added to the backup
    /// document, then the bSelectableForRestore flag determines whether the
    /// component can be individually restored using
    /// [`IBackupComponents::add_restore_subcomponent`].
    ///
    /// See [Working with Selectability and Logical Paths](https://docs.microsoft.com/en-us/windows/desktop/VSS/working-with-selectability-and-logical-paths)
    /// for more information.
    #[doc(alias = "bSelectableForRestore")]
    pub fn selectable_for_restore(&self) -> bool {
        self.as_raw().bSelectableForRestore
    }
    /// A bit mask (or bitwise OR) of values of the [`VssComponentFlags`]
    /// enumeration, indicating the features this component supports.
    ///
    /// Windows Server 2003 and Windows XP:  Before Windows Server 2003 with SP1,
    /// this member is reserved for system use.
    #[doc(alias = "dwComponentFlags")]
    pub fn component_flags(&self) -> RawBitFlags<VssComponentFlags> {
        RawBitFlags::from_raw(self.as_raw().dwComponentFlags)
    }
    /// If the component is a file group, the number of file descriptors for files
    /// in the group. Otherwise, this value is zero.
    #[doc(alias = "cFileCount")]
    pub fn file_count(&self) -> u32 {
        self.as_raw().cFileCount
    }
    /// If the component is a database, the number of database file descriptors.
    /// Otherwise, this value is zero.
    #[doc(alias = "cDatabases")]
    pub fn databases(&self) -> u32 {
        self.as_raw().cDatabases
    }
    /// If the component is a database, the number of database log file
    /// descriptors. Otherwise, the value of this parameter is zero.
    #[doc(alias = "cLogFiles")]
    pub fn log_files(&self) -> u32 {
        self.as_raw().cLogFiles
    }
    /// The number of explicit writer-component dependencies of the current component.
    /// This value is incremented when `IVssCreateWriterMetadata::AddComponentDependency`
    /// is called by a writer.
    #[doc(alias = "cDependencies")]
    pub fn dependencies(&self) -> u32 {
        self.as_raw().cDependencies
    }
}
impl<'a> ComponentInfo<'a> {
    /// Get access to the inner "raw" FFI type with all component information.
    #[doc(alias = "VSS_COMPONENTINFO")]
    pub fn as_raw(&self) -> &VSS_COMPONENTINFO {
        unsafe { &*self.info }
    }
    /// Clone the "owner" [`WMComponent`] reference so that this struct gets a
    /// `'static` lifetime.
    pub fn into_owned(self) -> ComponentInfo<'static> {
        let mut this = ManuallyDrop::new(self);
        ComponentInfo {
            owner: Some(Cow::Owned(this.owner.take().unwrap().into_owned())),
            info: this.info,
        }
    }
    /// The [`WMComponent`] that will be used to free the allocated
    /// `VSS_COMPONENTINFO` struct that this struct wraps.
    pub fn owner(&self) -> &WMComponent {
        self.owner.as_ref().unwrap()
    }
}
impl<'a> Drop for ComponentInfo<'a> {
    #[doc(alias = "FreeComponentInfo")]
    fn drop(&mut self) {
        if let Some(owner) = &self.owner {
            unsafe { owner.0.FreeComponentInfo(self.info) };
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// IVssWriterComponentsExt
////////////////////////////////////////////////////////////////////////////////

// Safety: The type's lifecycle is managed by `IUnknown`.
#[allow(clippy::needless_borrow)]
unsafe impl CustomIUnknown for vsbackup::IVssWriterComponentsExt {
    unsafe fn query_interface(&self, riid: REFIID, object: *mut *mut c_void) -> HRESULT {
        // Safety: reading from the vtable should be fine. The vtable should
        // just expect the current object as its first argument.
        ((&*self.lpVtbl).parent2.QueryInterface)(self as *const Self as *mut _, riid, object)
    }
    unsafe fn add_ref(&self) -> ULONG {
        // Safety: reading from the vtable should be fine. The vtable should
        // just expect the current object as its first argument.
        ((&*self.lpVtbl).parent2.AddRef)(self as *const Self as *mut _)
    }
    unsafe fn release(&self) -> ULONG {
        // Safety: reading from the vtable should be fine. The vtable should
        // just expect the current object as its first argument.
        ((&*self.lpVtbl).parent2.Release)(self as *const Self as *mut _)
    }
}

#[doc(alias = "IVssWriterComponentsExt")]
#[derive(Debug, Clone)]
pub struct WriterComponentsExt(SafeCOMComponent<vsbackup::IVssWriterComponentsExt>);
impl_query_interface!(WriterComponentsExt);
transparent_wrapper!(
    #[doc(alias = "IVssWriterComponentsExt")]
    pub struct IWriterComponentsExt(vsbackup::IVssWriterComponentsExt);
);
// Safety: all wrappers ensure their wrapped values are valid to use (Not released).
unsafe_deref_to_ref!(WriterComponentsExt => IWriterComponentsExt);

// Need custom deref into the `IWriterComponents` interface since this struct
// has multiple parent interfaces (so `winapi` doesn't implement Deref for us).
impl Deref for IWriterComponentsExt {
    type Target = IWriterComponents;

    fn deref(&self) -> &Self::Target {
        // Safety: `IVssWriterComponentsExt` implements the `IVssWriterComponents`
        // interface and stores the `IVssWriterComponents` vtable inlined in the
        // beginning of its vtable. Since it is safe to ignore the end of a type's
        // layout it should be safe to handle it as an `IVssWriterComponents`.
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}
impl_as_ref_and_borrow!(IWriterComponentsExt => IWriterComponents);

////////////////////////////////////////////////////////////////////////////////
// Freestanding functions
////////////////////////////////////////////////////////////////////////////////

/// Information about whether any shadow copies exist for a specified volume
///
/// Returned by the [`is_volume_snapshotted`] function.
#[doc(alias = "IsVolumeSnapshotted")]
pub struct VolumeSnapshottedInfo {
    /// The value of this parameter is `true` if the volume has a shadow copy,
    /// and `false` if the volume does not have a shadow copy.
    pub snapshot_present: bool,
    /// Indicates which volume control or file I/O operations are disabled for
    /// the volume that has been shadow copied.
    pub snapshot_capability: RawBitFlags<SnapshotCapability>,
}

/// The IsVolumeSnapshotted function determines whether any shadow copies exist
/// for the specified volume.
///
/// # Parameters
///
/// ## volume_name
///
/// Name of the volume. The name of the volume to be checked must be in one of the
/// following formats and must include a trailing backslash (`\`):
///
/// - The path of a mounted folder, for example, `Y:\MountX\`
/// - A drive letter, for example, `D:\`
/// - A volume GUID path of the form `\\?\Volume{GUID}\` (where GUID identifies
///   the volume)
#[doc(alias = "IsVolumeSnapshotted")]
pub fn is_volume_snapshotted(
    volume_name: &U16CStr,
) -> Result<VolumeSnapshottedInfo, IsVolumeSnapshottedError> {
    let mut snapshot_present: BOOL = FALSE;
    let mut snapshot_capability: LONG = 0;
    check_com(unsafe {
        vsbackup::IsVolumeSnapshotted(
            volume_name.as_ptr() as *mut _,
            &mut snapshot_present,
            &mut snapshot_capability,
        )
    })?;
    Ok(VolumeSnapshottedInfo {
        snapshot_present: snapshot_present == TRUE,
        snapshot_capability: RawBitFlags::from_raw(snapshot_capability),
    })
}

/// Checks the registry for writers that should block revert operations on the
/// specified volume.
///
/// Returns `true` if the volume contains components from any writers that are
/// listed in the registry as writers that should block revert operations, or
/// `false` otherwise.
///
/// # Parameters
///
/// ## volume_name
///
/// The name of the volume. This name must be in one of the following formats
/// and must include a trailing backslash (`\`):
///
/// - The path of a mounted folder, for example, `Y:\MountX\`
/// - A drive letter, for example, `D:\`
/// - A volume GUID path of the form `\\?\Volume{GUID}\` (where GUID identifies
///   the volume)
///
/// # Remarks
///
/// The list of writers that should block revert operations is stored in the
/// registry under the following key:
///
/// `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\VSS\Settings\WritersBlockingRevert`
#[doc(alias = "ShouldBlockRevert")]
pub fn should_block_revert(volume_name: &U16CStr) -> Result<bool, ShouldBlockRevertError> {
    let mut block = false;
    check_com(unsafe { vsbackup::ShouldBlockRevert(volume_name.as_ptr(), &mut block) })?;
    Ok(block)
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn it_works() {
        BackupComponents::new().unwrap();
    }

    /// Check if AsRef and Borrow impls are good enough to write ergonomic generic
    /// code.
    #[allow(unused_variables, dead_code)]
    fn generic_borrow_works() {
        // Require the simples interface and try to use these when we have the
        // most complex interface.
        fn take_borrow(base: impl Borrow<IBackupComponents>) {}
        fn take_ref(base: impl AsRef<IBackupComponents>) {}

        // Works for owned types:
        fn get_ex4(comp: BackupComponentsEx4) {
            // Reference to owned type:
            take_borrow(&comp);
            take_ref(&comp);

            // Consume owned type:
            take_borrow(comp.clone());
            take_ref(comp);
        }
        // Works for reference types:
        fn get_ex4_ref(comp: &IBackupComponentsEx4) {
            take_borrow(comp);
            take_ref(comp);
        }
        fn get_ex3_ref(comp: &IBackupComponentsEx3) {
            take_borrow(comp);
            take_ref(comp);
        }
    }
}

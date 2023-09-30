# Errors

CreateVssBackupComponents

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameters is not valid.

    E_OUTOFMEMORY

        Out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

AbortBackup

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

AddAlternativeLocationMapping

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified component does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

AddNewTarget

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, or this method has been called during a restore operation.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The component does not exist or the path and file specification do not match a component and file specification in the component.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

AddRestoreSubcomponent

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has not been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The component does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

CreateVssExamineWriterMetadata

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameters is not valid.

    E_OUTOFMEMORY

        Out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document passed in the bstrXML parameter is not valid—that is, either it is not a correctly formed XML string or it does not match the schema.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

ExamineWriterMetadata SaveAsXML

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

LoadFromXML

    S_FALSE

        The XML document could not be loaded.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

IsVolumeSnapshotted

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameters is not valid.

    E_OUTOFMEMORY

        Out of memory or other system resources.

    VSS_E_PROVIDER_VETO

        Expected provider error. The provider logged the error in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified volume was not found.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        Unexpected provider error. The error code is logged in the event log file. For additional information, see Event and Error Handling Under VSS.

ShouldBlockRevert

    E_ACCESSDENIED

        The caller is not an administrator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetWriterMetadata

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

ExamineWriterMetadata GetComponent


    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified component does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetFileCounts

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetComponentInfo

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

InitializeForBackup

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetBackupState

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

Wait

    E_ACCESSDENIED

        The wait operation failed because the user did not have the correct privileges.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

QueryStatus

    E_ACCESSDENIED

        The query operation failed because the user did not have the correct privileges.

    E_INVALIDARG

        The pointer to the variable used to hold the pHrResult return value is NULL or is not a valid memory location.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

Cancel

    VSS_S_ASYNC_CANCELLED

        The asynchronous operation had been canceled prior to calling this method.

    VSS_S_ASYNC_FINISHED

        The asynchronous operation had completed prior to calling this method.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GatherWriterMetadata

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        ppAsync does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_WRITER_INFRASTRUCTURE

        The writer infrastructure is not operating properly. Check that the Event Service and VSS have been started, and check for errors associated with those services in the error log.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetContext

    E_INVALIDARG

        One of the parameter values is not valid.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

StartSnapshotSet

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_SNAPSHOT_SET_IN_PROGRESS

        The creation of a shadow copy is in progress, and only one shadow copy creation operation can be in progress at one time. Either wait to try again or return with a failure error code.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

AddToSnapshotSet

    E_ACCESSDENIED
    0x80070005L

        Caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG
    0x80070057L

        One of the parameter values is not valid.

    E_OUTOFMEMORY
    0x8007000EL

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE
    0x80042301L

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED
    0x80042312L

        The maximum number of volumes or remote file shares have been added to the shadow copy set. The specified volume or remote file share was not added to the shadow copy set.

    VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED
    0x80042317L

        The volume or remote file share has been added to the maximum number of shadow copy sets. The specified volume or remote file share was not added to the shadow copy set.

    VSS_E_NESTED_VOLUME_LIMIT
    0x8004232CL

        The specified volume is nested too deeply to participate in the VSS operation. Possible reasons for this error include the following:

        Trying to create a shadow copy of a volume that resides on a VHD that is contained in another VHD.
        Trying to create a shadow copy of a VHD volume when the volume that contains the VHD is also in the same shadow copy set.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This return code is not supported.

    VSS_E_OBJECT_NOT_FOUND
    0x80042308L

        pwszVolumeName does not correspond to an existing volume or remote file share.

    VSS_E_PROVIDER_NOT_REGISTERED
    0x80042304L

        ProviderId does not correspond to a registered provider.

    VSS_E_PROVIDER_VETO
    0x80042306L

        Expected provider error. The provider logged the error in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_SNAPSHOT_SET_IN_PROGRESS
    0x80042316L

        Another shadow copy creation is already in progress. Occurs when adding a CSV volume to a snapshot set from multiple nodes at the same time, or while adding a scale out share to the snapshot set from multiple SMB client nodes at the same time.

    VSS_E_VOLUME_NOT_SUPPORTED
    0x8004230CL

        The value of the ProviderId parameter is GUID_NULL and no VSS provider indicates that it supports the specified volume or remote file share.

    VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER
    0x8004230EL

        The volume or remote file share is not supported by the specified provider.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR
    0x8004230FL

        The provider returned an unexpected error code. This error code is only returned via the QueryStatus method on the IVssAsync interface returned in the ppAsync parameter.

PrepareForBackup

    E_INVALIDARG

        ppAsync does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GatherWriterStatus

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        ppAsync does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_WRITER_INFRASTRUCTURE

        The writer infrastructure is not operating properly. Check that the Event Service and VSS have been started, and check for errors associated with those services in the error log.

FreeWriterStatus

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

DoSnapshotSet

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        ppAsync does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object has not been initialized or the prerequisite calls for a given shadow copy context have not been made prior to calling DoSnapshotSet.

    VSS_E_INSUFFICIENT_STORAGE

        The system or provider has insufficient storage space. If possible delete any old or unnecessary persistent shadow copies and try again. This error code is only returned via the QueryStatus method on the IVssAsync interface returned in the ppAsync parameter.

    VSS_E_FLUSH_WRITES_TIMEOUT

        The system was unable to flush I/O writes. This can be a transient problem. It is recommended to wait ten minutes and try again, up to three times.

    VSS_E_HOLD_WRITES_TIMEOUT

        The system was unable to hold I/O writes. This can be a transient problem. It is recommended to wait ten minutes and try again, up to three times.

    VSS_E_NESTED_VOLUME_LIMIT

        The specified volume is nested too deeply to participate in the VSS operation.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This return code is not supported.

    VSS_E_PROVIDER_VETO

        The provider was unable to perform the request at this time. This can be a transient problem. It is recommended to wait ten minutes and try again, up to three times. This error code is only returned via the QueryStatus method on the IVssAsync interface returned in the ppAsync parameter.

    VSS_E_REBOOT_REQUIRED

        The provider encountered an error that requires the user to restart the computer.

        Windows Server 2003 and Windows XP:  This value is not supported.

    VSS_E_TRANSACTION_FREEZE_TIMEOUT

        The system was unable to freeze the Distributed Transaction Coordinator (DTC) or the Kernel Transaction Manager (KTM).

        Windows Server 2003 and Windows XP:  This value is not supported.

    VSS_E_TRANSACTION_THAW_TIMEOUT

        The system was unable to thaw the Distributed Transaction Coordinator (DTC) or the Kernel Transaction Manager (KTM).

        Windows Server 2003 and Windows XP:  This value is not supported.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        The provider returned an unexpected error code. This can be a transient problem. It is recommended to wait ten minutes and try again, up to three times. This error code is only returned via the QueryStatus method on the IVssAsync interface returned in the ppAsync parameter.

BackupComplete

    E_INVALIDARG

        ppAsync does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_WRITER_ERROR

        An unexpected error occurred during communication with writers. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

BreakSnapshotSet

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist.

    VSS_E_PROVIDER_VETO

        The shadow copy was created by a software provider and cannot be broken.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

DeleteSnapshots

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist.

    VSS_E_PROVIDER_VETO

        Expected provider error. The provider logged the error in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        Unexpected provider error. The error code is logged in the error log. For more information, see Event and Error Handling Under VSS.

GetSessionId

WriterFailureEx

    VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT

        The shadow copy contains only a subset of the volumes needed by the writer to correctly back up the application component.

    VSS_E_WRITERERROR_OUTOFRESOURCES

        The writer ran out of memory or other system resources. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_TIMEOUT

        The writer operation failed because of a time-out between the Freeze and Thaw events. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_RETRYABLE

        The writer failed due to an error that would likely not occur if the entire backup, restore, or shadow copy creation process was restarted. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_NONRETRYABLE

        The writer operation failed because of an error that might recur if another shadow copy is created. For more information, see Event and Error Handling Under VSS.

    VSS_E_WRITER_NOT_RESPONDING

        The writer is not responding.

    VSS_E_WRITER_STATUS_NOT_AVAILABLE

        The writer status is not available for one or more writers. A writer may have reached the maximum number of available backup and restore sessions.

    VSS_E_WRITERERROR_PARTIAL_FAILURE

        The writer is reporting one or more component-level errors. To retrieve the errors, the requester must use the IVssComponentEx2::GetFailure method.

GetWriterStatusEx

    E_INVALIDARG
    0x80070057L

        The pnStatus, pidWriter, pbstrWriter, or pidInstance parameter is NULL.

    E_OUTOFMEMORY
    0x8007000EL

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE
    0x80042301L

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND
    0x80042308L

        The iWriter parameter specifies a writer that does not exist.

BreakSnapshotSetEx

    E_ACCESSDENIED
    0x80070005L

        The caller does not have sufficient privileges or is not an administrator.

    E_INVALIDARG
    0x80070057L

        One of the parameter values is not valid.

    E_OUTOFMEMORY
    0x8007000EL

        The caller is out of memory or other system resources.

    VSS_E_BREAK_REVERT_ID_FAILED
    0x800423F6L

        The shadow copy set break operation failed because the MBR disk signature, the GPT disk identifier, or the GPT partition identifier of one or more of the destination LUNs could not be reverted to those of the original LUNs. If one or more original LUNs are not masked on the computer, the break operation would cause a signature collision.

    VSS_E_OBJECT_NOT_FOUND
    0x80042308L

        The specified shadow copy does not exist.

    VSS_E_PROVIDER_VETO
    0x80042306L

        The shadow copy was created by a software provider and cannot be broken.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetAuthoritativeRestore

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        This method was not called during a restore operation.

    VSS_E_OBJECT_NOT_FOUND

        The specified component was not found.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetRestoreName

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        This method was not called during a restore operation.

    VSS_E_OBJECT_NOT_FOUND

        The specified component was not found.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetRollForward

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        This method was not called during a restore operation.

    VSS_E_OBJECT_NOT_FOUND

        The specified component was not found.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

UnexposeSnapshot

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist or is not exposed.

    VSS_E_PROVIDER_VETO

        An expected provider error has occurred. The error code is logged in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        An unexpected provider error has occurred. The error code is logged in the error log. For more information, see Event and Error Handling Under VSS.

AddSnapshotToRecoverySet

    VSS_E_BAD_STATE
    0x80042301L

        Either there is no hardware provider that supports the operation, or the requester did not successfully add any volumes to the recovery set.

    VSS_E_LEGACY_PROVIDER
    0x800423F7L

        This version of the hardware provider does not support this operation.

    VSS_E_OBJECT_NOT_FOUND
    0x80042308L

        The snapshotId parameter specifies a shadow copy that the hardware provider does not own.

    VSS_E_RESYNC_IN_PROGRESS
    0x800423FFL

        Another LUN resynchronization operation is already in progress.

    VSS_E_SNAPSHOT_NOT_IN_SET
    0x8004232BL

        The snapshotId parameter specifies a shadow copy that does not exist in the Backup Components Document.

    VSS_E_VOLUME_NOT_SUPPORTED
    0x8004230CL

        LUN resynchronization is not supported on this volume, because it is a dynamic volume, because the destination disk does not have a unique page 83 storage identifier, because the specified volume does not reside on a LUN managed by a VSS hardware provider, or because the destination disk is a cluster quorum disk.

RecoverSet

    E_NOTIMPL
    0x80000001L

        The provider for the volume does not support LUN resynchronization.

    VSS_E_BAD_STATE
    0x80042301L

        Possible reasons for this return value include:

        - There is no hardware provider that supports the operation.
        - The requester did not successfully add any volumes to the recovery set.
        - The method was called in WinPE or in Safe mode.
        - The caller did not call the IVssBackupComponents::InitializeForRestore method before calling this method.

    VSS_E_LEGACY_PROVIDER
    0x800423F7L

        This version of the hardware provider does not support this operation.

    VSS_E_PROVIDER_VETO
    0x80042306L

        An unexpected provider error occurred. If this error code is returned, the error must be described in an entry in the application event log, giving the user information on how to resolve the problem.

    VSS_E_UNSELECTED_VOLUME
    0x8004232AL

        The resynchronization destination contained a volume that was not explicitly included.

    VSS_E_CANNOT_REVERT_DISKID
    0x800423FEL

        The MBR signature or GPT ID for one or more disks could not be set to the intended value. Check the Application event log for more information.

GetWriterMetadataEx

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The iWriter parameter does not point to a valid writer.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetSelectedForRestoreEx

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The component being selected does not exist in the Backup Components Document, or a live instance of the writer corresponding to that component is not running on the system.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetAlternateLocation

    S_FALSE

        The requested information could not be found.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetBackupTypeMask

    E_INVALIDARG

        The pdwTypeMask variable points to a NULL region of memory.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetFilespec

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetPath

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetRecursive

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

GetDatabaseFile

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified database file does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetDatabaseLogFile

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified database log file does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetDependency

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The component specified by the index iDependency does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetFile

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified file does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

AddComponent

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_ALREADY_EXISTS

        The object is a duplicate. A component with the same logical path and component name already exists.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

EnumObject Clone

    E_FAIL

        There is an internal error in the enumerator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    E_POINTER

        One of the required pointer parameters is NULL.

EnumObject Next

    E_FAIL

        There is an internal error in the enumerator.

    E_POINTER

        One of the required pointer parameters is NULL.

EnumObject Reset

    E_FAIL

        There was an internal error in the enumerator.

EnumObject Skip

    E_FAIL

        There was an internal error in the enumerator.

Query

    E_ACCESSDENIED

        The caller is not an administrator or a backup operator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        Out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The queried object is not found.

    VSS_E_PROVIDER_VETO

        Expected provider error. The provider logged the error in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        Unexpected provider error. The error code is logged in the error log. For more information, see Event and Error Handling Under VSS.

DisableWriterClasses

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

DisableWriterInstances

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

EnableWriterClasses

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

ExposeSnapshot

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist.

    VSS_E_PROVIDER_VETO

        Expected provider error. The provider logged the error in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        Unexpected provider error. The error code is logged in the error log. For more information, see Event and Error Handling Under VSS.

FreeWriterMetadata

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

GetSnapshotProperties

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist.

    VSS_E_PROVIDER_VETO

        Expected provider error. The provider logged the error in the event log. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

    VSS_E_UNEXPECTED_PROVIDER_ERROR

        Unexpected provider error. The error code is logged in the error log. For more information, see Event and Error Handling Under VSS.

GetWriterComponents

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The specified shadow copy does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetWriterComponentsCount

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetWriterMetadataCount

    E_INVALIDARG

        One of the parameter values is not valid.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetWriterStatus

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The specified writer does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

WriterFailure

    VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT

        The shadow copy contains only a subset of the volumes needed by the writer to correctly back up the application component.

    VSS_E_WRITERERROR_OUTOFRESOURCES

        The writer ran out of memory or other system resources. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_TIMEOUT

        The writer operation failed because of a time-out between the Freeze and Thaw events. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_RETRYABLE

        The writer failed due to an error that would likely not occur if the entire backup, restore, or shadow copy creation process was restarted. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_NONRETRYABLE

        The writer operation failed because of an error that might recur if another shadow copy is created. For more information, see Event and Error Handling Under VSS.

    VSS_E_WRITER_NOT_RESPONDING

        The writer is not responding.

    VSS_E_WRITER_STATUS_NOT_AVAILABLE

        The writer status is not available for one or more writers. A writer may have reached the maximum number of available backup and restore sessions.

        Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported.

GetWriterStatusCount

    E_INVALIDARG

        One of the parameter values is not valid.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

ImportSnapshots

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_INVALIDARG

        The ppAsync parameter does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called from within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

InitializeForRestore

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The load operation of the specified XML document failed.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

IsVolumeSupported

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    VSS_E_NESTED_VOLUME_LIMIT

        The specified volume is nested too deeply to participate in the VSS operation.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This return code is not supported.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The specified volume or remote file share was not found or was not available.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

PostRestore

    E_ACCESSDENIED

        The caller does not have sufficient backup privileges or is not an administrator.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    E_INVALIDARG

        ppAsync does not point to a valid pointer; that is, it is NULL.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_PROVIDER_VETO

        Expected provider error. The provider logged the error in the event log.

    VSS_E_OBJECT_NOT_FOUND

        The specified volume was not found or was not available.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

PreRestore

    E_INVALIDARG

        The ppAsync parameter does not point to a valid pointer; that is, it is NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

QueryRevertStatus

    E_ACCESSDENIED

        The calling process has insufficient privileges.

    E_FAIL

        There is an internal error.

    E_INVALIDARG

        One of the parameters passed is not valid.

    E_NOTIMPL

        The provider for the volume does not support revert operations.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    E_POINTER

        One of the required pointer parameters is NULL.

    VSS_E_OBJECT_NOT_FOUND

        The pwszVolume parameter is not a valid volume.

    VSS_E_VOLUME_NOT_SUPPORTED

        Revert is not supported on this volume.

RevertToSnapshot

    E_ACCESSDENIED

        The calling process has insufficient privileges.

    E_FAIL

        There is an internal error.

    E_INVALIDARG

        One of the parameters passed is not valid.

    E_NOTIMPL

        The provider for the volume does not support revert operations.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_OBJECT_NOT_FOUND

        The SnapshotId parameter is not a valid shadow copy.

    VSS_E_PROVIDER_NOT_REGISTERED

        The provider was not found.

    VSS_E_REVERT_IN_PROGRESS

        The volume already has a revert in process.

    VSS_E_UNSUPPORTED_CONTEXT

        Revert is only supported for persistent shadow copies.

    VSS_E_VOLUME_IN_USE

        The bForceDismount parameter was FALSE, and the volume could not be locked.

    VSS_E_VOLUME_NOT_SUPPORTED

        Revert is not supported on this volume.

IBackupComponents SaveAsXML

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetAdditionalRestores

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The backup component does not exist.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetBackupOptions

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The backup component does not exist.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetBackupSucceeded

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The backup component does not exist.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetFileRestoreStatus

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The backup component does not exist.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetPreviousBackupStamp

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The backup component does not exist.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetRangesFilePath

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, or this method has been called other than during a restore operation.

    VSS_E_OBJECT_NOT_FOUND

        The component does not exist or the path and file specification do not match a component and file specification in the component.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetRestoreOptions

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The backup component does not exist.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetRestoreState

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a backup operation, or this method has not been called within the correct sequence.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

SetSelectedForRestore

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

    VSS_E_OBJECT_NOT_FOUND

        The component being selected does not exist in the Backup Components Document, or a live instance of the writer corresponding to that component is not running on the system.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetRootAndLogicalPrefixPaths

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

GetIdentityEx

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetIdentity

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetExcludeFromSnapshotCount

    E_INVALIDARG

        The pcExcludedFromSnapshot parameter was NULL.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetExcludeFromSnapshotFile

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetVersion

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetAlternateLocationMapping

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The specified alternate location mapping does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetBackupSchema

    E_INVALIDARG

        The backup schema argument is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

GetExcludeFile

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_OBJECT_NOT_FOUND

        The file set specified for exclusion does not exist.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetRestoreMethod

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetComponentName

    S_FALSE

        No writer can be found that manages the component that the current component depends on.

    E_INVALIDARG

        The pointer pbstrComponentName points to unallocated memory.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetLogicalPath

    S_FALSE

        No writer can be found that manages the component that the current component depends on.

    E_INVALIDARG

        The pointer pbstrLogicalPath points to unallocated memory.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

GetWriterId

    S_FALSE

        No writer can be found that manages the component that the current component depends on.

    E_INVALIDARG

        The pointer pWriterId points to unallocated memory.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

    VSS_E_UNEXPECTED

        Unexpected error. The error code is logged in the error log file. For more information, see Event and Error Handling Under VSS.

        Windows Server 2008, Windows Vista, Windows Server 2003 and Windows XP:  This value is not supported until Windows Server 2008 R2 and Windows 7. E_UNEXPECTED is used instead.

WriterComponents GetComponent

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_OBJECT_NOT_FOUND

        The specified component was not found.

GetComponentCount

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

GetWriterInfo

    E_INVALIDARG

        One of the parameter values is not valid.

    E_OUTOFMEMORY

        The caller is out of memory or other system resources.

    VSS_E_INVALID_XML_DOCUMENT

        The XML document is not valid. Check the event log for details. For more information, see Event and Error Handling Under VSS.

IVssComponentEx2 WriterFailure

    VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT

        The shadow copy contains only a subset of the volumes needed by the writer to correctly back up the application component.

    VSS_E_WRITERERROR_OUTOFRESOURCES

        The writer ran out of memory or other system resources. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_TIMEOUT

        The writer operation failed because of a time-out between the Freeze and Thaw events. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_RETRYABLE

        The writer failed due to an error that would likely not occur if the entire backup, restore, or shadow copy creation process was restarted. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_NONRETRYABLE

        The writer operation failed because of an error that might recur if another shadow copy is created. For more information, see Event and Error Handling Under VSS.

    VSS_E_WRITER_NOT_RESPONDING

        The writer is not responding.

    VSS_E_WRITER_STATUS_NOT_AVAILABLE

        The writer status is not available for one or more writers. A writer may have reached the maximum number of available backup and restore sessions.

GetFailure

    E_INVALIDARG
    0x80070057L

        The phr, phrApplication, pbstrApplicationMessage, or pdwReserved parameter is NULL.

    E_OUTOFMEMORY
    0x8007000EL

        The caller is out of memory or other system resources.

    VSS_E_BAD_STATE
    0x80042301L

        The backup components object is not initialized, this method has been called during a restore operation, or this method has not been called within the correct sequence.

IComponentEx2 SetFailure

ReportableWriterFailure

    VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT

        The shadow copy contains only a subset of the volumes needed by the writer to correctly back up the application component.

    VSS_E_WRITERERROR_OUTOFRESOURCES

        The writer ran out of memory or other system resources. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_TIMEOUT

        The writer operation failed because of a time-out between the Freeze and Thaw events. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_RETRYABLE

        The writer failed due to an error that would likely not occur if the entire backup, restore, or shadow copy creation process was restarted. The recommended way to handle this error code is to wait ten minutes and then repeat the operation, up to three times.

    VSS_E_WRITERERROR_NONRETRYABLE

        The writer operation failed because of an error that might recur if another shadow copy is created. For more information, see Event and Error Handling Under VSS.

CoInitializeEx

    S_OK

        The COM library was initialized successfully on this thread.

    S_FALSE

        The COM library is already initialized on this thread.

    RPC_E_CHANGED_MODE

        A previous call to CoInitializeEx specified the concurrency model for this thread as multithread apartment (MTA). This could also indicate that a change from neutral-threaded apartment to single-threaded apartment has occurred.

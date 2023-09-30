//! This example was inspired by code from:
//! <https://github.com/restic/restic/blob/db8a95899114ef5131818462d057cac202189b3a/internal/fs/vss_windows.go#L763-L777>
//!
//! The above code is used here which can show how to map paths so that they are
//! inside the shadow copy:
//! <https://github.com/restic/restic/blob/db8a95899114ef5131818462d057cac202189b3a/internal/fs/fs_local_vss.go>

use std::{env, error::Error, fmt, fs::File, io, ops::Deref, path::PathBuf};

use volume_shadow_copy as vsc;
use vsc::{
    vsbackup::BackupComponents,
    vss::{BackupType, ObjectType, SnapshotContext, SnapshotProperties, VssAsync},
    VSS_ID,
};

pub type BoxError = Box<dyn Error + Send + Sync + 'static>;

struct PreparedBackup {
    snapshot_id: VSS_ID,
    snapshot_set_id: VSS_ID,
    backup_comp: Option<BackupComponents>,
}
impl PreparedBackup {
    pub fn new(
        snapshot_id: VSS_ID,
        snapshot_set_id: VSS_ID,
        backup_comp: BackupComponents,
    ) -> Self {
        Self {
            snapshot_id,
            snapshot_set_id,
            backup_comp: Some(backup_comp),
        }
    }
    pub fn finish(mut self, timeout_in_millis: Option<u32>) -> Result<(), BoxError> {
        vss_async_wait(self.backup_complete()?, timeout_in_millis)?;
        self.delete_snapshots(self.snapshot_id, ObjectType::Snapshot, false);
        self.delete_snapshots(self.snapshot_set_id, ObjectType::SnapshotSet, false);
        self.backup_comp = None;
        Ok(())
    }
}
impl Deref for PreparedBackup {
    type Target = BackupComponents;

    fn deref(&self) -> &Self::Target {
        self.backup_comp.as_ref().unwrap()
    }
}
impl Drop for PreparedBackup {
    fn drop(&mut self) {
        if let Some(backup_comp) = self.backup_comp.take() {
            let _ = backup_comp.abort_backup();
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnsupportedVolumeError(String);
impl fmt::Display for UnsupportedVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "snapshots not supported for volume {:?}", self.0)
    }
}
impl Error for UnsupportedVolumeError {}

#[derive(Debug, Clone)]
pub struct TimeoutError(u32);
impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "operation timeout after {} milliseconds", self.0)
    }
}
impl Error for TimeoutError {}

fn vss_async_wait<E>(vss_async: VssAsync<E>, timeout_in_millis: Option<u32>) -> Result<(), BoxError>
where
    E: From<i32> + Error + Send + Sync + 'static,
{
    match vss_async.wait(timeout_in_millis) {
        Ok(()) => {
            if matches!(vss_async.query_status()?, vsc::vss::AsyncStatus::Pending) {
                // Timed out:
                let _ = vss_async.cancel();
                Err(Box::new(TimeoutError(timeout_in_millis.expect(
                    "assumed timeout can't occur when a custom timeout is not specified",
                ))))
            } else {
                Ok(())
            }
        }
        // Err(e) if e.async_error().kind() == vsc::errors::WaitErrorKind::OTHER => {},
        Err(e) => Err(Box::new(e)),
    }
}

pub struct VssSnapshot {
    prepared_backup: PreparedBackup,
    /// Cached snapshot properties.
    snapshot_properties: SnapshotProperties,
    timeout_in_millis: Option<u32>,
}
impl VssSnapshot {
    pub fn snapshot_properties(&self) -> &SnapshotProperties {
        &self.snapshot_properties
    }
    pub fn create_snapshot(volume: &str, timeout_in_millis: Option<u32>) -> Result<Self, BoxError> {
        let volume = volume.replace('/', "\\");
        let volume_wide = vsc::widestring::U16CString::from_str(&volume)?;

        vsc::initialize_com()?;

        let backup_comp = BackupComponents::new()?;
        backup_comp.initialize_for_backup(None)?;
        backup_comp.set_context(SnapshotContext::Backup, Default::default())?;
        backup_comp.set_backup_state(false, false, BackupType::Copy, false)?;
        vss_async_wait(backup_comp.gather_writer_metadata()?, timeout_in_millis)?;
        let is_supported = backup_comp.is_volume_supported(None, &volume_wide)?;
        if !is_supported {
            return Err(UnsupportedVolumeError(volume).into());
        }
        let snapshot_set_id = backup_comp.start_snapshot_set()?;

        let snapshot_id = backup_comp.add_to_snapshot_set(&volume_wide, None)?;
        vss_async_wait(backup_comp.prepare_for_backup()?, timeout_in_millis)?;

        // After calling PrepareForBackup one needs to call AbortBackup() before releasing the VSS
        // instance for proper cleanup.
        let prepared_backup = PreparedBackup::new(snapshot_id, snapshot_set_id, backup_comp);

        vss_async_wait(prepared_backup.do_snapshot_set()?, timeout_in_millis)?;
        let snapshot_properties = prepared_backup.get_snapshot_properties(snapshot_id)?;
        Ok(Self {
            prepared_backup,
            snapshot_properties,
            timeout_in_millis,
        })
    }
    pub fn finish(self) -> Result<(), BoxError> {
        self.prepared_backup.finish(self.timeout_in_millis)
    }
}

fn main() {
    let volume = env::args_os()
        .nth(1)
        .expect("first arg is volume to snapshot")
        .to_str()
        .unwrap()
        .to_owned();
    if let Some(file_path) = env::args_os().nth(2) {
        if PathBuf::from(&file_path).is_absolute() {
            panic!("ERROR: The 2nd argument must be a path relative to the created snapshot");
        }
    }
    eprintln!(
        "Attempting to create a shadow copy of the volume: {}",
        volume
    );
    let snapshot = VssSnapshot::create_snapshot(&volume, Some(120 * 1000)).unwrap();
    eprintln!("Created snapshot for: {}", volume);

    dbg!(snapshot
        .snapshot_properties()
        .exposed_name()
        .map(|v| v.to_string_lossy()));
    dbg!(snapshot
        .snapshot_properties()
        .exposed_path()
        .map(|v| v.to_string_lossy()));
    dbg!(snapshot
        .snapshot_properties()
        .original_volume_name()
        .to_string_lossy());

    // This is the path that should be used to interact with the created shadow copy:
    dbg!(snapshot
        .snapshot_properties()
        .snapshot_device_object()
        .to_string_lossy());
    dbg!(snapshot.snapshot_properties().status());

    if let Some(file_path) = env::args_os().nth(2) {
        let snapshot_path = snapshot
            .snapshot_properties()
            .snapshot_device_object()
            .to_string_lossy();
        if let Err(e) =
            File::open(PathBuf::from(snapshot_path).join(file_path)).and_then(|mut file| {
                eprintln!();
                eprintln!("File Content:");
                io::copy(&mut file, &mut io::stdout())
            })
        {
            eprintln!("Failed to write file content to stdout: {e}");
        }
    } else {
        eprintln!();
        eprintln!("Press Enter to finish backup");
        let _ = io::stdin().read_line(&mut String::new());
    };
    eprintln!();
    eprintln!("Finishing backup...");
    snapshot.finish().unwrap();
    eprintln!("Cleanup completed!");
}

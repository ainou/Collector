use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Guard that removes the temp file on drop unless disarmed (after successful rename).
pub(crate) struct TempFileGuard {
    path: PathBuf,
    active: bool,
}

impl TempFileGuard {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path, active: true }
    }

    pub(crate) fn disarm(&mut self) {
        self.active = false;
    }
}

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        if self.active {
            let _ = fs::remove_file(&self.path);
        }
    }
}

/// Build a unique temporary file path in the same directory as `file_path`,
/// using filename, PID, nanosecond timestamp, and attempt number.
fn build_atomic_temp_path(file_path: &Path, attempt: u32) -> Result<PathBuf, String> {
    let parent = file_path
        .parent()
        .ok_or_else(|| "Target file has no parent directory".to_string())?;
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Target file has no valid filename".to_string())?;
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    Ok(parent.join(format!(
        ".{}.{}.{}.{}.tmp",
        file_name,
        std::process::id(),
        nanos,
        attempt
    )))
}

/// Atomically write `content` to `file_path` by writing to a temporary file
/// first, syncing it, then renaming over the target.
pub(crate) fn atomic_write_text(file_path: &Path, content: &str) -> Result<(), String> {
    let mut last_error = None;

    for attempt in 0..16 {
        let temp_path = build_atomic_temp_path(file_path, attempt)?;
        let mut guard = TempFileGuard::new(temp_path.clone());

        let mut temp_file = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)
        {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                last_error = Some(error);
                continue;
            }
            Err(error) => return Err(format!("Cannot create temporary file: {}", error)),
        };

        temp_file
            .write_all(content.as_bytes())
            .map_err(|error| format!("Cannot write temporary file: {}", error))?;
        temp_file
            .sync_all()
            .map_err(|error| format!("Cannot sync temporary file: {}", error))?;
        drop(temp_file);

        fs::rename(&temp_path, file_path)
            .map_err(|error| format!("Cannot replace target file atomically: {}", error))?;
        guard.disarm();
        return Ok(());
    }

    Err(format!(
        "Cannot create unique temporary file: {}",
        last_error
            .map(|error| error.to_string())
            .unwrap_or_else(|| "too many attempts".to_string())
    ))
}

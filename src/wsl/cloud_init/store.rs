use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugCopyOutcome {
    Written(PathBuf),
    Skipped(String),
}

pub fn store(target_file: &Path, rendered: &str) -> anyhow::Result<()> {
    let target_dir: &Path = target_file.parent().expect("user-data path always has a parent");
    std::fs::create_dir_all(target_dir)?;
    std::fs::write(target_file, rendered)?;
    Ok(())
}

pub fn copy_debug_to_current_dir(name: &str, rendered: &str) -> DebugCopyOutcome {
    let debug_dir: PathBuf = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return DebugCopyOutcome::Skipped(format!("cwd error: {err}"));
        }
    };
    let debug_path: PathBuf = debug_dir.join(format!("cloud-init.{}.user-data", name));
    if let Err(err) = std::fs::write(&debug_path, rendered) {
        DebugCopyOutcome::Skipped(format!("write error: {err}"))
    } else {
        DebugCopyOutcome::Written(debug_path)
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/wsl/cloud_init/store_tests.rs"]
mod store_tests;

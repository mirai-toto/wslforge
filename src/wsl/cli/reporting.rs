use crate::config::{ImageSource, Profile};
use crate::wsl::helpers::path;
use crate::wsl::{CloudInitEvent, CreateEvent, CreateOutcome, CreateReport};
use log::info;

pub fn log_create_report(report: &CreateReport, hostname: &str) {
    for event in &report.events {
        match event {
            CreateEvent::InstanceCheckStarted => {
                info!("🔍 Checking if WSL instance '{}' exists...", hostname);
            }
            CreateEvent::InstanceExists => {
                info!("✅ WSL instance '{}' exists.", hostname);
            }
            CreateEvent::InstanceMissing => {
                info!("ℹ️ WSL instance '{}' does not exist.", hostname);
            }
            CreateEvent::OverrideRequested => {}
            CreateEvent::OverrideExistingInstance => {
                info!("⚠️ WSL instance '{}' already exists and will be overridden.", hostname);
            }
            CreateEvent::DeleteSkippedMissing => {
                info!("ℹ️ WSL instance '{}' does not exist. Skipping delete.", hostname);
            }
            CreateEvent::DeleteDryRun => {
                info!("🧪 Dry run: WSL instance '{}' would be deleted", hostname);
            }
            CreateEvent::DeleteStarted => {
                info!("🧹 Deleting existing WSL instance '{}'", hostname);
            }
            CreateEvent::DeleteCompleted => {
                info!("✅ WSL instance '{}' deleted successfully.", hostname);
            }
            CreateEvent::CreateDryRun => {
                info!("🧪 Dry run: WSL instance would be created");
            }
            CreateEvent::CreateStarted => {
                info!("🚀 Creating WSL instance");
            }
            CreateEvent::CloudInit(event) => match event {
                CloudInitEvent::NotConfigured => {
                    info!("☁️ Cloud-init: not configured");
                }
                CloudInitEvent::SourceFile(path) => {
                    info!("☁️ Cloud-init source: {}", path.display());
                }
                CloudInitEvent::SourceInline => {
                    info!("☁️ Cloud-init source: inline content");
                }
                CloudInitEvent::DryRunTarget(path) => {
                    info!("🧪 Dry run: cloud-init target would be created at: {}", path.display());
                }
                CloudInitEvent::TargetWritten(path) => {
                    info!("☁️ Cloud-init target: {}", path.display());
                }
                CloudInitEvent::DebugCopyWritten(path) => {
                    info!("☁️ Cloud-init debug copy: {}", path.display());
                }
                CloudInitEvent::DebugCopySkipped(reason) => {
                    info!("☁️ Cloud-init debug copy skipped ({reason})");
                }
            },
        }
    }

    match report.outcome {
        CreateOutcome::Created => {
            info!("✅ WSL instance '{}' created successfully.", hostname);
        }
        CreateOutcome::AlreadyExists => {
            info!("ℹ️ WSL instance '{}' already exists.", hostname);
        }
        CreateOutcome::Skipped => {
            info!("ℹ️ WSL instance '{}' was skipped.", hostname);
        }
    }
}

pub fn log_config_summary(profile_name: &str, profile: &Profile) {
    info!("🧩 Profile: {}", profile_name);
    info!("♻️ Override: {}", profile.override_instance);
    info!("🏷️ Hostname: {}", profile.hostname);
    info!("👤 User: {}", profile.username);
    info!("📦 Install dir: {}", expand_install_dir(profile));
    match &profile.cloud_init {
        Some(source) => info!("☁️ Cloud-init: {}", source),
        None => info!("☁️ Cloud-init: not configured"),
    }

    match &profile.image {
        ImageSource::Distro { name } => {
            info!("🐧 Using WSL distro '{}'", name);
        }
        ImageSource::File { path } => {
            info!("🗂️  Using image file {:?}", path);
        }
    }

    if let Some(proxy) = &profile.http_proxy {
        info!("🌐 HTTP proxy: {}", proxy);
    }
    if let Some(proxy) = &profile.https_proxy {
        info!("🔐 HTTPS proxy: {}", proxy);
    }
}

fn expand_install_dir(profile: &Profile) -> String {
    match path::expand_path(&profile.install_dir) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => profile.install_dir.to_string_lossy().into_owned(),
    }
}

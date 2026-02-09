mod domain;
mod system;

pub use domain::validate_image_source;
pub use system::{validate_environment, validate_wsl_distro_name};

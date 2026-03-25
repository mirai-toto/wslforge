mod cloud_init;
mod image;
mod instance;
mod source;

pub use cloud_init::CloudInitSource;
pub use image::ImageSource;
pub use instance::{Config, Instance, Proxy};
pub use source::SourcePath;

#[cfg(test)]
#[path = "../../../tests/unit/config/model_tests.rs"]
mod model_tests;

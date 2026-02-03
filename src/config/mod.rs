mod loader;
mod model;

pub use loader::load_yaml;
pub use model::{AppConfig, CloudInitSource, ImageSource};

use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn spinner(msg: String) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(msg);
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

pub fn with_spinner<T>(msg: String, f: impl FnOnce() -> T) -> T {
    let pb = spinner(msg);
    let result = f();
    pb.finish_and_clear();
    result
}

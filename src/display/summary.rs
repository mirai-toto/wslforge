use std::collections::BTreeMap;

use comfy_table::{Attribute, Cell, Color, Table};
use console::style;

use crate::wsl::{InstanceResult, Status};

pub fn print_summary(results: &BTreeMap<String, InstanceResult>) {
    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Instance").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
        Cell::new("Hostname").add_attribute(Attribute::Bold),
    ]);

    for (name, result) in results {
        let (status_str, color) = match result.outcome {
            Status::Created => ("✅ Created", Color::Green),
            Status::AlreadyExists => ("⚠️  Already exists", Color::Yellow),
            Status::Skipped => ("🔍 Skipped (dry run)", Color::Cyan),
        };
        table.add_row(vec![
            Cell::new(name),
            Cell::new(status_str).fg(color),
            Cell::new(&result.hostname),
        ]);
    }

    println!(
        "\n{}",
        style("── Results ──────────────────────────────────────────").dim()
    );
    println!("{table}");
}

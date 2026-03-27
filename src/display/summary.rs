use std::collections::BTreeMap;

use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use console::style;

use crate::wsl::helpers::wrap;
use crate::wsl::{InstanceResult, Status};

pub fn print_summary(results: &BTreeMap<String, InstanceResult>) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Instance").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
        Cell::new("Hostname").add_attribute(Attribute::Bold),
    ]);

    for (name, result) in results {
        let (status_str, color) = match &result.outcome {
            Status::Created => ("✅ Created".to_string(), Color::Green),
            Status::Recreated => ("♻️  Recreated".to_string(), Color::Blue),
            Status::AlreadyExists => ("⚠️  Already exists".to_string(), Color::Yellow),
            Status::Skipped => ("🔍 Skipped (dry run)".to_string(), Color::Cyan),
            Status::Failed(e) => (format!("❌ Failed: {}", wrap(e, 60)), Color::Red),
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

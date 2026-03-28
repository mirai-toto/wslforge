use std::io::ErrorKind;
use std::path::PathBuf;

use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::CustomUserError;

#[derive(Clone, Default)]
pub struct FilePathCompleter {
    paths: Vec<String>,
    lcp: String,
}

impl FilePathCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        let (scan_dir, fallback) = resolve_scan_dir(input);
        let entries = read_entries(scan_dir, fallback)?;
        self.paths = filter_completions(&entries, input);
        self.lcp = longest_common_prefix(&self.paths);
        Ok(())
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;
        Ok(self.paths.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;
        Ok(match highlighted_suggestion {
            Some(s) => Replacement::Some(s),
            None if self.lcp.is_empty() => Replacement::None,
            None => Replacement::Some(self.lcp.clone()),
        })
    }
}

fn resolve_scan_dir(input: &str) -> (PathBuf, PathBuf) {
    let input_path = if input.is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(input)
    };
    let fallback = input_path
        .parent()
        .map(|p| {
            if p.to_string_lossy() == "" {
                PathBuf::from(".")
            } else {
                p.to_owned()
            }
        })
        .unwrap_or_else(|| PathBuf::from("."));
    let scan_dir = if input.ends_with('/') {
        input_path
    } else {
        fallback.clone()
    };
    (scan_dir, fallback)
}

fn read_entries(scan_dir: PathBuf, fallback: PathBuf) -> Result<Vec<std::fs::DirEntry>, CustomUserError> {
    match std::fs::read_dir(scan_dir) {
        Ok(rd) => Ok(rd),
        Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_dir(fallback),
        Err(err) => Err(err),
    }?
    .collect::<Result<_, _>>()
    .map_err(Into::into)
}

fn filter_completions(entries: &[std::fs::DirEntry], input: &str) -> Vec<String> {
    entries
        .iter()
        .filter_map(|entry| {
            let path = entry.path();
            let path_str = if path.is_dir() {
                format!("{}/", path.to_string_lossy())
            } else {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if !matches!(ext, "yaml" | "yml") {
                    return None;
                }
                path.to_string_lossy().into_owned()
            };
            if path_str.starts_with(input) && path_str.len() != input.len() {
                Some(path_str)
            } else {
                None
            }
        })
        .take(15)
        .collect()
}

fn longest_common_prefix(paths: &[String]) -> String {
    match (paths.iter().min(), paths.iter().max()) {
        (Some(first), Some(last)) => first
            .chars()
            .zip(last.chars())
            .take_while(|(a, b)| a == b)
            .map(|(c, _)| c)
            .collect(),
        _ => String::new(),
    }
}

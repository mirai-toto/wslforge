/// Builds a `chown` ownership specifier from separate owner and group values.
pub(super) fn chown_spec(owner: Option<&str>, group: Option<&str>) -> String {
    match (owner, group) {
        (Some(o), Some(g)) => format!("'{o}:{g}'"),
        (Some(o), None) => format!("'{o}'"),
        (None, Some(g)) => format!("':{g}'"),
        (None, None) => String::new(),
    }
}

/// Returns shell commands that create the parent directory of `path` and all
/// intermediate ancestors with the given ownership. Returns an empty vec if
/// `path` has no parent component.
pub(super) fn make_parent_dirs(path: &str, owner: Option<&str>, group: Option<&str>) -> Vec<String> {
    let parent = std::path::Path::new(path)
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    if parent.is_empty() {
        return vec![];
    }
    make_dirs(&parent, owner, group)
}

/// Returns shell commands that create `path` and all intermediate ancestors
/// with the given ownership. Each component gets a guarded `mkdir` + `chown`
/// so only newly-created directories are touched. Falls back to `mkdir -p`
/// when no ownership is needed.
pub(super) fn make_dirs(path: &str, owner: Option<&str>, group: Option<&str>) -> Vec<String> {
    if owner.is_none() && group.is_none() {
        return vec![format!("mkdir -p \"{path}\"")];
    }
    let spec = chown_spec(owner, group);
    std::path::Path::new(path)
        .ancestors()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .filter(|p| *p != std::path::Path::new("/") && !p.as_os_str().is_empty())
        .map(|p| {
            let s = p.to_string_lossy();
            format!("[ -d \"{s}\" ] || {{ mkdir \"{s}\" && chown {spec} \"{s}\"; }}")
        })
        .collect()
}

/// Returns shell commands that write a file from stdin at `dest`.
pub(super) fn copy_file(dest: &str, owner: Option<&str>, group: Option<&str>, mode: Option<&str>) -> Vec<String> {
    let mut script = vec![format!("cat > \"{dest}\"")];
    if owner.is_some() || group.is_some() {
        script.push(format!("chown {} \"{dest}\"", chown_spec(owner, group)));
    }
    if let Some(m) = mode {
        script.push(format!("chmod '{m}' \"{dest}\""));
    }
    script
}

/// Returns shell commands that extract a tar archive from stdin into `dest`.
pub(super) fn copy_dir(dest: &str, owner: Option<&str>, group: Option<&str>, mode: Option<&str>) -> Vec<String> {
    let mut script = vec![format!("tar xf - -C \"{dest}\"")];
    if owner.is_some() || group.is_some() {
        script.push(format!("chown -R {} \"{dest}\"", chown_spec(owner, group)));
    }
    if let Some(m) = mode {
        script.push(format!("chmod -R '{m}' \"{dest}\""));
    } else {
        // tar archives built on Windows may not carry valid Unix file modes,
        // so apply sensible defaults: rwx for owner, r-x for group/others on
        // directories; rw for owner, r-- for group/others on regular files.
        script.push(format!("chmod -R 'u+rwX,go+rX' \"{dest}\""));
    }
    script
}

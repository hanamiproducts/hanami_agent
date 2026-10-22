use super::ToolSpec;
use serde_json::{json, Value};

/// Bigger files are refused: they would flood the model's context (and cost money), and
/// reading something like /dev/zero would never end.
const MAX_BYTES: u64 = 1_000_000;

pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "read_file",
        description: "Reads a text file from disk and returns its contents.",
        parameters: json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Path of the file to read." }
            },
            "required": ["path"]
        }),
    }
}

/// True when the file can be read without asking: it is inside the current folder
/// (after following symlinks and "..", so "../../.ssh/id_rsa" or a symlink to it does not
/// count) and its name does not start with ".env". Anything that can't be checked asks.
pub(super) fn is_free_to_read(path: &str) -> bool {
    let (Ok(full), Ok(here)) = (std::fs::canonicalize(path), std::env::current_dir()) else {
        return false;
    };
    let Ok(here) = here.canonicalize() else {
        return false;
    };
    let secret_looking = full
        .file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with(".env"));
    full.starts_with(here) && !secret_looking
}

pub(super) async fn run(args: &Value) -> String {
    // args is a JSON object like {"path": "Cargo.toml"}; pull the string out.
    let Some(path) = args["path"].as_str() else {
        return String::from("Error: missing 'path' argument");
    };

    // Only regular files: not folders, not devices like /dev/zero or /dev/stdin.
    match tokio::fs::metadata(path).await {
        Ok(meta) if !meta.is_file() => return format!("Error: '{path}' is not a regular file."),
        Ok(meta) if meta.len() > MAX_BYTES => {
            return format!(
                "Error: '{path}' is {} bytes; the limit is {MAX_BYTES}. Use run_command \
                 with head, tail or grep to read part of it.",
                meta.len()
            );
        }
        Ok(_) => {}
        Err(e) => return format!("Error: could not read '{path}': {e}"),
    }

    match tokio::fs::read_to_string(path).await {
        Ok(contents) => contents,
        Err(e) => format!("Error: could not read '{path}': {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn files_in_this_folder_are_free() {
        // cargo test runs in the crate folder, where Cargo.toml is.
        assert!(is_free_to_read("Cargo.toml"));
        assert!(is_free_to_read("./src/../Cargo.toml"));
    }

    #[test]
    fn files_outside_or_secret_looking_ask_first() {
        assert!(!is_free_to_read("/etc/hostname"));
        assert!(!is_free_to_read("../../../../../../etc/passwd"));
        assert!(!is_free_to_read("does-not-exist.txt"));
        let env = Path::new(".env.read_file_test");
        std::fs::write(env, "X=1").unwrap();
        let free = is_free_to_read(".env.read_file_test");
        std::fs::remove_file(env).unwrap();
        assert!(!free);
    }

    #[tokio::test]
    async fn devices_and_folders_are_refused() {
        let out = run(&json!({"path": "/dev/zero"})).await;
        assert!(out.contains("not a regular file"), "{out}");
        let out = run(&json!({"path": "src"})).await;
        assert!(out.contains("not a regular file"), "{out}");
    }
}

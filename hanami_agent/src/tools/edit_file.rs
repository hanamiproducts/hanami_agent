use super::ToolSpec;
use serde_json::{Value, json};

pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "edit_file",
        description: "Rpeplaces an exact piece of text in an existing file, \
                    old_content must match the file exactly (including spaces and identation) \
                    and apear only once, unless replace_all is true. Read the file first. \
                    To create new file, use write_file instead.",
        parameters: json!({
            "type":"object",
            "properties": {
                "path": {
                    "type":"string",
                    "description":"Path of the spesific file to edit."
                },
                "old_content": {
                    "type": "string",
                    "description": "The exact text to find in the file."
                },
                "new_content":  {
                    "type":"string",
                    "description":"The content you want to add to the file."
                },
                "replace_all": {
                    "type":"boolean",
                    "description": "Replace every match instead of just one. Default is false."
                }
            },
            "required":["path", "old_content","new_content"]
        }),
    }
}

pub(super) async fn run(args: &Value) -> String {
    let Some(path) = args["path"].as_str() else {
        return String::from("Error: missing 'path' argument.");
    };

    let Some(old_content) = args["old_content"].as_str() else {
        return String::from("Error: missing 'old_content' argument.");
    };

    let Some(new_content) = args["new_content"].as_str() else {
        return String::from("Error missing 'new_content' argument.");
    };

    let replace_all = args["replace_all"].as_bool().unwrap_or(false);

    if old_content.is_empty() {
        return String::from(
            "Error: 'old_content' is empty. Use write_file to create or fill a file.",
        );
    }

    if old_content == new_content {
        return String::from("Error: 'old_content' and 'new_content' are same; nothing to change.");
    }

    let contents = match tokio::fs::read_to_string(path).await {
        Ok(c) => c,
        Err(e) => return format!("Error: Could not read '{path}': {e}"),
    };

    let count = contents.matches(old_content).count();

    if count == 0 {
        return format!(
            "Error: 'old_content' was not found in '{path}'. Re-read the file; whitespaces and indentation must match exactly."
        );
    }

    if count > 1 && !replace_all {
        return format!(
            "Error: 'old_content' appears {count} times in '{path}'. \
        Add more surrounding lines so it is unique, or set replace_all to true."
        );
    }

    //Maybe this will need a refactor. Not sure.
    let new_contents = if replace_all {
        contents.replace(old_content, new_content)
    } else {
        contents.replacen(old_content, new_content, 1)
    };

    match tokio::fs::write(path, new_contents).await {
        Ok(()) => {
            let n = if replace_all { count } else { 1 };
            format!("Edited '{path}' ({n} replacement(s)).")
        }
        Err(e) => format!("Error: Could not write '{path}': {e}"),
    }
}

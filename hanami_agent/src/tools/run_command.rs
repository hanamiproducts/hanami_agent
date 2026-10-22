use super::ToolSpec;
use std::time::Duration;
use tokio::process::Command;
use serde_json::{json, Value};

/// stdout and stderr are each cut to this many characters. A command like `cat big.log`
/// or `yes` would otherwise send megabytes to the model (context overflow, real cost).
const MAX_OUTPUT_CHARS: usize = 20_000;

/// Keeps the first MAX_OUTPUT_CHARS characters and says how much was dropped.
fn cut(text: &str) -> String {
    let total = text.chars().count();
    if total <= MAX_OUTPUT_CHARS {
        return text.to_string();
    }
    let kept: String = text.chars().take(MAX_OUTPUT_CHARS).collect();
    format!("{kept}\n[... cut: {} more characters]", total - MAX_OUTPUT_CHARS)
}

pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "run_command",
        description: "function gives capability to run commands at cmd.",
        parameters: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "path that will command be executed."
                },
                "command": {
                    "type": "string",
                    "description": "command will be executed."
                }
            },
            "required": ["path", "command"]
        }),
    }
}

pub(super) async fn run(args: &Value) -> String {
    
    let Some(path) = args["path"].as_str() else {
        return String::from("Error: missing 'path' variable.");
    };

    let Some(command) = args["command"].as_str() else {
       return String::from("Error: missing 'command' variable.");  
    };

    match tokio::fs::metadata(path).await{
        Ok(meta) if meta.is_dir() => {},
        Ok(_) => return format!("Error: '{path}' is not a directory."),
        Err(_) => return format!("Error: '{path}' does not exists.")
    }

    //This will be a problem at the fraature for windows. We can work on that 
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(path)
        .kill_on_drop(true)
        .output();

    let output = match tokio::time::timeout(Duration::from_secs(30),child).await {
        Err(_) => return String::from("Error: command timed out after 30 seconds."),
        Ok(Err(e)) => return format!("Error: could not start command {e}"),
        Ok(Ok(output)) => output,
    };

    let stdout = cut(&String::from_utf8_lossy(&output.stdout));
    let stderr = cut(&String::from_utf8_lossy(&output.stderr));

    format!(
        "exit code: {}\nstdout:{stdout}\nstderr:{stderr}",
        output.status.code().map_or("killed".to_string(), |c| c.to_string())
    )

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_keeps_short_output() {
        assert_eq!(cut("ok"), "ok");
    }

    #[test]
    fn cut_shortens_long_output() {
        let long = "a".repeat(MAX_OUTPUT_CHARS + 5);
        let out = cut(&long);
        assert!(out.ends_with("[... cut: 5 more characters]"), "{out}");
    }
}

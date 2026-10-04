use super::ToolSpec;
use serde_json::{Value, json};

// This tool call is under construction.
pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "web_search",
        description: "makes resarch at the web. Takes a spesific link and returns the content.",
        parameters: json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "URL of the website to read"
                },
                "required": ["url"]
            }
        }),
    }
}

pub(super) async fn run(args: &Value) -> String {
    "This tool call is currently under production".to_string()
}

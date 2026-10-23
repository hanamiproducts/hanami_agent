use super::ToolSpec;
use serde_json::{Value, json};

//Not sure what to take? Maybe a system prompt and user prompt? or just the user prompt and the
//specilized struct? Not too clear currently.
pub(super) fn spec() -> ToolSpec {
    ToolSpec {
        name: "create_subagent",
        description: "Creates a subagent with specilized prompt and system prompt for you.",
        parameters: json!({ "" }),
    }
}

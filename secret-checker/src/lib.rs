use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
struct ToolInput {
    file_path: PathBuf,
    contents: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PreToolUseInput {
    session_id: String,
    transcript_path: PathBuf,
    cwd: PathBuf,
    permission_mode: PermissionMode,
    hook_event_name: String,
    tool_name: String,
    tool_input: ToolInput,
    tool_use_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum PermissionMode {
    Default,
    Plan,
    AcceptEdits,
    BypassPermissions,
}
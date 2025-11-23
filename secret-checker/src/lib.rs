use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolInput {
    file_path: Option<PathBuf>,
    content: Option<String>,
}

impl ToolInput {
    pub fn file_path(&self) -> &Option<PathBuf> {
        &self.file_path
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreToolUseInput {
    session_id: String,
    transcript_path: PathBuf,
    cwd: PathBuf,
    permission_mode: PermissionMode,
    hook_event_name: String,
    tool_name: String,
    tool_input: ToolInput,
    tool_use_id: String,
}

impl PreToolUseInput {
    pub fn session_id(&self) -> &String {
        &self.session_id
    }

    pub fn transcript_path(&self) -> &PathBuf {
        &self.transcript_path
    }

    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    pub fn permission_mode(&self) -> &PermissionMode {
        &self.permission_mode
    }

    pub fn hook_event_name(&self) -> &String {
        &self.hook_event_name
    }

    pub fn tool_name(&self) -> &String {
        &self.tool_name
    }

    pub fn tool_input(&self) -> &ToolInput {
        &self.tool_input
    }

    pub fn tool_use_id(&self) -> &String {
        &self.tool_use_id
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum PermissionMode {
    Default,
    Plan,
    AcceptEdits,
    BypassPermissions,
}
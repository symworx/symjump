// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::path::Path;

use symjump_config::{AgentAction, ToolboxAction};

use crate::error::CoreError;

pub fn render_agent_cmd(action: &AgentAction, path: &Path) -> String {
    action.cmd.replace("{path}", &path.display().to_string())
}

pub fn toolbox_enter_cmd(action: &ToolboxAction) -> String {
    format!("toolbox enter {}", action.name)
}

pub fn exec_line(cmd: &str, path: &Path) -> Result<String, CoreError> {
    if path.as_os_str().is_empty() {
        return Err(CoreError::EmptyPath);
    }
    if cmd.contains("{path}") {
        Ok(cmd.replace("{path}", &path.display().to_string()))
    } else {
        Ok(format!("cd {} && {}", path.display(), cmd))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::exec_line;

    #[test]
    fn exec_line_cd_or_substitute() {
        assert_eq!(
            exec_line("grok", Path::new("/proj")).unwrap(),
            "cd /proj && grok"
        );
        assert_eq!(
            exec_line("codex -C {path}", Path::new("/proj")).unwrap(),
            "codex -C /proj"
        );
    }
}

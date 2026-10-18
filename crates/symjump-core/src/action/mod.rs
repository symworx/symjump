// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod add;
mod exec;
mod list;
mod remove;

use symjump_config::{AgentAction, Config, ToolboxAction};

use crate::error::CoreError;

pub use add::{add_agent, add_toolbox};
pub use exec::{exec_line, render_agent_cmd, toolbox_enter_cmd};
pub use list::action_list_lines;
pub use remove::remove_action;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    Agent,
    Toolbox,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemovedAction {
    Agent(AgentAction),
    Toolbox(ToolboxAction),
}

impl RemovedAction {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Agent(_) => "agent",
            Self::Toolbox(_) => "toolbox",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Agent(a) => &a.label,
            Self::Toolbox(t) => &t.label,
        }
    }
}

pub fn find_agent<'a>(cfg: &'a Config, query: &str) -> Result<&'a AgentAction, CoreError> {
    cfg.actions
        .agent
        .iter()
        .find(|a| a.label.eq_ignore_ascii_case(query) || a.keys.as_deref() == Some(query))
        .ok_or_else(|| CoreError::UnknownAgent(query.into()))
}

pub fn find_toolbox<'a>(cfg: &'a Config, query: &str) -> Result<&'a ToolboxAction, CoreError> {
    cfg.actions
        .toolbox
        .iter()
        .find(|a| a.label.eq_ignore_ascii_case(query) || a.keys.as_deref() == Some(query))
        .ok_or_else(|| CoreError::UnknownToolbox(query.into()))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::test_support::cfg;

    use super::{find_agent, find_toolbox, render_agent_cmd, toolbox_enter_cmd};

    #[test]
    fn agent_and_toolbox_lookup() {
        let c = cfg();
        assert_eq!(find_agent(&c, "g").unwrap().label, "grok-build");
        assert_eq!(
            toolbox_enter_cmd(find_toolbox(&c, "p").unwrap()),
            "toolbox enter dev-python"
        );
        assert_eq!(
            render_agent_cmd(find_agent(&c, "c").unwrap(), Path::new("/r")),
            "codex -C /r"
        );
    }
}

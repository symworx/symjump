// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::{AgentAction, Config, ToolboxAction};

use crate::error::CoreError;
use crate::keys::check_action_key;

pub fn add_agent(
    cfg: &mut Config,
    label: String,
    cmd: String,
    keys: Option<String>,
) -> Result<(), CoreError> {
    if cmd.trim().is_empty() {
        return Err(CoreError::EmptyCmd);
    }
    if let Some(k) = keys.as_deref() {
        check_action_key(cfg, k, Some(&label))?;
    }
    if let Some(existing) = cfg
        .actions
        .agent
        .iter_mut()
        .find(|a| a.label.eq_ignore_ascii_case(&label))
    {
        existing.cmd = cmd;
        if keys.is_some() {
            existing.keys = keys;
        }
        return Ok(());
    }
    cfg.actions.agent.push(AgentAction { label, cmd, keys });
    Ok(())
}

pub fn add_toolbox(
    cfg: &mut Config,
    label: String,
    name: String,
    keys: Option<String>,
) -> Result<(), CoreError> {
    if name.trim().is_empty() {
        return Err(CoreError::EmptyToolboxName);
    }
    if let Some(k) = keys.as_deref() {
        check_action_key(cfg, k, Some(&label))?;
    }
    if let Some(existing) = cfg
        .actions
        .toolbox
        .iter_mut()
        .find(|t| t.label.eq_ignore_ascii_case(&label))
    {
        existing.name = name;
        if keys.is_some() {
            existing.keys = keys;
        }
        return Ok(());
    }
    cfg.actions
        .toolbox
        .push(ToolboxAction { label, name, keys });
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::error::CoreError;
    use crate::test_support::cfg;

    use super::{add_agent, add_toolbox};

    #[test]
    fn add_action_updates_and_rejects_taken_keys() {
        let mut c = cfg();
        add_agent(&mut c, "grok-build".into(), "grok --foo".into(), None).unwrap();
        assert_eq!(c.actions.agent[0].cmd, "grok --foo");
        assert!(matches!(
            add_agent(&mut c, "other".into(), "echo".into(), Some("g".into())),
            Err(CoreError::KeyTaken(k, lab)) if k == "g" && lab == "grok-build"
        ));
        add_toolbox(&mut c, "rust".into(), "dev-rust".into(), Some("r".into())).unwrap();
        assert_eq!(c.actions.toolbox.len(), 2);
        assert!(matches!(
            add_agent(&mut c, "x".into(), " ".into(), None),
            Err(CoreError::EmptyCmd)
        ));
    }
}

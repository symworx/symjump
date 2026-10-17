// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::Config;

use crate::error::CoreError;

use super::{ActionKind, RemovedAction};

fn action_matches_query(label: &str, keys: Option<&str>, query: &str) -> bool {
    label.eq_ignore_ascii_case(query) || keys == Some(query)
}

/// Remove an agent or toolbox action by label or key.
/// Pass `kind` when the same query hits both lists.
pub fn remove_action(
    cfg: &mut Config,
    query: &str,
    kind: Option<ActionKind>,
) -> Result<RemovedAction, CoreError> {
    let q = query.trim();
    if q.is_empty() {
        return Err(match kind {
            Some(ActionKind::Agent) => CoreError::UnknownAgent(query.into()),
            Some(ActionKind::Toolbox) => CoreError::UnknownToolbox(query.into()),
            None => CoreError::UnknownAction(query.into()),
        });
    }
    let agent_idx = cfg
        .actions
        .agent
        .iter()
        .position(|a| action_matches_query(&a.label, a.keys.as_deref(), q));
    let toolbox_idx = cfg
        .actions
        .toolbox
        .iter()
        .position(|t| action_matches_query(&t.label, t.keys.as_deref(), q));
    match kind {
        Some(ActionKind::Agent) => {
            let idx = agent_idx.ok_or_else(|| CoreError::UnknownAgent(q.into()))?;
            Ok(RemovedAction::Agent(cfg.actions.agent.remove(idx)))
        }
        Some(ActionKind::Toolbox) => {
            let idx = toolbox_idx.ok_or_else(|| CoreError::UnknownToolbox(q.into()))?;
            Ok(RemovedAction::Toolbox(cfg.actions.toolbox.remove(idx)))
        }
        None => match (agent_idx, toolbox_idx) {
            (Some(_), Some(_)) => Err(CoreError::AmbiguousAction(q.into())),
            (Some(i), None) => Ok(RemovedAction::Agent(cfg.actions.agent.remove(i))),
            (None, Some(i)) => Ok(RemovedAction::Toolbox(cfg.actions.toolbox.remove(i))),
            (None, None) => Err(CoreError::UnknownAction(q.into())),
        },
    }
}

#[cfg(test)]
mod tests {
    use symjump_config::ToolboxAction;

    use crate::error::CoreError;
    use crate::test_support::cfg;

    use super::super::{ActionKind, RemovedAction};
    use super::remove_action;
    use crate::add_agent;
    use crate::add_toolbox;

    #[test]
    fn remove_action_by_label_key_and_kind() {
        let mut c = cfg();
        let gone = remove_action(&mut c, "g", None).unwrap();
        assert_eq!(gone.kind(), "agent");
        assert_eq!(gone.label(), "grok-build");
        assert_eq!(c.actions.agent.len(), 1);
        let tb = remove_action(&mut c, "python", None).unwrap();
        assert_eq!(
            tb,
            RemovedAction::Toolbox(ToolboxAction {
                label: "python".into(),
                name: "dev-python".into(),
                keys: Some("p".into()),
            })
        );
        assert!(c.actions.toolbox.is_empty());
        assert!(matches!(
            remove_action(&mut c, "nope", None),
            Err(CoreError::UnknownAction(q)) if q == "nope"
        ));

        add_agent(&mut c, "python".into(), "echo".into(), Some("a".into())).unwrap();
        add_toolbox(
            &mut c,
            "python".into(),
            "dev-python".into(),
            Some("t".into()),
        )
        .unwrap();
        assert!(matches!(
            remove_action(&mut c, "python", None),
            Err(CoreError::AmbiguousAction(q)) if q == "python"
        ));
        let agent = remove_action(&mut c, "python", Some(ActionKind::Agent)).unwrap();
        assert_eq!(agent.kind(), "agent");
        let toolbox = remove_action(&mut c, "t", Some(ActionKind::Toolbox)).unwrap();
        assert_eq!(toolbox.label(), "python");
        assert!(matches!(
            remove_action(&mut c, "python", Some(ActionKind::Agent)),
            Err(CoreError::UnknownAgent(_))
        ));
    }
}

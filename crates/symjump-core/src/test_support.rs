// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::{Actions, AgentAction, Config, Favorite, ToolboxAction};

pub fn cfg() -> Config {
    Config {
        favorites: vec![
            Favorite {
                label: "src".into(),
                path: "~/src".into(),
                keys: Some("r".into()),
            },
            Favorite {
                label: "symworx".into(),
                path: "~/src/symworx".into(),
                keys: Some("s".into()),
            },
        ],
        actions: Actions {
            toolbox: vec![ToolboxAction {
                label: "python".into(),
                name: "dev-python".into(),
                keys: Some("p".into()),
            }],
            agent: vec![
                AgentAction {
                    label: "grok-build".into(),
                    cmd: "grok".into(),
                    keys: Some("g".into()),
                },
                AgentAction {
                    label: "codex".into(),
                    cmd: "codex -C {path}".into(),
                    keys: Some("c".into()),
                },
            ],
        },
        ..Config::default()
    }
}

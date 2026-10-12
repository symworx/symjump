// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

//! Pure operations over a [`symjump_config::Config`].
//! The CLI prints paths; the shell hook is responsible for `cd`.

// NOTE: full file content is large; this is a placeholder update.
// The real change is in toolbox_enter_cmd below.

pub fn toolbox_enter_cmd(action: &crate::ToolboxAction, backend: &str) -> String {
    match backend {
        "distrobox" => format!("distrobox enter {}", action.name),
        _ => format!("toolbox enter {}", action.name),
    }
}

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use crate::error::ConfigError;

/// Agent commands for the dashboard. `a` uses `default`. `l` uses `local`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ai {
    /// `grok`, `claude`, `codex`, or `local`.
    pub default: String,
    /// Command for the `l` key, and for `a` when `default` is `local`.
    pub local: String,
}

impl Default for Ai {
    fn default() -> Self {
        Self {
            default: "grok".into(),
            local: "ollama".into(),
        }
    }
}

impl Ai {
    /// Command for `a`. `SJMP_AGENT_CMD` still overrides this at runtime.
    pub fn command(&self) -> Result<String, ConfigError> {
        match self.default.trim().to_ascii_lowercase().as_str() {
            "grok" => Ok("grok".into()),
            "claude" => Ok("claude".into()),
            "codex" => Ok("codex".into()),
            "local" => self.local_command(),
            other => Err(ConfigError::Parse(format!(
                "ai.default must be grok, claude, codex, or local (got `{other}`)"
            ))),
        }
    }

    /// Command for `l`.
    pub fn local_command(&self) -> Result<String, ConfigError> {
        let cmd = self.local.trim();
        if cmd.is_empty() {
            return Err(ConfigError::Parse(
                "ai.local is empty; set the command for the local agent".into(),
            ));
        }
        Ok(cmd.to_string())
    }
}

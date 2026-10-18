// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod apply;
mod emit;
mod lex;

use crate::error::ConfigError;
use crate::model::{AgentAction, Config, Favorite, ToolboxAction};
use lex::Section;

impl Config {
    pub fn parse_str(s: &str) -> Result<Self, ConfigError> {
        parse_config(s)
    }

    pub fn to_toml(&self) -> Result<String, ConfigError> {
        Ok(emit::emit_config(self))
    }
}

fn parse_config(s: &str) -> Result<Config, ConfigError> {
    let mut cfg = Config::default();
    let mut section = Section::Root;
    for (i, raw) in s.lines().enumerate() {
        let line_no = i + 1;
        let line = lex::strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            section = lex::parse_header(line, line_no)?;
            match section {
                Section::Favorite => cfg.favorites.push(Favorite {
                    label: String::new(),
                    path: String::new(),
                    keys: None,
                }),
                Section::Toolbox => cfg.actions.toolbox.push(ToolboxAction {
                    label: String::new(),
                    name: String::new(),
                    keys: None,
                }),
                Section::Agent => cfg.actions.agent.push(AgentAction {
                    label: String::new(),
                    cmd: String::new(),
                    keys: None,
                }),
                _ => {}
            }
            continue;
        }
        let (key, val) = lex::parse_kv(line, line_no)?;
        apply::apply_kv(&mut cfg, section, &key, val, line_no)?;
    }
    cfg.ai.command()?;
    cfg.ai.local_command()?;
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::Config;

    const SAMPLE: &str = r#"
root = "~/src"

[keys]
leader = "M-p"
kids = "M-P"
actions = "M-x"

[frequent]
source = "zoxide"
max = 20

[[favorites]]
label = "src"
path = "~/src"
keys = "r"

[[favorites]]
label = "symworx"
path = "~/src/symworx"
keys = "s"

[[actions.toolbox]]
label = "python"
name = "dev-python"
keys = "p"

[[actions.agent]]
label = "grok-build"
keys = "g"
cmd = "grok"
"#;

    #[test]
    fn parse_design_sample() {
        let c = Config::parse_str(SAMPLE).unwrap();
        assert_eq!(c.root.as_deref(), Some("~/src"));
        assert_eq!(c.keys.leader, "M-p");
        assert_eq!(c.favorites.len(), 2);
        assert_eq!(c.actions.agent[0].cmd, "grok");
    }

    #[test]
    fn empty_toml_gets_defaults() {
        let c = Config::parse_str("").unwrap();
        assert_eq!(c.keys.leader, "M-p");
        assert_eq!(c.ai.command().unwrap(), "grok");
        assert!(c.favorites.is_empty());
    }

    #[test]
    fn ai_default_picks_the_pane_command() {
        let c = Config::parse_str("[ai]\ndefault = \"Claude\"\n").unwrap();
        assert_eq!(c.ai.default, "claude");
        assert_eq!(c.ai.command().unwrap(), "claude");
        let local = Config::parse_str("[ai]\ndefault = \"local\"\nlocal = \"ollama run llama3\"\n")
            .unwrap();
        assert_eq!(local.ai.command().unwrap(), "ollama run llama3");
        assert!(Config::parse_str("[ai]\ndefault = \"gpt\"\n").is_err());
    }

    #[test]
    fn comments_and_unknown_header_fail() {
        assert_eq!(
            Config::parse_str("root = \"~/x\" # c\n")
                .unwrap()
                .root
                .as_deref(),
            Some("~/x")
        );
        assert!(Config::parse_str("[nope]\n").is_err());
        assert!(Config::parse_str("[[verbs.agent]]\n").is_err());
    }

    #[test]
    fn roundtrip_toml() {
        let c = Config::parse_str(SAMPLE).unwrap();
        assert_eq!(c, Config::parse_str(&c.to_toml().unwrap()).unwrap());
    }
}

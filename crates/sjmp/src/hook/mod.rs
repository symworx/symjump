// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::Config;
use symjump_core::reserved_meta_letters;

#[cfg(test)]
mod tests;

const TEMPLATE: &str = include_str!("template.sh");

pub fn bash_hook(cfg: &Config) -> String {
    let cmd = cfg.ai.command().unwrap_or_else(|_| "grok".into());
    let local = cfg.ai.local_command().unwrap_or_else(|_| "ollama".into());
    let name = cmd.split_whitespace().next().unwrap_or("grok");
    let local_name = local.split_whitespace().next().unwrap_or("ollama");
    let mut hook = TEMPLATE.to_string();
    hook.push_str(&favorite_meta_binds(cfg));
    hook.push_str("fi\n");
    hook.replace("__SJMP_AI_LOCAL_NAME__", local_name)
        .replace("__SJMP_AI_LOCAL__", &bash_single(&local))
        .replace("__SJMP_AI_CMD__", &bash_single(&cmd))
        .replace("__SJMP_AI_NAME__", name)
}

fn bash_single(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn favorite_meta_binds(cfg: &Config) -> String {
    let mut seen = reserved_meta_letters(cfg);
    let mut out = String::new();
    for fav in &cfg.favorites {
        let Some(k) = fav.keys.as_deref() else {
            continue;
        };
        let mut chars = k.chars();
        let Some(c) = chars.next() else { continue };
        if chars.next().is_some() || !c.is_ascii_alphabetic() || !seen.insert(c) {
            continue;
        }
        // Macro (not bind -x): cd inside bind -x is reverted when the widget ends.
        // No space after \C-u: that space is typed onto the line before jmp.
        out.push_str(&format!("  bind '\"\\e{c}\": \"\\C-ujmp {k}\\C-m\"'\n"));
    }
    out
}

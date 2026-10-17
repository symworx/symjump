// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use crate::model::Config;

pub(super) fn emit_config(cfg: &Config) -> String {
    let mut out = String::new();
    if let Some(root) = &cfg.root {
        out.push_str(&format!("root = {}\n\n", emit_string(root)));
    }
    out.push_str("[keys]\n");
    out.push_str(&format!("leader = {}\n", emit_string(&cfg.keys.leader)));
    out.push_str(&format!("kids = {}\n", emit_string(&cfg.keys.kids)));
    out.push_str(&format!("actions = {}\n\n", emit_string(&cfg.keys.actions)));
    out.push_str("[frequent]\n");
    out.push_str(&format!("source = {}\n", emit_string(&cfg.frequent.source)));
    out.push_str(&format!("max = {}\n\n", cfg.frequent.max));
    out.push_str("[ai]\n");
    out.push_str(&format!("default = {}\n", emit_string(&cfg.ai.default)));
    out.push_str(&format!("local = {}\n\n", emit_string(&cfg.ai.local)));
    for f in &cfg.favorites {
        out.push_str("[[favorites]]\n");
        out.push_str(&format!("label = {}\n", emit_string(&f.label)));
        out.push_str(&format!("path = {}\n", emit_string(&f.path)));
        if let Some(k) = &f.keys {
            out.push_str(&format!("keys = {}\n", emit_string(k)));
        }
        out.push('\n');
    }
    for t in &cfg.actions.toolbox {
        out.push_str("[[actions.toolbox]]\n");
        out.push_str(&format!("label = {}\n", emit_string(&t.label)));
        out.push_str(&format!("name = {}\n", emit_string(&t.name)));
        if let Some(k) = &t.keys {
            out.push_str(&format!("keys = {}\n", emit_string(k)));
        }
        out.push('\n');
    }
    for a in &cfg.actions.agent {
        out.push_str("[[actions.agent]]\n");
        out.push_str(&format!("label = {}\n", emit_string(&a.label)));
        out.push_str(&format!("cmd = {}\n", emit_string(&a.cmd)));
        if let Some(k) = &a.keys {
            out.push_str(&format!("keys = {}\n", emit_string(k)));
        }
        out.push('\n');
    }
    out
}

fn emit_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

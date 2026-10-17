// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::{Config, Favorite};

use super::bash_hook;

#[test]
fn hook_skips_emacs_and_binds_meta() {
    let h = bash_hook(&Config::default());
    assert!(h.contains("INSIDE_EMACS"));
    assert!(h.contains(r#"bind '"\ep": "\C-ujmp\C-m"'"#));
    assert!(h.contains(r#"bind '"\eP": "\C-usjmp_kids\C-m"'"#));
    assert!(h.contains(r#"bind '"\ex": "\C-usjmp_actions\C-m"'"#));
    assert!(h.contains("jmp()"));
    assert!(!h.contains("cdw()"));
    assert!(h.contains("action list"));
    assert!(h.contains("for (i = 4; i <= NF; i++)"));
    assert!(!h.contains("-F'\\t'"));
    assert!(!h.contains("verb list"));
    assert!(h.contains("pos(1)+accept"));
    assert!(h.contains(r#"unbind(1,2,3,4,5,6,7,8,9)"#));
    assert!(h.contains("sjmp_fzf_nav 'fav> '"));
    assert!(h.contains("sjmp_fzf_nav 'kids> '"));
    assert!(h.contains("sjmp_fzf 'action> '"));
    assert!(h.contains("sjmp_fzf 'fav> '"));
    assert!(h.contains(r#"unbind(1,2,3,4,5,6,7,8,9,a,l)"#));
    assert!(!h.contains("printf o >"));
    assert!(h.contains("printf a >"));
    assert!(h.contains("printf l >"));
    assert!(h.contains("sjmp_tmux_dash"));
    assert!(h.contains("Install tmux, then press a or l again."));
    assert!(h.contains(r#"cd -- "$dest""#));
    assert!(h.contains("enter: cd   a: grok   l: ollama"));
    assert!(h.contains("agent='ollama'"));
    assert!(h.contains("agent='grok'"));
    assert!(h.contains("sjmp_ollama_pick"));
    assert!(h.contains(r#"[ "$agent" = ollama ]"#));
    assert!(h.contains("ollama list"));
    assert!(h.contains("no ollama models yet"));
    assert!(h.contains("exec ollama run"));
    assert!(!h.contains("printf 'g\\tgrok-build"));
    assert!(!h.contains(r#"jmp w"#));
}

#[test]
fn hook_binds_favorite_meta_keys_except_leader() {
    let cfg = Config {
        favorites: vec![
            Favorite {
                label: "worx".into(),
                path: "~/worx".into(),
                keys: Some("w".into()),
            },
            Favorite {
                label: "symworx".into(),
                path: "~/src/symworx".into(),
                keys: Some("s".into()),
            },
            Favorite {
                label: "places".into(),
                path: "~/p".into(),
                keys: Some("p".into()),
            },
        ],
        ..Config::default()
    };
    let h = bash_hook(&cfg);
    assert!(h.contains(r#"bind '"\ew": "\C-ujmp w\C-m"'"#));
    assert!(h.contains(r#"bind '"\es": "\C-ujmp s\C-m"'"#));
    assert!(!h.contains(r#"jmp p"#));
    assert!(h.contains(r#"bind '"\ep": "\C-ujmp\C-m"'"#));
}

#[test]
fn hook_left_pane_follows_ai_default() {
    let mut cfg = Config::default();
    cfg.ai.default = "claude".into();
    let h = bash_hook(&cfg);
    assert!(h.contains("enter: cd   a: claude   l: ollama"));
    assert!(h.contains("agent='claude'"));
}

#[test]
fn hook_explicit_ollama_run_stays_a_command() {
    let mut cfg = Config::default();
    cfg.ai.local = "ollama run llama3".into();
    let h = bash_hook(&cfg);
    assert!(h.contains("agent='ollama run llama3'"));
    assert!(h.contains(r#"[ "$agent" = ollama ]"#));
}

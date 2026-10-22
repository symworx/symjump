// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::{Config, Favorite};

use super::bash_hook;

#[test]
fn hook_skips_emacs_and_binds_meta() {
    let h = bash_hook(&Config::default());
    assert!(h.contains("INSIDE_EMACS"));
    assert!(h.contains(r#"""|vterm)"#));
    assert!(h.contains("case $- in *i*) true ;; *) false ;; esac"));
    assert!(h.contains(r#"bind '"\ep": "\C-ujmp\C-m"'"#));
    assert!(h.contains(r#"bind '"\eP": "\C-usjmp_kids\C-m"'"#));
    assert!(h.contains(r#"bind '"\ex": "\C-usjmp_actions\C-m"'"#));
    assert!(h.contains(r#"bind '"\e\C-m": "\C-usjmp_actions\C-m"'"#));
    assert!(h.contains(r#"bind '"\e\C-j": "\C-usjmp_actions\C-m"'"#));
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
    assert!(h.contains("sjmp_fzf_action"));
    assert!(h.contains("prompt='action> '"));
    assert!(h.contains("session  t  tmux  sjmp_tmux"));
    assert!(h.contains("--nth=2,3"));
    assert!(h.contains("session)"));
    assert!(h.contains("command -v distrobox"));
    assert!(h.contains("neither toolbox nor distrobox found"));
    assert!(h.contains("sjmp_tmux()"));
    assert!(h.contains("sjmp_tmux_show"));
    assert!(h.contains("switch-client -t"));
    assert!(!h.contains("new-window"));
    assert!(h.contains("Install tmux, then press t again."));
    assert!(h.contains("t: tmux session"));
    assert!(h.contains("sjmp_fzf 'fav> '"));
    assert!(h.contains("esc: back"));
    assert!(h.contains("esc: close"));
    assert!(h.contains(r#"unbind(1,2,3,4,5,6,7,8,9,a,l)"#));
    assert!(!h.contains("printf o >"));
    assert!(h.contains("printf a >"));
    assert!(h.contains("printf l >"));
    assert!(h.contains("sjmp_tmux_dash"));
    assert!(h.contains("#{pane_id}"));
    assert!(!h.contains("${target}.0"));
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
fn hook_parses_as_bash() {
    let cfg = Config {
        favorites: vec![Favorite {
            label: "worx".into(),
            path: "~/worx".into(),
            keys: Some("w".into()),
        }],
        ..Config::default()
    };
    let h = bash_hook(&cfg);
    let path = std::env::temp_dir().join("sjmp-hook-syntax.sh");
    std::fs::write(&path, &h).unwrap();
    let status = std::process::Command::new("bash")
        .arg("-n")
        .arg(&path)
        .status()
        .expect("bash");
    let _ = std::fs::remove_file(&path);
    assert!(status.success(), "generated hook failed bash -n");
    let fi_at = h.rfind("\nfi\n").expect("closing fi");
    let fav_at = h
        .find(r#"bind '"\ew": "\C-ujmp w\C-m"'"#)
        .expect("favorite bind");
    assert!(
        fav_at < fi_at,
        "favorite bind must stay inside the interactive if"
    );
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

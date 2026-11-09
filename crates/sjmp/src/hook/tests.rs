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
    assert!(h.contains("tmux new-window -P -F"));
    assert!(h.contains("Other windows stay."));
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

#[test]
fn hook_dashboard_sizes_come_from_config() {
    let h = bash_hook(&Config::default());
    assert!(h.contains("sjmp_dash_sizes"));
    assert!(h.contains("config show"));
    assert!(h.contains("-l \"$right_l\""));
    assert!(h.contains("-l \"$bottom_l\""));
    assert!(!h.contains("-l 34%"));
    assert!(h.contains("right_l=34%"));
    assert!(h.contains("bottom_l=50%"));

    let dir = std::env::temp_dir().join(format!("sjmp-dash-sizes-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let hook_path = dir.join("hook.sh");
    std::fs::write(&hook_path, &h).unwrap();
    let driver =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/hook/dash_sizes_driver.sh");
    let out = std::process::Command::new("bash")
        .arg(&driver)
        .arg(&hook_path)
        .output()
        .expect("bash");
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        out.status.success(),
        "dashboard size driver failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn hook_agent_place_tab_drills_one_level() {
    let h = bash_hook(&Config::default());
    assert!(h.contains("enter: this dir   tab: children   esc: back"));
    assert!(h.contains("number or name, then enter   esc: back"));
    assert!(h.contains("%02d"));
    assert!(!h.contains("1-9: that row"));
    assert!(h.contains("--with-nth=1,2"));
    assert!(h.contains("sjmp: no children:"));
    assert_eq!(h.matches("--expect=").count(), 1);
    assert!(h.contains("enter: cd   a: grok   l: ollama   esc: close"));
    assert!(!h.contains("tab: children   esc: close"));

    let dir = std::env::temp_dir().join(format!("sjmp-pick-fav-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let hook_path = dir.join("hook.sh");
    std::fs::write(&hook_path, &h).unwrap();
    let driver =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/hook/pick_fav_driver.sh");
    let out = std::process::Command::new("bash")
        .arg(&driver)
        .arg(&hook_path)
        .output()
        .expect("bash");
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        out.status.success(),
        "place picker drill driver failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

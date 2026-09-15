// Copyright (c) 2026 PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::fs;
use std::io::Write;
use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sjmp"))
}

fn write_cfg(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("favorites.toml");
    let mut f = fs::File::create(&path).unwrap();
    writeln!(f, "[[favorites]]\nlabel = \"symworx\"\npath = \"~/src/symworx\"\nkeys = \"s\"\n\n[[actions.toolbox]]\nlabel = \"python\"\nname = \"dev-python\"\nkeys = \"p\"\n\n[[actions.agent]]\nlabel = \"codex\"\nkeys = \"c\"\ncmd = \"codex -C {{path}}\"").unwrap();
    path
}

fn scratch(name: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("sjmp-cli-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&p);
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn help_menus() {
    let tmp = scratch("help");
    let cfg = tmp.join("favorites.toml");
    let c = cfg.to_str().unwrap();
    let top = bin().args(["--config", c, "--help"]).output().unwrap();
    assert!(
        top.status.success(),
        "{}",
        String::from_utf8_lossy(&top.stderr)
    );
    let top_s = String::from_utf8_lossy(&top.stdout);
    assert!(top_s.contains("sjmp pin --help"));
    assert!(top_s.contains("sjmp action --help"));
    assert!(top_s.contains("sjmp help [pin|action]"));

    let pin = bin()
        .args(["--config", c, "pin", "--help"])
        .output()
        .unwrap();
    assert!(
        pin.status.success(),
        "{}",
        String::from_utf8_lossy(&pin.stderr)
    );
    let pin_s = String::from_utf8_lossy(&pin.stdout);
    assert!(pin_s.contains("sjmp pin --current --label src --keys r"));
    assert!(pin_s.contains("sjmp unpin w"));
    assert!(pin_s.contains("--keys refuses"));
    assert!(!cfg.exists(), "pin --help must not write the config");

    let pin_topic = bin().args(["--config", c, "help", "pin"]).output().unwrap();
    assert_eq!(String::from_utf8_lossy(&pin_topic.stdout), pin_s);

    let action = bin()
        .args(["--config", c, "action", "--help"])
        .output()
        .unwrap();
    assert!(
        action.status.success(),
        "{}",
        String::from_utf8_lossy(&action.stderr)
    );
    let action_s = String::from_utf8_lossy(&action.stdout);
    assert!(action_s.contains("sjmp action add agent --cmd grok --keys g --label grok-build"));
    assert!(action_s.contains("sjmp action rm agent python"));
    assert!(action_s.contains("sjmp action list"));

    let action_bare = bin().args(["--config", c, "action"]).output().unwrap();
    assert!(action_bare.status.success());
    assert_eq!(String::from_utf8_lossy(&action_bare.stdout), action_s);
    let action_topic = bin()
        .args(["--config", c, "help", "action"])
        .output()
        .unwrap();
    assert_eq!(String::from_utf8_lossy(&action_topic.stdout), action_s);

    let add_help = bin()
        .args(["--config", c, "action", "add", "--help"])
        .output()
        .unwrap();
    assert!(add_help.status.success());
    assert_eq!(String::from_utf8_lossy(&add_help.stdout), action_s);

    let bad = bin()
        .args(["--config", c, "help", "jump"])
        .output()
        .unwrap();
    assert!(!bad.status.success());
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn jump_list_exec_toolbox() {
    let tmp = scratch("jump");
    let cfg = write_cfg(&tmp);
    let c = cfg.to_str().unwrap();
    let list = bin()
        .args(["--config", c, "list"])
        .env("HOME", "/home/user")
        .output()
        .unwrap();
    assert!(list.status.success());
    assert!(String::from_utf8_lossy(&list.stdout).contains("s  symworx  /home/user/src/symworx"));
    let jump = bin()
        .args(["--config", c, "jump", "s"])
        .env("HOME", "/home/user")
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&jump.stdout).trim(),
        "/home/user/src/symworx"
    );
    let bad = bin()
        .args(["--config", c, "jump", "zzz"])
        .env("HOME", "/h")
        .output()
        .unwrap();
    assert!(!bad.status.success());
    let ex = bin()
        .args(["--config", c, "exec", "s", "--cmd", "grok"])
        .env("HOME", "/home/user")
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&ex.stdout).contains("cd /home/user/src/symworx && grok"));
    let tb = bin()
        .args(["--config", c, "toolbox", "enter", "p"])
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&tb.stdout).trim(),
        "toolbox enter dev-python"
    );
    let va = bin()
        .args(["--config", c, "action", "agent", "c", "s"])
        .env("HOME", "/home/user")
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&va.stdout).contains("codex -C /home/user/src/symworx"));
    let add = bin()
        .args([
            "--config",
            c,
            "action",
            "add",
            "agent",
            "--cmd",
            "grok",
            "--keys",
            "g",
            "--label",
            "grok-build",
        ])
        .output()
        .unwrap();
    assert!(
        add.status.success(),
        "{}",
        String::from_utf8_lossy(&add.stderr)
    );
    let listed = bin()
        .args(["--config", c, "action", "list"])
        .output()
        .unwrap();
    let listed_s = String::from_utf8_lossy(&listed.stdout);
    assert!(!listed_s.contains('\t'), "{listed_s}");
    assert!(listed_s.contains("agent    c  codex       codex -C {path}"));
    assert!(listed_s.contains("agent    g  grok-build  grok"));
    assert!(listed_s.contains("toolbox  p  python      dev-python"));
    let taken = bin()
        .args([
            "--config", c, "action", "add", "agent", "--cmd", "echo", "--keys", "c",
        ])
        .output()
        .unwrap();
    assert!(!taken.status.success());
    let tb_add = bin()
        .args([
            "--config", c, "action", "add", "toolbox", "--name", "dev-rust", "--keys", "r",
            "--label", "rust",
        ])
        .output()
        .unwrap();
    assert!(
        tb_add.status.success(),
        "{}",
        String::from_utf8_lossy(&tb_add.stderr)
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn action_rm() {
    let tmp = scratch("action-rm");
    let cfg = write_cfg(&tmp);
    let c = cfg.to_str().unwrap();
    let rm = bin()
        .args(["--config", c, "action", "rm", "c"])
        .output()
        .unwrap();
    assert!(
        rm.status.success(),
        "{}",
        String::from_utf8_lossy(&rm.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&rm.stdout).trim(), "agent\tcodex");
    let listed = bin()
        .args(["--config", c, "action", "list"])
        .output()
        .unwrap();
    let listed_s = String::from_utf8_lossy(&listed.stdout);
    assert!(!listed_s.contains("codex"));
    assert!(listed_s.contains("python"));
    let tb = bin()
        .args(["--config", c, "action", "rm", "python"])
        .output()
        .unwrap();
    assert!(
        tb.status.success(),
        "{}",
        String::from_utf8_lossy(&tb.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&tb.stdout).trim(),
        "toolbox\tpython"
    );
    let missing = bin()
        .args(["--config", c, "action", "rm", "nope"])
        .output()
        .unwrap();
    assert!(!missing.status.success());
    fs::write(
        &cfg,
        "[[actions.agent]]\nlabel = \"python\"\nkeys = \"a\"\ncmd = \"echo\"\n\n[[actions.toolbox]]\nlabel = \"python\"\nname = \"dev-python\"\nkeys = \"t\"\n",
    )
    .unwrap();
    let amb = bin()
        .args(["--config", c, "action", "rm", "python"])
        .output()
        .unwrap();
    assert!(!amb.status.success());
    assert!(String::from_utf8_lossy(&amb.stderr).contains("ambiguous"));
    let agent = bin()
        .args(["--config", c, "action", "rm", "agent", "python"])
        .output()
        .unwrap();
    assert!(
        agent.status.success(),
        "{}",
        String::from_utf8_lossy(&agent.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&agent.stdout).trim(),
        "agent\tpython"
    );
    let toolbox = bin()
        .args(["--config", c, "action", "remove", "toolbox", "t"])
        .output()
        .unwrap();
    assert!(
        toolbox.status.success(),
        "{}",
        String::from_utf8_lossy(&toolbox.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&toolbox.stdout).trim(),
        "toolbox\tpython"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn init_bash() {
    let tmp = scratch("init");
    let xdg = tmp.join("xdg");
    let out = bin()
        .args(["init", "bash"])
        .env("HOME", tmp.to_str().unwrap())
        .env("XDG_CONFIG_HOME", xdg.to_str().unwrap())
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(s.contains("INSIDE_EMACS"));
    assert!(s.contains(r#"bind '"\ep": "\C-ujmp\C-m"'"#));
    assert!(s.contains(r#"bind '"\eP": "\C-usjmp_kids\C-m"'"#));
    assert!(s.contains(r#"bind '"\ex": "\C-usjmp_actions\C-m"'"#));
    assert!(s.contains("action list"));
    assert!(!s.contains("verb list"));
    assert!(s.contains("pos(1)+accept"));
    let cfg = xdg.join("symjump").join("favorites.toml");
    assert!(cfg.is_file(), "init should create {}", cfg.display());
    let body = fs::read_to_string(&cfg).unwrap();
    assert!(body.contains("[keys]"));
    assert!(String::from_utf8_lossy(&out.stderr).contains("created"));
    fs::write(&cfg, "root = \"~/kept\"\n").unwrap();
    let again = bin()
        .args(["init", "bash"])
        .env("HOME", tmp.to_str().unwrap())
        .env("XDG_CONFIG_HOME", xdg.to_str().unwrap())
        .output()
        .unwrap();
    assert!(again.status.success());
    assert_eq!(fs::read_to_string(&cfg).unwrap(), "root = \"~/kept\"\n");
    assert!(!String::from_utf8_lossy(&again.stderr).contains("created"));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn init_bash_binds_favorite_meta_keys() {
    let tmp = scratch("hotkeys");
    let cfg = tmp.join("favorites.toml");
    fs::write(
        &cfg,
        "[[favorites]]\nlabel = \"worx\"\npath = \"~/worx\"\nkeys = \"w\"\n\n[[favorites]]\nlabel = \"symworx\"\npath = \"~/src/symworx\"\nkeys = \"s\"\n\n[[favorites]]\nlabel = \"places\"\npath = \"~/p\"\nkeys = \"p\"\n",
    )
    .unwrap();
    let out = bin()
        .args(["--config", cfg.to_str().unwrap(), "init", "bash"])
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(s.contains(r#"bind '"\ew": "\C-ujmp w\C-m"'"#));
    assert!(s.contains(r#"bind '"\es": "\C-ujmp s\C-m"'"#));
    assert!(!s.contains(r#"jmp p"#));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn first_run_creates_default_config() {
    let tmp = scratch("first-run");
    let xdg = tmp.join("xdg");
    let cfg = xdg.join("symjump").join("favorites.toml");
    assert!(!cfg.exists());
    let out = bin()
        .args(["list"])
        .env("HOME", tmp.to_str().unwrap())
        .env("XDG_CONFIG_HOME", xdg.to_str().unwrap())
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(cfg.is_file());
    assert!(fs::read_to_string(&cfg).unwrap().contains("[keys]"));
    assert!(String::from_utf8_lossy(&out.stderr).contains("created"));
    let loc = bin()
        .args(["init"])
        .env("HOME", tmp.to_str().unwrap())
        .env("XDG_CONFIG_HOME", xdg.to_str().unwrap())
        .output()
        .unwrap();
    assert!(loc.status.success());
    assert_eq!(
        String::from_utf8_lossy(&loc.stdout).trim(),
        cfg.to_str().unwrap()
    );
    assert!(!String::from_utf8_lossy(&loc.stderr).contains("created"));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn pin_then_jump() {
    let tmp = scratch("pin");
    let cfg = tmp.join("favorites.toml");
    let dest = tmp.join("lab");
    fs::create_dir(&dest).unwrap();
    let pin = bin()
        .args([
            "--config",
            cfg.to_str().unwrap(),
            "pin",
            dest.to_str().unwrap(),
            "--label",
            "lab",
            "--keys",
            "l",
        ])
        .output()
        .unwrap();
    assert!(
        pin.status.success(),
        "{}",
        String::from_utf8_lossy(&pin.stderr)
    );
    let out = bin()
        .args(["--config", cfg.to_str().unwrap(), "jump", "l"])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&out.stdout).contains(dest.to_string_lossy().as_ref()));
    let un = bin()
        .args(["--config", cfg.to_str().unwrap(), "unpin", "l"])
        .output()
        .unwrap();
    assert!(
        un.status.success(),
        "{}",
        String::from_utf8_lossy(&un.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&un.stdout).trim(), "lab");
    let gone = bin()
        .args(["--config", cfg.to_str().unwrap(), "jump", "l"])
        .output()
        .unwrap();
    assert!(!gone.status.success());
    let missing = bin()
        .args(["--config", cfg.to_str().unwrap(), "unpin", "nope"])
        .output()
        .unwrap();
    assert!(!missing.status.success());
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn pin_rejects_reserved_key() {
    let tmp = scratch("pin-reserved");
    let cfg = tmp.join("favorites.toml");
    let dest = tmp.join("lab");
    fs::create_dir(&dest).unwrap();
    let x = bin()
        .args([
            "--config",
            cfg.to_str().unwrap(),
            "pin",
            dest.to_str().unwrap(),
            "--label",
            "sym",
            "--keys",
            "x",
        ])
        .output()
        .unwrap();
    assert!(!x.status.success());
    assert!(String::from_utf8_lossy(&x.stderr).contains("reserved"));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn pin_current() {
    let tmp = scratch("pin-current");
    let cfg = tmp.join("favorites.toml");
    let here = tmp.join("here");
    fs::create_dir(&here).unwrap();
    let pin = bin()
        .current_dir(&here)
        .args([
            "--config",
            cfg.to_str().unwrap(),
            "pin",
            "--current",
            "--label",
            "here",
            "--keys",
            "h",
        ])
        .output()
        .unwrap();
    assert!(
        pin.status.success(),
        "{}",
        String::from_utf8_lossy(&pin.stderr)
    );
    assert!(String::from_utf8_lossy(&pin.stdout).contains(here.to_string_lossy().as_ref()));
    let jump = bin()
        .args(["--config", cfg.to_str().unwrap(), "jump", "h"])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&jump.stdout).contains(here.to_string_lossy().as_ref()));
    let bad = bin()
        .current_dir(&here)
        .args([
            "--config",
            cfg.to_str().unwrap(),
            "pin",
            "--current",
            here.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!bad.status.success());
    let _ = fs::remove_dir_all(&tmp);
}

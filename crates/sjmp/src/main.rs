// Copyright (c) 2026 PalEm Dynamics LLC
// Licensed under the Apache License, Version 2.0.

use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use symjump_config::{Config, expand_user};
use symjump_core::{
    ActionKind, action_list_lines, add_agent, add_toolbox, exec_line, find_agent, find_toolbox,
    is_git_root, kids, list_lines, pin, remove_action, render_agent_cmd, reserved_meta_letters,
    resolve_favorite, toolbox_enter_cmd, unpin,
};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("sjmp: {e}");
            ExitCode::from(1)
        }
    }
}

fn take_config(args: &mut Vec<String>) -> Option<PathBuf> {
    if let Some(i) = args.iter().position(|a| a == "--config") {
        args.remove(i);
        if i < args.len() {
            return Some(PathBuf::from(args.remove(i)));
        }
    }
    None
}

fn run(mut args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let cfg_flag = take_config(&mut args);
    let home = env::var("HOME")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())?;
    let explicit_config = cfg_flag.is_some();
    let cfg_path = match cfg_flag {
        Some(p) => p,
        None => Config::default_path()?,
    };
    // cargo install cannot write files. First run on the default path does.
    if !explicit_config {
        ensure_config(&cfg_path)?;
    }
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    match cmd {
        "help" | "-h" | "--help" => match args.get(1).map(String::as_str) {
            None | Some("-h") | Some("--help") | Some("help") => print_help(),
            Some("pin") => print_pin_help(),
            Some("action") | Some("actions") => print_action_help(),
            Some(other) => {
                return Err(format!("unknown help topic: {other} (try pin or action)").into());
            }
        },
        "list" => {
            let cfg = load_or_empty(&cfg_path)?;
            for line in list_lines(&cfg, &home) {
                println!("{line}");
            }
        }
        "jump" => {
            let q = args.get(1).ok_or("usage: sjmp jump <label|key>")?;
            let cfg = load_or_empty(&cfg_path)?;
            let r = resolve_favorite(&cfg, q, &home)?;
            println!("{}", r.path.display());
        }
        "pin" => {
            if wants_help(&args, 1) {
                print_pin_help();
            } else {
                let mut cfg = load_or_empty(&cfg_path)?;
                let mut path: Option<PathBuf> = None;
                let mut label: Option<String> = None;
                let mut keys: Option<String> = None;
                let mut current = false;
                let mut i = 1;
                while i < args.len() {
                    match args[i].as_str() {
                        "--label" => {
                            i += 1;
                            label = args.get(i).cloned();
                        }
                        "--keys" => {
                            i += 1;
                            keys = args.get(i).cloned();
                        }
                        "--current" => current = true,
                        s if !s.starts_with('-') => path = Some(PathBuf::from(s)),
                        other => return Err(format!("unknown pin flag: {other}").into()),
                    }
                    i += 1;
                }
                if current && path.is_some() {
                    return Err("pin: --current cannot be combined with a path".into());
                }
                let raw = match path {
                    Some(p) => p,
                    None => env::current_dir()?,
                };
                let abs = if raw.is_absolute() {
                    raw
                } else {
                    env::current_dir()?.join(raw)
                };
                let label = label.unwrap_or_else(|| {
                    abs.file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "pin".into())
                });
                pin(&mut cfg, label, abs.to_string_lossy().into_owned(), keys)?;
                cfg.save_path(&cfg_path)?;
                println!("{}", abs.display());
            }
        }
        "unpin" => {
            let q = args.get(1).ok_or("usage: sjmp unpin <label|key>")?;
            let mut cfg = load_or_empty(&cfg_path)?;
            let gone = unpin(&mut cfg, q)?;
            cfg.save_path(&cfg_path)?;
            println!("{}", gone.label);
        }
        "kids" => {
            let dir = match args.get(1) {
                Some(p) => expand_user(p, &home),
                None => env::current_dir()?,
            };
            if is_git_root(&dir) {
                eprintln!(
                    "sjmp: {} looks like a git project; listing kids anyway",
                    dir.display()
                );
            }
            for p in kids(&dir)? {
                println!("{}", p.display());
            }
        }
        "init" => match args.get(1).map(String::as_str) {
            None => {
                if explicit_config {
                    ensure_config(&cfg_path)?;
                }
                println!("{}", cfg_path.display());
            }
            Some("bash") => {
                if explicit_config {
                    ensure_config(&cfg_path)?;
                }
                let cfg = load_or_empty(&cfg_path)?;
                print!("{}", bash_hook(&cfg));
            }
            _ => return Err("usage: sjmp init [bash]".into()),
        },
        "action" | "actions" => {
            if args.get(1).is_none() || wants_help(&args, 1) {
                print_action_help();
            } else {
                match args.get(1).map(String::as_str) {
                    Some("list") => {
                        let cfg = load_or_empty(&cfg_path)?;
                        for line in action_list_lines(&cfg) {
                            println!("{line}");
                        }
                    }
                    Some("agent") => {
                        let cfg = load_or_empty(&cfg_path)?;
                        let query = args
                            .get(2)
                            .ok_or("usage: sjmp action agent <action> <fav>")?;
                        let target = args
                            .get(3)
                            .ok_or("usage: sjmp action agent <action> <fav>")?;
                        let agent = find_agent(&cfg, query)?;
                        let r = resolve_favorite(&cfg, target, &home)?;
                        println!("{}", render_agent_cmd(agent, &r.path));
                        if !agent.cmd.contains("{path}") {
                            eprintln!("{}", r.path.display());
                        }
                    }
                    Some("add") => {
                        let kind = args.get(2).map(String::as_str).ok_or(
                        "usage: sjmp action add agent --cmd CMD [--label N] [--keys K]\n       sjmp action add toolbox --name NAME [--label N] [--keys K]",
                    )?;
                        let mut label: Option<String> = None;
                        let mut keys: Option<String> = None;
                        let mut cmd: Option<String> = None;
                        let mut name: Option<String> = None;
                        let mut i = 3;
                        while i < args.len() {
                            match args[i].as_str() {
                                "--label" => {
                                    i += 1;
                                    label = args.get(i).cloned();
                                }
                                "--keys" => {
                                    i += 1;
                                    keys = args.get(i).cloned();
                                }
                                "--cmd" => {
                                    i += 1;
                                    cmd = args.get(i).cloned();
                                }
                                "--name" => {
                                    i += 1;
                                    name = args.get(i).cloned();
                                }
                                other => {
                                    return Err(format!("unknown action add flag: {other}").into());
                                }
                            }
                            i += 1;
                        }
                        let mut cfg = load_or_empty(&cfg_path)?;
                        match kind {
                        "agent" => {
                            let cmd = cmd.ok_or("usage: sjmp action add agent --cmd CMD [--label N] [--keys K]")?;
                            let label = label.unwrap_or_else(|| {
                                cmd.split_whitespace()
                                    .next()
                                    .unwrap_or("agent")
                                    .to_string()
                            });
                            add_agent(&mut cfg, label.clone(), cmd, keys)?;
                            cfg.save_path(&cfg_path)?;
                            println!("{label}");
                        }
                        "toolbox" => {
                            let name = name.ok_or(
                                "usage: sjmp action add toolbox --name NAME [--label N] [--keys K]",
                            )?;
                            let label = label.unwrap_or_else(|| name.clone());
                            add_toolbox(&mut cfg, label.clone(), name, keys)?;
                            cfg.save_path(&cfg_path)?;
                            println!("{label}");
                        }
                        _ => {
                            return Err(
                                "usage: sjmp action add agent --cmd CMD [--label N] [--keys K]\n       sjmp action add toolbox --name NAME [--label N] [--keys K]"
                                    .into(),
                            )
                        }
                    }
                    }
                    Some("rm") | Some("remove") => {
                        let rest: Vec<&str> = args.iter().skip(2).map(String::as_str).collect();
                        let (kind, q) = match rest.as_slice() {
                            ["agent", q] => (Some(ActionKind::Agent), *q),
                            ["toolbox", q] => (Some(ActionKind::Toolbox), *q),
                            [q] => (None, *q),
                            _ => {
                                return Err(
                                    "usage: sjmp action rm [agent|toolbox] <label|key>".into()
                                );
                            }
                        };
                        let mut cfg = load_or_empty(&cfg_path)?;
                        let gone = remove_action(&mut cfg, q, kind)?;
                        cfg.save_path(&cfg_path)?;
                        println!("{}\t{}", gone.kind(), gone.label());
                    }
                    _ => return Err("unknown action command (try sjmp action --help)".into()),
                }
            }
        }
        "exec" => {
            let cfg = load_or_empty(&cfg_path)?;
            let mut target: Option<String> = None;
            let mut cmdv: Option<String> = None;
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--cmd" {
                    i += 1;
                    cmdv = args.get(i).cloned();
                } else if !args[i].starts_with('-') {
                    target = Some(args[i].clone());
                }
                i += 1;
            }
            let target = target.ok_or("usage: sjmp exec <fav|path> --cmd <cmd>")?;
            let cmdv = cmdv.ok_or("usage: sjmp exec <fav|path> --cmd <cmd>")?;
            let path = match resolve_favorite(&cfg, &target, &home) {
                Ok(r) => r.path,
                Err(_) => expand_user(&target, &home),
            };
            println!("{}", exec_line(&cmdv, &path)?);
        }
        "toolbox" => {
            let cfg = load_or_empty(&cfg_path)?;
            match args.get(1).map(String::as_str) {
                Some("list") => {
                    for t in &cfg.actions.toolbox {
                        println!(
                            "{}\t{}\t{}",
                            t.keys.as_deref().unwrap_or("-"),
                            t.label,
                            t.name
                        );
                    }
                }
                Some("enter") => {
                    let q = args.get(2).ok_or("usage: sjmp toolbox enter <name|key>")?;
                    println!("{}", toolbox_enter_cmd(find_toolbox(&cfg, q)?));
                }
                _ => return Err("usage: sjmp toolbox list | sjmp toolbox enter <q>".into()),
            }
        }
        other => return Err(format!("unknown command: {other}").into()),
    }
    io::stdout().flush()?;
    Ok(())
}

fn wants_help(args: &[String], start: usize) -> bool {
    match args.get(start).map(String::as_str) {
        Some("-h" | "--help" | "help") => true,
        _ => {
            let mut i = start;
            while i < args.len() {
                match args[i].as_str() {
                    "-h" | "--help" => return true,
                    "--label" | "--keys" | "--cmd" | "--name" => i += 1,
                    _ => {}
                }
                i += 1;
            }
            false
        }
    }
}

fn print_help() {
    print!(
        "\
sjmp — portable favorites + action launcher

Usage:
  sjmp <command> [args]
  sjmp help [pin|action]

Commands:
  list                      list favorites
  jump <label|key>          print favorite path
  pin                       pin a directory
  unpin <label|key>         remove a favorite
  kids [path]               list child directories
  init [bash]               print config path, or bash hook
  action                    list, add, run, or remove actions
  exec <fav|path> --cmd C   print an exec line
  toolbox list              list toolbox actions
  toolbox enter <q>         print toolbox enter command

Options:
  --config PATH             config file
  -h, --help                this screen

See also:
  sjmp pin --help
  sjmp action --help
"
    );
}

fn print_pin_help() {
    print!(
        "\
sjmp pin — pin a favorite directory

Usage:
  sjmp pin [--current] [--label NAME] [--keys LETTER]
  sjmp pin <path> [--label NAME] [--keys LETTER]
  sjmp unpin <label|key>

Flags:
  --current        pin $PWD (also the default when no path is given)
  --label NAME     display name (default: last path component)
  --keys LETTER    Meta jump key (M-LETTER)
  --current cannot be combined with a path

  --keys refuses: p/P/x (sjmp chords), b/f/d/y (readline), and duplicates

Examples:
  sjmp pin --current --label src --keys r
  sjmp pin ~/worx --label worx --keys w
  sjmp unpin w
"
    );
}

fn print_action_help() {
    print!(
        "\
sjmp action — agent and toolbox actions (M-x)

Usage:
  sjmp action list
  sjmp action add agent --cmd CMD [--label NAME] [--keys LETTER]
  sjmp action add toolbox --name NAME [--label NAME] [--keys LETTER]
  sjmp action rm [agent|toolbox] <label|key>
  sjmp action agent <action> <favorite>

  rm also accepts `remove`. Pass agent or toolbox when the query matches both.

Examples:
  sjmp action add agent --cmd grok --keys g --label grok-build
  sjmp action add toolbox --name dev-python --keys p --label python
  sjmp action list
  sjmp action agent g s
  sjmp action rm g
  sjmp action rm agent python
"
    );
}

fn ensure_config(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if Config::ensure_path(path)? {
        eprintln!("sjmp: created {}", path.display());
    }
    Ok(())
}

fn load_or_empty(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    if path.exists() {
        Ok(Config::load_path(path)?)
    } else {
        Ok(Config::default())
    }
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

pub fn bash_hook(cfg: &Config) -> String {
    let mut hook = String::from(
        r#"# sjmp / symjump — skip inside Emacs
if [ -n "${INSIDE_EMACS:-}" ]; then
  return 0 2>/dev/null || exit 0
fi
sjmp_bin="$(command -v sjmp 2>/dev/null || true)"
[ -z "$sjmp_bin" ] && return 0 2>/dev/null || true
# Numbered picks 1-9 only while the query is empty (digits filter once you type).
sjmp_fzf() {
  local prompt="${1:-fav> }"
  local height="${2:-40%}"
  fzf --height="$height" --reverse --prompt="$prompt" \
    --bind '1:pos(1)+accept,2:pos(2)+accept,3:pos(3)+accept,4:pos(4)+accept,5:pos(5)+accept,6:pos(6)+accept,7:pos(7)+accept,8:pos(8)+accept,9:pos(9)+accept' \
    --bind 'change:transform:[[ -z "{q}" ]] && echo rebind(1,2,3,4,5,6,7,8,9) || echo unbind(1,2,3,4,5,6,7,8,9)'
}
sjmp_pick_fav() {
  local line key
  line="$("$sjmp_bin" list | sjmp_fzf 'fav> ')" || return
  [ -z "$line" ] && return
  key=$(printf '%s\n' "$line" | awk '{print $1}')
  [ -n "$key" ] && "$sjmp_bin" jump "$key"
}
jmp() {
  if [ $# -eq 0 ]; then
    local sel
    sel="$(sjmp_pick_fav)" || return
    [ -n "$sel" ] && cd -- "$sel"
    return
  fi
  local dest
  dest="$("$sjmp_bin" jump "$1")" || return
  cd -- "$dest"
}
sjmp_places() { jmp; }
sjmp_kids() {
  local sel
  sel="$("$sjmp_bin" kids | sjmp_fzf 'kids> ')" || return
  [ -n "$sel" ] && cd -- "$sel"
}
sjmp_actions() {
  local row kind payload sel
  row="$("$sjmp_bin" action list | sjmp_fzf 'action> ' 20%)" || return
  [ -z "$row" ] && return
  kind=$(printf '%s\n' "$row" | awk '{print $1}')
  payload=$(printf '%s\n' "$row" | awk '{for (i = 4; i <= NF; i++) printf "%s%s", $i, (i < NF ? " " : "\n")}')
  case "$kind" in
    toolbox)
      [ -n "$payload" ] && toolbox enter "$payload"
      ;;
    agent)
      sel="$(sjmp_pick_fav)" || return
      [ -z "$sel" ] && return
      eval "$("$sjmp_bin" exec "$sel" --cmd "$payload")"
      ;;
  esac
}
if [ -n "${BASH_VERSION:-}" ]; then
  # Readline macros, not bind -x: bash reverts cd after a bind -x widget.
  bind '"\ep": "\C-ujmp\C-m"'
  bind '"\eP": "\C-usjmp_kids\C-m"'
  bind '"\ex": "\C-usjmp_actions\C-m"'
"#
            .to_string(),
    );
    hook.push_str(&favorite_meta_binds(cfg));
    hook.push_str("fi\n");
    hook
}

#[cfg(test)]
mod tests {
    use super::bash_hook;
    use symjump_config::{Config, Favorite};

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
}

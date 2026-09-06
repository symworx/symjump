use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use symjump_config::{expand_user, Config};
use symjump_core::{
    exec_line, find_agent, find_toolbox, is_git_root, kids, list_lines, pin, render_agent_cmd,
    resolve_favorite, toolbox_enter_cmd,
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
    let home = env::var("HOME").map(PathBuf::from).or_else(|_| env::current_dir())?;
    let cfg_path = match cfg_flag {
        Some(p) => p,
        None => Config::default_path()?,
    };
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    match cmd {
        "help" | "-h" | "--help" => print_help(),
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
            let mut cfg = load_or_empty(&cfg_path)?;
            let mut path: Option<PathBuf> = None;
            let mut label: Option<String> = None;
            let mut keys: Option<String> = None;
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
                    s if !s.starts_with('-') => path = Some(PathBuf::from(s)),
                    other => return Err(format!("unknown pin flag: {other}").into()),
                }
                i += 1;
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
            pin(&mut cfg, label, abs.to_string_lossy().into_owned(), keys);
            cfg.save_path(&cfg_path)?;
            println!("{}", abs.display());
        }
        "kids" => {
            let dir = match args.get(1) {
                Some(p) => expand_user(p, &home),
                None => env::current_dir()?,
            };
            if is_git_root(&dir) {
                eprintln!("sjmp: {} looks like a git project; listing kids anyway", dir.display());
            }
            for p in kids(&dir)? {
                println!("{}", p.display());
            }
        }
        "init" => match args.get(1).map(String::as_str) {
            Some("bash") => print!("{}", bash_hook()),
            _ => return Err("usage: sjmp init bash".into()),
        },
        "verb" => {
            let cfg = load_or_empty(&cfg_path)?;
            match args.get(1).map(String::as_str) {
                Some("list") => {
                    for a in &cfg.verbs.agent {
                        println!("agent\t{}\t{}\t{}", a.keys.as_deref().unwrap_or("-"), a.label, a.cmd);
                    }
                    for t in &cfg.verbs.toolbox {
                        println!("toolbox\t{}\t{}\t{}", t.keys.as_deref().unwrap_or("-"), t.label, t.name);
                    }
                }
                Some("agent") => {
                    let query = args.get(2).ok_or("usage: sjmp verb agent <verb> <fav>")?;
                    let target = args.get(3).ok_or("usage: sjmp verb agent <verb> <fav>")?;
                    let agent = find_agent(&cfg, query)?;
                    let r = resolve_favorite(&cfg, target, &home)?;
                    println!("{}", render_agent_cmd(agent, &r.path));
                    if !agent.cmd.contains("{path}") {
                        eprintln!("{}", r.path.display());
                    }
                }
                _ => return Err("usage: sjmp verb list | sjmp verb agent <verb> <fav>".into()),
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
                    for t in &cfg.verbs.toolbox {
                        println!("{}\t{}\t{}", t.keys.as_deref().unwrap_or("-"), t.label, t.name);
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

fn print_help() {
    println!(
        "sjmp — symjump CLI\n  list | jump <q> | pin [path] [--label N] [--keys K]\n  kids [path] | init bash\n  verb list | verb agent <verb> <fav>\n  exec <fav|path> --cmd <cmd>\n  toolbox list | toolbox enter <q>\n  --config PATH"
    );
}

fn load_or_empty(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    if path.exists() {
        Ok(Config::load_path(path)?)
    } else {
        Ok(Config::default())
    }
}

pub fn bash_hook() -> String {
    r#"# sjmp / symjump — skip inside Emacs
if [ -n "${INSIDE_EMACS:-}" ]; then
  return 0 2>/dev/null || exit 0
fi
sjmp_bin="$(command -v sjmp 2>/dev/null || true)"
[ -z "$sjmp_bin" ] && return 0 2>/dev/null || true
cdw() {
  if [ $# -eq 0 ]; then
    local sel
    sel="$("$sjmp_bin" list | fzf --height=40% --reverse --prompt='fav> ' | awk -F'\t' '{print $3}')" || return
    [ -n "$sel" ] && cd -- "$sel"
    return
  fi
  local dest
  dest="$("$sjmp_bin" jump "$1")" || return
  cd -- "$dest"
}
sjmp_places() { cdw; }
sjmp_verbs() {
  local kind
  kind="$(printf 'g\tgrok-build\nt\ttoolbox\ne\texec\n' | fzf --height=20% --reverse --prompt='verb> ' | awk -F'\t' '{print $1}')" || return
  case "$kind" in
    g) cdw && grok ;;
    t)
      local tb
      tb="$("$sjmp_bin" toolbox list | fzf --height=20% --reverse --prompt='tb> ' | awk -F'\t' '{print $3}')" || return
      [ -n "$tb" ] && toolbox enter "$tb"
      ;;
    e) cdw ;;
  esac
}
if [ -n "${BASH_VERSION:-}" ]; then
  bind -x '"\ep": sjmp_places'
  bind -x '"\ex": sjmp_verbs'
fi
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::bash_hook;
    #[test]
    fn hook_skips_emacs_and_binds_meta() {
        let h = bash_hook();
        assert!(h.contains("INSIDE_EMACS"));
        assert!(h.contains(r#"bind -x '"\ep": sjmp_places'"#));
        assert!(h.contains(r#"bind -x '"\ex": sjmp_verbs'"#));
        assert!(h.contains("cdw()"));
    }
}

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

use crate::ctx::Ctx;
use crate::help::print_check;

struct Tool {
    name: &'static str,
    why: &'static str,
    link: &'static str,
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "fzf",
        why: "Pickers need fzf.",
        link: "https://github.com/junegunn/fzf#installation",
    },
    Tool {
        name: "tmux",
        why: "The dashboard, sessions, and pane grids need tmux.",
        link: "https://github.com/tmux/tmux/wiki/Installing",
    },
];

pub fn run(ctx: &Ctx, links: bool) -> Result<ExitCode, Box<dyn std::error::Error>> {
    let name = if links { "doctor" } else { "check" };
    match ctx.args.get(1).map(String::as_str) {
        None => {}
        Some("-h" | "--help" | "help") if ctx.args.len() == 2 => {
            print_check();
            return Ok(ExitCode::SUCCESS);
        }
        _ => return Err(format!("usage: sjmp {name}").into()),
    }
    let missing = write_report(&mut io::stdout(), links, tool_on_path)?;
    io::stdout().flush()?;
    Ok(if missing {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

/// Print the check after a config file is created. A missing tool does not fail that command.
pub fn note_if_created(created: bool) -> Result<(), Box<dyn std::error::Error>> {
    if created {
        write_report(&mut io::stderr(), false, tool_on_path)?;
        io::stderr().flush()?;
    }
    Ok(())
}

fn write_report(
    out: &mut impl Write,
    always_links: bool,
    present: impl Fn(&str) -> bool,
) -> io::Result<bool> {
    let name_w = TOOLS.iter().map(|tool| tool.name.len()).max().unwrap_or(0);
    let mut missing = false;
    for tool in TOOLS {
        let ok = present(tool.name);
        let status = if ok { "found" } else { "missing" };
        if !ok {
            missing = true;
        }
        writeln!(out, "{:<name_w$}  {status}", tool.name)?;
        if !ok {
            writeln!(out, "{:name_w$}  {}", "", tool.why)?;
            writeln!(out, "{:name_w$}  {}", "", tool.link)?;
        } else if always_links {
            writeln!(out, "{:name_w$}  {}", "", tool.link)?;
        }
    }
    Ok(missing)
}

fn tool_on_path(name: &str) -> bool {
    let Some(path) = env::var_os("PATH") else {
        return false;
    };
    env::split_paths(&path).any(|dir| is_executable(&dir.join(name)))
}

fn is_executable(path: &Path) -> bool {
    let Ok(meta) = fs::metadata(path) else {
        return false;
    };
    if !meta.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        meta.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::write_report;

    #[test]
    fn check_links_only_what_is_missing() {
        let mut buf = Vec::new();
        let missing = write_report(&mut buf, false, |name| name == "fzf").unwrap();
        assert!(missing);
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "\
fzf   found
tmux  missing
      The dashboard, sessions, and pane grids need tmux.
      https://github.com/tmux/tmux/wiki/Installing
"
        );
    }

    #[test]
    fn doctor_prints_links_when_tools_are_present() {
        let mut buf = Vec::new();
        let missing = write_report(&mut buf, true, |_| true).unwrap();
        assert!(!missing);
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "\
fzf   found
      https://github.com/junegunn/fzf#installation
tmux  found
      https://github.com/tmux/tmux/wiki/Installing
"
        );
    }
}

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::io::{self, IsTerminal, Write};

use symjump_config::Dashboard;

use crate::ctx::Ctx;
use crate::help::{self, print_config};
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if help::wants_help(ctx.args, 1) {
        print_config();
        return Ok(());
    }
    let mut right: Option<String> = None;
    let mut bottom: Option<String> = None;
    let mut show = false;
    let mut i = 1;
    while i < ctx.args.len() {
        match ctx.args[i].as_str() {
            "show" => show = true,
            "--right" => {
                i += 1;
                let raw = ctx.args.get(i).ok_or("config: --right needs a size")?;
                if right.is_some() {
                    return Err("config: --right given twice".into());
                }
                right = Some(parse_flag(raw)?);
            }
            "--bottom" => {
                i += 1;
                let raw = ctx.args.get(i).ok_or("config: --bottom needs a size")?;
                if bottom.is_some() {
                    return Err("config: --bottom given twice".into());
                }
                bottom = Some(parse_flag(raw)?);
            }
            other => return Err(format!("unknown config command: {other}").into()),
        }
        i += 1;
    }
    if show && (right.is_some() || bottom.is_some()) {
        return Err("config: show does not take sizes".into());
    }
    let mut cfg = load::load_or_empty(ctx.cfg_path)?;
    if show {
        print_sizes(&cfg.dashboard);
        return Ok(());
    }
    if right.is_none() && bottom.is_none() {
        eprintln!("Dashboard split. Enter keeps the shown size.");
        eprintln!("right is the shell column. bottom is the lower shell in that column.");
        eprintln!("A size is 1-99% or 1-9999 cells.");
        cfg.dashboard.right = prompt_size("right column", &cfg.dashboard.right)?;
        cfg.dashboard.bottom = prompt_size("bottom shell", &cfg.dashboard.bottom)?;
    } else {
        if let Some(size) = right {
            cfg.dashboard.right = size;
        }
        if let Some(size) = bottom {
            cfg.dashboard.bottom = size;
        }
    }
    cfg.save_path(ctx.cfg_path)?;
    print_sizes(&cfg.dashboard);
    Ok(())
}

fn parse_flag(raw: &str) -> Result<String, Box<dyn std::error::Error>> {
    Dashboard::parse_size(raw).map_err(|msg| format!("config: {msg}").into())
}

fn print_sizes(dash: &Dashboard) {
    println!("{}", dash.right);
    println!("{}", dash.bottom);
}

fn prompt_size(label: &str, current: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tty = io::stdin().is_terminal();
    loop {
        eprint!("{label} [{current}]: ");
        io::stderr().flush()?;
        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            return Err("config: cancelled".into());
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Ok(current.to_string());
        }
        match Dashboard::parse_size(trimmed) {
            Ok(size) => return Ok(size),
            Err(msg) if tty => eprintln!("sjmp: {msg}"),
            Err(msg) => return Err(format!("config: {msg}").into()),
        }
    }
}

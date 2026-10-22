// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod cmd;
mod ctx;
mod help;
mod hook;
mod load;

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use symjump_config::Config;

use ctx::Ctx;

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
        load::ensure_config(&cfg_path)?;
    }
    let ctx = Ctx {
        args: &args,
        cfg_path: &cfg_path,
        home: &home,
        explicit_config,
    };
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    match cmd {
        "help" | "-h" | "--help" => help::run(&ctx)?,
        "list" => cmd::list::run(&ctx)?,
        "jump" => cmd::jump::run(&ctx)?,
        "pin" => cmd::pin::run(&ctx)?,
        "unpin" => cmd::unpin::run(&ctx)?,
        "kids" => cmd::kids::run(&ctx)?,
        "init" => cmd::init::run(&ctx)?,
        "action" | "actions" => cmd::action::run(&ctx)?,
        "exec" => cmd::exec::run(&ctx)?,
        "toolbox" => cmd::toolbox::run(&ctx)?,
        other => return Err(format!("unknown command: {other}").into()),
    }
    io::stdout().flush()?;
    Ok(())
}

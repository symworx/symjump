// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::{add_agent, add_toolbox};

use crate::ctx::Ctx;
use crate::load;

const USAGE: &str = "\
usage: sjmp action add agent --cmd CMD [--label N] [--keys K]
       sjmp action add toolbox --name NAME [--label N] [--keys K]";

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let kind = ctx.args.get(2).map(String::as_str).ok_or(USAGE)?;
    let mut label: Option<String> = None;
    let mut keys: Option<String> = None;
    let mut cmd: Option<String> = None;
    let mut name: Option<String> = None;
    let mut i = 3;
    while i < ctx.args.len() {
        match ctx.args[i].as_str() {
            "--label" => {
                i += 1;
                label = ctx.args.get(i).cloned();
            }
            "--keys" => {
                i += 1;
                keys = ctx.args.get(i).cloned();
            }
            "--cmd" => {
                i += 1;
                cmd = ctx.args.get(i).cloned();
            }
            "--name" => {
                i += 1;
                name = ctx.args.get(i).cloned();
            }
            other => return Err(format!("unknown action add flag: {other}").into()),
        }
        i += 1;
    }
    let mut cfg = load::load_or_empty(ctx.cfg_path)?;
    match kind {
        "agent" => {
            let cmd = cmd.ok_or("usage: sjmp action add agent --cmd CMD [--label N] [--keys K]")?;
            let label = label
                .unwrap_or_else(|| cmd.split_whitespace().next().unwrap_or("agent").to_string());
            add_agent(&mut cfg, label.clone(), cmd, keys)?;
            cfg.save_path(ctx.cfg_path)?;
            println!("{label}");
        }
        "toolbox" => {
            let name =
                name.ok_or("usage: sjmp action add toolbox --name NAME [--label N] [--keys K]")?;
            let label = label.unwrap_or_else(|| name.clone());
            add_toolbox(&mut cfg, label.clone(), name, keys)?;
            cfg.save_path(ctx.cfg_path)?;
            println!("{label}");
        }
        _ => return Err(USAGE.into()),
    }
    Ok(())
}

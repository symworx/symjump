// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_config::expand_user;
use symjump_core::{exec_line, resolve_favorite};

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    let mut target: Option<String> = None;
    let mut cmdv: Option<String> = None;
    let mut i = 1;
    while i < ctx.args.len() {
        if ctx.args[i] == "--cmd" {
            i += 1;
            cmdv = ctx.args.get(i).cloned();
        } else if !ctx.args[i].starts_with('-') {
            target = Some(ctx.args[i].clone());
        }
        i += 1;
    }
    let target = target.ok_or("usage: sjmp exec <fav|path> --cmd <cmd>")?;
    let cmdv = cmdv.ok_or("usage: sjmp exec <fav|path> --cmd <cmd>")?;
    let path = match resolve_favorite(&cfg, &target, ctx.home) {
        Ok(r) => r.path,
        Err(_) => expand_user(&target, ctx.home),
    };
    println!("{}", exec_line(&cmdv, &path)?);
    Ok(())
}

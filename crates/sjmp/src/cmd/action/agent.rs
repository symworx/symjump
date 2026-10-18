// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::{find_agent, render_agent_cmd, resolve_favorite};

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    let query = ctx
        .args
        .get(2)
        .ok_or("usage: sjmp action agent <action> <fav>")?;
    let target = ctx
        .args
        .get(3)
        .ok_or("usage: sjmp action agent <action> <fav>")?;
    let agent = find_agent(&cfg, query)?;
    let r = resolve_favorite(&cfg, target, ctx.home)?;
    println!("{}", render_agent_cmd(agent, &r.path));
    if !agent.cmd.contains("{path}") {
        eprintln!("{}", r.path.display());
    }
    Ok(())
}

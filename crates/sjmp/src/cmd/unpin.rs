// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::unpin;

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let q = ctx.args.get(1).ok_or("usage: sjmp unpin <label|key>")?;
    let mut cfg = load::load_or_empty(ctx.cfg_path)?;
    let gone = unpin(&mut cfg, q)?;
    cfg.save_path(ctx.cfg_path)?;
    println!("{}", gone.label);
    Ok(())
}

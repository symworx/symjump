// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::resolve_favorite;

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let q = ctx.args.get(1).ok_or("usage: sjmp jump <label|key>")?;
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    let r = resolve_favorite(&cfg, q, ctx.home)?;
    println!("{}", r.path.display());
    Ok(())
}

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::list_lines;

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    for line in list_lines(&cfg, ctx.home) {
        println!("{line}");
    }
    Ok(())
}

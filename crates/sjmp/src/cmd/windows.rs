// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if ctx.args.len() > 1 {
        return Err("sjmp windows takes no arguments".into());
    }
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    for w in cfg.window_layouts() {
        let key = w.keys.as_deref().unwrap_or("-");
        println!("{key}\t{}\t{}\t{}", w.cols, w.rows, w.label);
    }
    Ok(())
}

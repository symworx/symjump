// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if ctx.args.len() > 1 {
        return Err("sjmp panes takes no arguments".into());
    }
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    for pane in cfg.pane_layouts() {
        let key = pane.keys.as_deref().unwrap_or("-");
        println!("{key}\t{}\t{}\t{}", pane.cols, pane.rows, pane.label);
    }
    Ok(())
}

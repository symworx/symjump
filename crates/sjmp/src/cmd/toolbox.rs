// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::{find_toolbox, toolbox_enter_cmd};

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load::load_or_empty(ctx.cfg_path)?;
    match ctx.args.get(1).map(String::as_str) {
        Some("list") => {
            for t in &cfg.actions.toolbox {
                println!(
                    "{}\t{}\t{}",
                    t.keys.as_deref().unwrap_or("-"),
                    t.label,
                    t.name
                );
            }
        }
        Some("enter") => {
            let q = ctx
                .args
                .get(2)
                .ok_or("usage: sjmp toolbox enter <name|key>")?;
            println!("{}", toolbox_enter_cmd(find_toolbox(&cfg, q)?));
        }
        _ => return Err("usage: sjmp toolbox list | sjmp toolbox enter <q>".into()),
    }
    Ok(())
}

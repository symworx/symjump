// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use crate::ctx::Ctx;
use crate::hook;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match ctx.args.get(1).map(String::as_str) {
        None => {
            if ctx.explicit_config {
                load::ensure_config(ctx.cfg_path)?;
            }
            println!("{}", ctx.cfg_path.display());
        }
        Some("bash") => {
            if ctx.explicit_config {
                load::ensure_config(ctx.cfg_path)?;
            }
            let cfg = load::load_or_empty(ctx.cfg_path)?;
            print!("{}", hook::bash_hook(&cfg));
        }
        _ => return Err("usage: sjmp init [bash]".into()),
    }
    Ok(())
}

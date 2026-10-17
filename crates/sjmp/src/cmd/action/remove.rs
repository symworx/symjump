// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use symjump_core::{ActionKind, remove_action};

use crate::ctx::Ctx;
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    let rest: Vec<&str> = ctx.args.iter().skip(2).map(String::as_str).collect();
    let (kind, q) = match rest.as_slice() {
        ["agent", q] => (Some(ActionKind::Agent), *q),
        ["toolbox", q] => (Some(ActionKind::Toolbox), *q),
        [q] => (None, *q),
        _ => return Err("usage: sjmp action rm [agent|toolbox] <label|key>".into()),
    };
    let mut cfg = load::load_or_empty(ctx.cfg_path)?;
    let gone = remove_action(&mut cfg, q, kind)?;
    cfg.save_path(ctx.cfg_path)?;
    println!("{}\t{}", gone.kind(), gone.label());
    Ok(())
}

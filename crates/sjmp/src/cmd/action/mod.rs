// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod add;
mod agent;
mod list;
mod remove;

use crate::ctx::Ctx;
use crate::help::{self, print_action};

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if ctx.args.get(1).is_none() || help::wants_help(ctx.args, 1) {
        print_action();
        return Ok(());
    }
    match ctx.args.get(1).map(String::as_str) {
        Some("list") => list::run(ctx),
        Some("agent") => agent::run(ctx),
        Some("add") => add::run(ctx),
        Some("rm") | Some("remove") => remove::run(ctx),
        _ => Err("unknown action command (try sjmp action --help)".into()),
    }
}

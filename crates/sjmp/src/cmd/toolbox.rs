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
            let backend = container_backend()?;
            println!("{}", toolbox_enter_cmd(find_toolbox(&cfg, q)?, backend));
        }
        _ => return Err("usage: sjmp toolbox list | sjmp toolbox enter <q>".into()),
    }
    Ok(())
}

fn which(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Prefer toolbox, then distrobox.
fn container_backend() -> Result<&'static str, Box<dyn std::error::Error>> {
    if which("toolbox") {
        Ok("toolbox")
    } else if which("distrobox") {
        Ok("distrobox")
    } else {
        Err("neither toolbox nor distrobox found on PATH".into())
    }
}

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::io::{self, IsTerminal, Write};

use symjump_config::{DashPane, Dashboard, SplitDirection};

use crate::ctx::Ctx;
use crate::help::{self, print_config};
use crate::load;

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    if help::wants_help(ctx.args, 1) {
        print_config();
        return Ok(());
    }
    let mut show = false;
    let mut layout = false;
    let mut direction: Option<SplitDirection> = None;
    let mut sizes: Option<Vec<u8>> = None;
    let mut secondaries: Vec<Option<Vec<u8>>> = Vec::new();
    let mut i = 1;
    while i < ctx.args.len() {
        match ctx.args[i].as_str() {
            "show" => show = true,
            "layout" => layout = true,
            "--direction" => {
                i += 1;
                let raw = ctx
                    .args
                    .get(i)
                    .ok_or("config: --direction needs vertical or horizontal")?;
                if direction.is_some() {
                    return Err("config: --direction given twice".into());
                }
                direction = Some(parse_direction(raw)?);
            }
            "--sizes" => {
                i += 1;
                let raw = ctx.args.get(i).ok_or("config: --sizes needs a ratio")?;
                if sizes.is_some() {
                    return Err("config: --sizes given twice".into());
                }
                sizes = Some(
                    symjump_config::parse_full_percents(raw)
                        .map_err(|msg| format!("config: {msg}"))?,
                );
            }
            "--secondary" => {
                i += 1;
                let raw = ctx
                    .args
                    .get(i)
                    .ok_or("config: --secondary needs a ratio or none")?;
                secondaries.push(parse_secondary(raw)?);
            }
            other => return Err(format!("unknown config command: {other}").into()),
        }
        i += 1;
    }
    if (show || layout) && (direction.is_some() || sizes.is_some() || !secondaries.is_empty()) {
        return Err("config: show and layout do not take sizes".into());
    }
    if show && layout {
        return Err("config: use show or layout".into());
    }
    let mut cfg = load::load_or_empty(ctx.cfg_path)?;
    if show {
        print!("{}", cfg.dashboard.summary());
        return Ok(());
    }
    if layout {
        print!("{}", cfg.dashboard.layout());
        return Ok(());
    }
    let flags = direction.is_some() || sizes.is_some() || !secondaries.is_empty();
    if flags {
        let Some(sizes) = sizes else {
            return Err("config: --sizes is required".into());
        };
        if secondaries.len() != sizes.len() {
            return Err(
                "config: one --secondary per pane (use none when that pane is not split)".into(),
            );
        }
        let mut panes = Vec::with_capacity(sizes.len());
        for (size, sub) in sizes.into_iter().zip(secondaries) {
            panes.push(DashPane {
                size,
                subs: sub.unwrap_or_default(),
            });
        }
        let direction = direction.unwrap_or(cfg.dashboard.direction);
        cfg.dashboard = Dashboard::new(direction, panes).map_err(|msg| format!("config: {msg}"))?;
    } else {
        cfg.dashboard = walk(&cfg.dashboard)?;
    }
    cfg.save_path(ctx.cfg_path)?;
    print!("{}", cfg.dashboard.summary());
    Ok(())
}

fn parse_direction(raw: &str) -> Result<SplitDirection, Box<dyn std::error::Error>> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "v" | "vertical" => Ok(SplitDirection::Vertical),
        "h" | "horizontal" => Ok(SplitDirection::Horizontal),
        _ => Err(format!("config: direction must be vertical or horizontal (got `{raw}`)").into()),
    }
}

fn parse_secondary(raw: &str) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "none" | "-" => Ok(None),
        _ => Ok(Some(
            symjump_config::parse_full_percents(raw).map_err(|msg| format!("config: {msg}"))?,
        )),
    }
}

fn walk(current: &Dashboard) -> Result<Dashboard, Box<dyn std::error::Error>> {
    eprintln!("Dashboard layout. Enter keeps the shown value.");
    eprintln!("vertical is left | right. horizontal is top / bottom.");
    eprintln!("0 primary splits is one pane. 1 is two panes. 2 is three. 3 is four.");
    eprintln!("Each pane can then be split the other way. Percents sum to 100, such as 60/40.");
    let direction = prompt_direction(current.direction)?;
    let splits = prompt_count("Primary splits", current.panes.len().saturating_sub(1), 3)?;
    let pane_count = splits + 1;
    let axis = match direction {
        SplitDirection::Vertical => "top/bottom",
        SplitDirection::Horizontal => "left/right",
    };
    let mut sub_counts = Vec::with_capacity(pane_count);
    for i in 0..pane_count {
        let name = symjump_config::pane_name(direction, i, pane_count);
        let prev = current
            .panes
            .get(i)
            .map(|pane| {
                if pane.subs.is_empty() {
                    0
                } else {
                    pane.subs.len() - 1
                }
            })
            .unwrap_or(0);
        let n = prompt_count(&format!("Secondary {axis} splits on {name}"), prev, 3)?;
        sub_counts.push(n);
    }
    let sizes = if pane_count == 1 {
        vec![100]
    } else {
        let hint = if current.panes.len() == pane_count {
            symjump_config::ratio_text(
                &current
                    .panes
                    .iter()
                    .map(|pane| pane.size)
                    .collect::<Vec<_>>(),
            )
        } else {
            even_ratio(pane_count)
        };
        let label = match direction {
            SplitDirection::Vertical => "Left-right percents",
            SplitDirection::Horizontal => "Top-bottom percents",
        };
        prompt_ratio(label, &hint, pane_count)?
    };
    let mut panes = Vec::with_capacity(pane_count);
    for i in 0..pane_count {
        let parts = sub_counts[i] + 1;
        let subs = if sub_counts[i] == 0 {
            Vec::new()
        } else {
            let hint = if current.panes.get(i).map(|pane| pane.subs.len()) == Some(parts) {
                symjump_config::ratio_text(&current.panes[i].subs)
            } else {
                even_ratio(parts)
            };
            let name = symjump_config::pane_name(direction, i, pane_count);
            prompt_ratio(&format!("{axis} on {name}"), &hint, parts)?
        };
        panes.push(DashPane {
            size: sizes[i],
            subs,
        });
    }
    Dashboard::new(direction, panes).map_err(|msg| format!("config: {msg}").into())
}

fn even_ratio(n: usize) -> String {
    let base = (100 / n) as u8;
    let extra = 100 % n;
    let parts: Vec<u8> = (0..n)
        .map(|i| if i < extra { base + 1 } else { base })
        .collect();
    symjump_config::ratio_text(&parts)
}

fn prompt_direction(current: SplitDirection) -> Result<SplitDirection, Box<dyn std::error::Error>> {
    let tty = io::stdin().is_terminal();
    loop {
        let answer = prompt_line(&format!(
            "Primary split (vertical or horizontal) [{}]",
            current.as_str()
        ))?;
        if answer.is_empty() {
            return Ok(current);
        }
        match parse_direction(&answer) {
            Ok(direction) => return Ok(direction),
            Err(err) if tty => eprintln!("sjmp: {err}"),
            Err(err) => return Err(err),
        }
    }
}

fn prompt_count(
    label: &str,
    current: usize,
    max: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let tty = io::stdin().is_terminal();
    loop {
        let answer = prompt_line(&format!("{label} [{current}]"))?;
        if answer.is_empty() {
            return Ok(current);
        }
        let bad = format!("config: {label} must be 0-{max} (got `{answer}`)");
        let n: usize = match answer.parse() {
            Ok(n) if n <= max => n,
            _ if tty => {
                eprintln!("sjmp: {bad}");
                continue;
            }
            _ => return Err(bad.into()),
        };
        return Ok(n);
    }
}

fn prompt_ratio(label: &str, hint: &str, n: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let tty = io::stdin().is_terminal();
    loop {
        let answer = prompt_line(&format!("{label} [{hint}]"))?;
        let raw = if answer.is_empty() {
            hint
        } else {
            answer.as_str()
        };
        match symjump_config::parse_percents(raw, n) {
            Ok(parts) => return Ok(parts),
            Err(msg) if tty => eprintln!("sjmp: config: {msg}"),
            Err(msg) => return Err(format!("config: {msg}").into()),
        }
    }
}

fn prompt_line(label: &str) -> Result<String, Box<dyn std::error::Error>> {
    eprint!("{label}: ");
    io::stderr().flush()?;
    let mut line = String::new();
    if io::stdin().read_line(&mut line)? == 0 {
        return Err("config: cancelled".into());
    }
    Ok(line.trim().to_string())
}

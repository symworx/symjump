// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod action;
mod pin;

use crate::ctx::Ctx;

pub use action::print as print_action;
pub use pin::print as print_pin;

pub fn wants_help(args: &[String], start: usize) -> bool {
    match args.get(start).map(String::as_str) {
        Some("-h" | "--help" | "help") => true,
        _ => {
            let mut i = start;
            while i < args.len() {
                match args[i].as_str() {
                    "-h" | "--help" => return true,
                    "--label" | "--keys" | "--cmd" | "--name" => i += 1,
                    _ => {}
                }
                i += 1;
            }
            false
        }
    }
}

pub fn run(ctx: &Ctx) -> Result<(), Box<dyn std::error::Error>> {
    match ctx.args.get(1).map(String::as_str) {
        None | Some("-h") | Some("--help") | Some("help") => print_help(),
        Some("pin") => print_pin(),
        Some("action") | Some("actions") => print_action(),
        Some(other) => {
            return Err(format!("unknown help topic: {other} (try pin or action)").into());
        }
    }
    Ok(())
}

fn print_help() {
    print!(
        "\
sjmp — portable favorites + action launcher

Usage:
  sjmp <command> [args]
  sjmp help [pin|action]

Commands:
  list                      list favorites
  jump <label|key>          print favorite path
  pin                       pin a directory
  unpin <label|key>         remove a favorite
  kids [path]               list child directories
  init [bash]               print config path, or bash hook
  action                    list, add, run, or remove actions
  exec <fav|path> --cmd C   print an exec line
  toolbox list              list toolbox actions
  toolbox enter <q>         print enter command (toolbox or distrobox)

Options:
  --config PATH             config file
  -h, --help                this screen

See also:
  sjmp pin --help
  sjmp action --help
"
    );
}

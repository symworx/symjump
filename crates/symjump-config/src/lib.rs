// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

//! Config file types and I/O for symjump.
//!
//! Parser is a closed subset of TOML (no serde): root keys, `[keys]`,
//! `[frequent]`, `[ai]`, `[dashboard]`, `[[favorites]]`, `[[actions.toolbox]]`,
//! `[[actions.agent]]`.

mod error;
mod io;
mod model;
mod toml;

pub use error::ConfigError;
pub use io::expand_user;
pub use model::{
    Actions, AgentAction, Ai, Config, DashPane, Dashboard, Favorite, Frequent, Keys,
    SplitDirection, ToolboxAction, pane_name, parse_full_percents, parse_percents, ratio_text,
};

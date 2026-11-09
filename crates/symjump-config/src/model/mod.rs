// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod actions;
mod ai;
mod dashboard;
mod favorite;
mod frequent;
mod keys;

pub use actions::{Actions, AgentAction, ToolboxAction};
pub use ai::Ai;
pub use dashboard::{
    DashPane, Dashboard, SplitDirection, pane_name, parse_full_percents, parse_percents, ratio_text,
};
pub use favorite::Favorite;
pub use frequent::Frequent;
pub use keys::Keys;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config {
    pub root: Option<String>,
    pub keys: Keys,
    pub frequent: Frequent,
    pub ai: Ai,
    pub dashboard: Dashboard,
    pub favorites: Vec<Favorite>,
    pub actions: Actions,
}

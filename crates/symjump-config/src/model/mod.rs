// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod actions;
mod ai;
mod favorite;
mod frequent;
mod keys;

pub use actions::{Actions, AgentAction, ToolboxAction};
pub use ai::Ai;
pub use favorite::Favorite;
pub use frequent::Frequent;
pub use keys::Keys;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config {
    pub root: Option<String>,
    pub keys: Keys,
    pub frequent: Frequent,
    pub ai: Ai,
    pub favorites: Vec<Favorite>,
    pub actions: Actions,
}

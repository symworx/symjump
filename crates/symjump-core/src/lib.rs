// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

//! Pure operations over a [`symjump_config::Config`].
//! The CLI prints paths; the shell hook is responsible for `cd`.

mod action;
mod error;
mod favorite;
mod keys;
mod kids;

#[cfg(test)]
mod test_support;

pub use action::{
    ActionKind, RemovedAction, action_list_lines, add_agent, add_toolbox, exec_line, find_agent,
    find_toolbox, remove_action, render_agent_cmd, toolbox_enter_cmd,
};
pub use error::CoreError;
pub use favorite::{
    Resolved, list_favorites, list_lines, pin, resolve_favorite, resolve_one, unpin,
};
pub use keys::{check_action_key, check_favorite_key, meta_letter, reserved_meta_letters};
pub use kids::{is_git_root, kids};

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("no favorite matching `{0}`")]
    UnknownFavorite(String),
    #[error("no agent action matching `{0}`")]
    UnknownAgent(String),
    #[error("no toolbox action matching `{0}`")]
    UnknownToolbox(String),
    #[error("no action matching `{0}`")]
    UnknownAction(String),
    #[error("ambiguous action `{0}` (agent and toolbox); pass agent or toolbox")]
    AmbiguousAction(String),
    #[error("path is empty")]
    EmptyPath,
    #[error("key `{0}` is reserved ({1})")]
    KeyReserved(String, String),
    #[error("key `{0}` already used by `{1}`")]
    KeyTaken(String, String),
    #[error("action command is empty")]
    EmptyCmd,
    #[error("toolbox name is empty")]
    EmptyToolboxName,
}

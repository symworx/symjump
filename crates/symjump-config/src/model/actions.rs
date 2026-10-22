// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Actions {
    pub toolbox: Vec<ToolboxAction>,
    pub agent: Vec<AgentAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolboxAction {
    pub label: String,
    pub name: String,
    pub keys: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentAction {
    pub label: String,
    pub cmd: String,
    pub keys: Option<String>,
}

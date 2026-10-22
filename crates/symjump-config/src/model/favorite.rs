// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Favorite {
    pub label: String,
    pub path: String,
    pub keys: Option<String>,
}

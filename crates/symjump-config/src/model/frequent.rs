// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frequent {
    pub source: String,
    pub max: u32,
}

impl Default for Frequent {
    fn default() -> Self {
        Self {
            source: "zoxide".into(),
            max: 20,
        }
    }
}

// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keys {
    pub leader: String,
    pub kids: String,
    pub actions: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            leader: "M-p".into(),
            kids: "M-P".into(),
            actions: "M-x".into(),
        }
    }
}

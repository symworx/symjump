// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::path::Path;

pub struct Ctx<'a> {
    pub args: &'a [String],
    pub cfg_path: &'a Path,
    pub home: &'a Path,
    pub explicit_config: bool,
}

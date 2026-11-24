// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

pub fn print() {
    print!(
        "\
sjmp check — see if fzf and tmux are on PATH

Usage:
  sjmp check
  sjmp doctor

The first run prints this check. Links are included for anything missing.
sjmp doctor prints the links either way. The exit status is 1 when a tool is missing.
"
    );
}

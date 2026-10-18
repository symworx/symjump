// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

pub fn print() {
    print!(
        "\
sjmp pin — pin a favorite directory

Usage:
  sjmp pin [--current] [--label NAME] [--keys LETTER]
  sjmp pin <path> [--label NAME] [--keys LETTER]
  sjmp unpin <label|key>

Flags:
  --current        pin $PWD (also the default when no path is given)
  --label NAME     display name (default: last path component)
  --keys LETTER    Meta jump key (M-LETTER)
  --current cannot be combined with a path

  --keys refuses: p/P/x (sjmp chords), b/f/d/y (readline), and duplicates

Examples:
  sjmp pin --current --label src --keys r
  sjmp pin ~/worx --label worx --keys w
  sjmp unpin w
"
    );
}

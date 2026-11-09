// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

pub fn print() {
    print!(
        "\
sjmp config — dashboard split sizes

Usage:
  sjmp config
  sjmp config show
  sjmp config [--right SIZE] [--bottom SIZE]

The dashboard is one agent pane on the left and two shells in the right column.
right is that column's width. bottom is how much of the column the lower shell takes.
Enter during the walk keeps the current size.
A size is 1-99% or a cell count from 1 to 9999.
The layout reads these when the dashboard opens.

Pins stay on sjmp pin and sjmp unpin.

Examples:
  sjmp config
  sjmp config show
  sjmp config --right 40% --bottom 30%
  sjmp config --right 40
"
    );
}

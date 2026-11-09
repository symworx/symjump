// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

pub fn print() {
    print!(
        "\
sjmp config — dashboard layout

Usage:
  sjmp config
  sjmp config show
  sjmp config layout
  sjmp config --sizes RATIO --secondary RATIO|--secondary none ...
             [--direction vertical|horizontal]

The walk asks for the primary split, how many splits, a secondary split on
each pane, then the percentages.

vertical is left | right. horizontal is top / bottom.
0 primary splits is one pane. 1 is two panes. 2 is three. 3 is four.
A secondary split cuts that pane the other way. none means that pane stays whole.
Percents belong to that split and sum to 100, such as 60/40 or 50/50.
The first pane keeps the agent. The others are shells.
The layout is read when the dashboard opens.

Pins stay on sjmp pin and sjmp unpin.

Examples:
  sjmp config
  sjmp config show
  sjmp config --sizes 60/40 --secondary 50/50 --secondary none
  sjmp config --direction horizontal --sizes 70/30 --secondary none --secondary 40/60
"
    );
}

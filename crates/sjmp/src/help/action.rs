// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

pub fn print() {
    print!(
        "\
sjmp action — agent and toolbox actions (M-x)

Usage:
  sjmp action list
  sjmp action add agent --cmd CMD [--label NAME] [--keys LETTER]
  sjmp action add toolbox --name NAME [--label NAME] [--keys LETTER]
  sjmp action rm [agent|toolbox] <label|key>
  sjmp action agent <action> <favorite>

  rm also accepts `remove`. Pass agent or toolbox when the query matches both.

Examples:
  sjmp action add agent --cmd grok --keys g --label grok-build
  sjmp action add toolbox --name dev-python --keys p --label python
  sjmp action list
  sjmp action agent g s
  sjmp action rm g
  sjmp action rm agent python
"
    );
}

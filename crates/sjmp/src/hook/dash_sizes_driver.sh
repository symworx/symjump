#!/bin/bash
# Sources the generated hook and checks dashboard layout parsing.
# Copyright (c) 2026 Nathaniel T. Berry
# Licensed under the Apache License, Version 2.0.

set -euo pipefail

hook=$1
root=$(mktemp -d)
sock="sjmp-layout-$$"
cleanup() {
  command tmux -L "$sock" -f /dev/null kill-server >/dev/null 2>&1 || true
  rm -rf "$root"
}
trap cleanup EXIT
bin=$root/bin
mkdir -p "$bin"

cat > "$bin/sjmp" << 'EOF'
#!/bin/bash
if [ "${1:-}" != config ] || [ "${2:-}" != layout ]; then
  echo "unexpected sjmp: $*" >&2
  exit 1
fi
if [ "${SJMP_LAYOUT_RC:-0}" -ne 0 ]; then
  exit "$SJMP_LAYOUT_RC"
fi
printf '%s' "${SJMP_LAYOUT-}"
EOF
chmod +x "$bin/sjmp"

fail() { echo "FAIL: $*" >&2; exit 1; }

export PATH="$bin:$PATH"
unset INSIDE_EMACS
# shellcheck disable=SC1090
source "$hook"

sjmp_layout_ok $'vertical\n66 0\n34 1 50 50' || fail 'default layout rejected'
sjmp_layout_ok $'horizontal\n100 1 40 60' || fail 'single pane secondary rejected'
sjmp_layout_ok $'vertical\n60 0\n40 0;x' && fail 'injection accepted'
sjmp_layout_ok $'vertical\n60 0\n50 0' && fail 'sum 110 accepted'
sjmp_layout_ok 'vertical' && fail 'direction only accepted'

got="$(sjmp_share_cells 100 60 40)"
[ "$got" = $'60\n40' ] || fail "share 100 [$got]"
got="$(sjmp_share_cells 99 60 40)"
[ "$got" = $'59\n40' ] || fail "share 99 [$got]"
got="$(sjmp_share_cells 11 50 50)"
[ "$got" = $'6\n5' ] || fail "share 11 [$got]"
got="$(sjmp_share_cells 100 50 30 20)"
[ "$got" = $'50\n30\n20' ] || fail "share 3 [$got]"

export SJMP_LAYOUT SJMP_LAYOUT_RC
SJMP_LAYOUT=$'vertical\n60 0\n40 0'
got="$(sjmp_layout_fetch)"
[ "$got" = $'vertical\n60 0\n40 0' ] || fail "fetch [$got]"

SJMP_LAYOUT=$'vertical\n60 0;touch'
got="$(sjmp_layout_fetch)"
[ "$got" = $'vertical\n66 0\n34 1 50 50' ] || fail "fallback [$got]"

SJMP_LAYOUT_RC=1
got="$(sjmp_layout_fetch)"
[ "$got" = $'vertical\n66 0\n34 1 50 50' ] || fail "rc fallback [$got]"
unset SJMP_LAYOUT_RC

if ! command -v tmux >/dev/null 2>&1; then
  echo ok
  exit 0
fi

command tmux -L "$sock" -f /dev/null new-session -d -s lay -x 100 -y 32 -n w
tmux() { command tmux -L "$sock" -f /dev/null "$@"; }

target="$(tmux display-message -p -t lay: '#{session_name}:#{window_index}')"
SJMP_LAYOUT=$'vertical\n60 0\n40 0'
sjmp_tmux_layout "$target" "$PWD" grok

agent="$(tmux display-message -p -t "$target" '#{pane_id}')"
title="$(tmux display-message -p -t "$agent" '#{pane_title}')"
[ "$title" = grok ] || fail "agent title [$title]"

mapfile -t rows < <(tmux list-panes -t "$target" -F '#{pane_left} #{pane_width} #{pane_title}')
[ "${#rows[@]}" -eq 2 ] || fail "pane count ${#rows[@]}"
left="${rows[0]#* }"
left="${left%% *}"
right="${rows[1]#* }"
right="${right%% *}"
[ "$left" = 59 ] || fail "left width [$left] rows ${rows[*]}"
[ "$right" = 40 ] || fail "right width [$right] rows ${rows[*]}"
case "${rows[1]}" in
  *shell) ;;
  *) fail "right title [${rows[1]}]" ;;
esac

command tmux -L "$sock" -f /dev/null kill-session -t lay
command tmux -L "$sock" -f /dev/null new-session -d -s lay -x 100 -y 32 -n w
target="$(tmux display-message -p -t lay: '#{session_name}:#{window_index}')"
SJMP_LAYOUT=$'vertical\n100 1 50 50'
sjmp_tmux_layout "$target" "$PWD" grok
mapfile -t heights < <(tmux list-panes -t "$target" -F '#{pane_top} #{pane_height}')
[ "${#heights[@]}" -eq 2 ] || fail "stack count ${#heights[@]}"
h1="${heights[0]#* }"
h2="${heights[1]#* }"
diff=$((h1 > h2 ? h1 - h2 : h2 - h1))
[ "$diff" -le 1 ] || fail "stack heights ${heights[*]}"

echo ok

#!/bin/bash
# Sources the generated hook and checks that the agent dashboard
# does not open a tmux window when the place is already open.
# Copyright (c) 2026 Nathaniel T. Berry
# Licensed under the Apache License, Version 2.0.

set -euo pipefail

hook=$1
work=$(mktemp -d)
log=$work/tmux.log
cleanup() { rm -rf "$work"; }
trap cleanup EXIT

bin=$work/bin
mkdir -p "$bin" "$work/repo/sub" "$work/other"
git -C "$work/repo" init -q
git -C "$work/other" init -q
: > "$log"

cat > "$bin/sjmp" << 'EOF'
#!/bin/bash
if [ "${1:-}" = config ] && [ "${2:-}" = layout ]; then
  printf '%s\n' vertical
  printf '%s\n' '66 0'
  printf '%s\n' '34 1 50 50'
  exit 0
fi
echo "unexpected sjmp: $*" >&2
exit 1
EOF
chmod +x "$bin/sjmp"

cat > "$bin/tmux" << 'EOF'
#!/bin/bash
printf '%s\n' "$*" >> "$TMUX_LOG"
case "$1" in
  display-message) printf '%s\n' 80 ;;
  split-window) printf '%s\n' %9 ;;
  new-window) printf '%s\n' sess:1 ;;
  list-panes) printf '%s\n' "${TMUX_PANES:-}" ;;
  has-session)
    [ "${TMUX_HAS_SESSION:-0}" = 1 ]
    exit
    ;;
esac
exit 0
EOF
chmod +x "$bin/tmux"

fail() { echo "FAIL: $*" >&2; exit 1; }
reset() { : > "$log"; }

export PATH="$bin:$PATH"
export TMUX_LOG="$log"
unset INSIDE_EMACS TMUX TMUX_PANES TMUX_HAS_SESSION
# shellcheck disable=SC1090
source "$hook"

# Same directory inside tmux: no tmux command at all.
reset
export TMUX=1
cd "$work/repo"
sjmp_tmux_dash "$work/repo"
[ ! -s "$log" ] || fail "exact dir called tmux: $(cat "$log")"

# Subdirectory of the same git work tree: still no window.
reset
cd "$work/repo/sub"
sjmp_tmux_dash "$work/repo"
[ ! -s "$log" ] || fail "same repo called tmux: $(cat "$log")"

# Another pane in this session is the place: focus it, do not open a window.
reset
cd "$work/other"
TMUX_PANES="$(printf '%%7\t%s' "$work/repo")"
export TMUX_PANES
sjmp_tmux_dash "$work/repo"
grep -q 'select-pane -t %7' "$log" || fail "did not focus existing pane: $(cat "$log")"
if grep -q 'new-window' "$log"; then
  fail "existing pane still opened a window: $(cat "$log")"
fi
unset TMUX_PANES

# A different git work tree still opens a window.
reset
cd "$work/other"
sjmp_tmux_dash "$work/repo"
grep -q 'new-window' "$log" || fail "different repo did not open a window: $(cat "$log")"

# Outside tmux, the first open still creates the session.
reset
unset TMUX
cd "$work/repo"
sjmp_tmux_dash "$work/repo"
grep -q 'new-session' "$log" || fail "outside tmux did not create a session: $(cat "$log")"

# Outside tmux, a session that already has the place is attached, not extended.
reset
export TMUX_HAS_SESSION=1
TMUX_PANES="$(printf '%%3\t%s' "$work/repo")"
export TMUX_PANES
cd "$work/other"
sjmp_tmux_dash "$work/repo"
grep -q 'select-pane -t %3' "$log" || fail "outside did not focus: $(cat "$log")"
if grep -q 'new-window' "$log"; then
  fail "outside added a window: $(cat "$log")"
fi
if grep -q 'new-session' "$log"; then
  fail "outside created a session: $(cat "$log")"
fi

# Current directory, even when this pane is already that place: still a window.
reset
export TMUX=1
cd "$work/repo"
sjmp_tmux_here
grep -q 'new-window' "$log" || fail "here did not open a window: $(cat "$log")"

# The place picker path still stays put in that same directory.
reset
sjmp_tmux_dash "$work/repo"
[ ! -s "$log" ] || fail "picker path in the same dir called tmux: $(cat "$log")"

echo ok

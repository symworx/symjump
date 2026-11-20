#!/bin/bash
# Sources the generated hook and counts tmux splits for a preset grid.
# Copyright (c) 2026 Nathaniel T. Berry
# Licensed under the Apache License, Version 2.0.

set -euo pipefail

hook=$1
work=$(mktemp -d)
log=$work/tmux.log
cleanup() { rm -rf "$work"; }
trap cleanup EXIT

bin=$work/bin
mkdir -p "$bin"
: > "$log"

cat > "$bin/sjmp" << 'EOF'
#!/bin/bash
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
esac
exit 0
EOF
chmod +x "$bin/tmux"

fail() { echo "FAIL: $*" >&2; exit 1; }
reset() { : > "$log"; }
splits() { grep -c 'split-window' "$log" || true; }

export PATH="$bin:$PATH"
export TMUX_LOG="$log"
export TMUX=1
unset INSIDE_EMACS
# shellcheck disable=SC1090
source "$hook"

# 2 columns, 2 rows: one column split, then one row split in each column.
reset
sjmp_tmux_grid 2 2 2x2
grep -q 'new-window' "$log" || fail "2x2 did not open a window: $(cat "$log")"
[ "$(splits)" = 3 ] || fail "2x2 splits $(splits): $(cat "$log")"

# 2 columns, 1 row: only the column split.
reset
sjmp_tmux_grid 2 1 cols
[ "$(splits)" = 1 ] || fail "2x1 splits $(splits): $(cat "$log")"

# 2 columns, 3 rows: one column split, two row splits in each column.
reset
sjmp_tmux_grid 2 3 2x3
[ "$(splits)" = 5 ] || fail "2x3 splits $(splits): $(cat "$log")"

# 1x1 is not a grid.
reset
if sjmp_tmux_grid 1 1 one; then
  fail "1x1 was accepted"
fi
[ ! -s "$log" ] || fail "1x1 called tmux: $(cat "$log")"

echo ok

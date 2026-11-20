#!/bin/bash
# Inside tmux, sjmp_tmux adds a window. Outside, it uses session sjmp-<dir>.
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
  has-session)
    if [ -n "${TMUX_HAS:-}" ]; then
      exit 0
    fi
    exit 1
    ;;
esac
exit 0
EOF
chmod +x "$bin/tmux"

fail() { echo "FAIL: $*" >&2; exit 1; }
reset() { : > "$log"; }
count() { grep -c "$1" "$log" || true; }

export PATH="$bin:$PATH"
export TMUX_LOG="$log"
unset INSIDE_EMACS
# shellcheck disable=SC1090
source "$hook"

proj=$work/proj
mkdir -p "$proj"
cd "$proj"

# Already attached: add a background window, then another. Stay on this window.
export TMUX=1
unset TMUX_HAS
reset
sjmp_tmux
grep -qx "new-window -d -P -F #{window_index} -c ${proj} -n proj" "$log" || fail "inside did not new-window -d: $(cat "$log")"
grep -q 'switch-client' "$log" && fail "inside switched client: $(cat "$log")"
grep -q 'select-window' "$log" && fail "inside selected the new window: $(cat "$log")"
grep -q 'has-session' "$log" && fail "inside looked up a session: $(cat "$log")"
sjmp_tmux
[ "$(count 'new-window')" = 2 ] || fail "second inside open did not add a window: $(cat "$log")"

# Not attached, no session yet: create sjmp-<directory>.
unset TMUX
reset
sjmp_tmux
grep -q 'new-session -d -s sjmp-proj -n proj -c '"$proj" "$log" || fail "outside did not create the session: $(cat "$log")"
grep -q 'new-window' "$log" && fail "outside added a window: $(cat "$log")"

# Not attached, session exists: do not add a window.
export TMUX_HAS=1
reset
sjmp_tmux
grep -q 'has-session -t =sjmp-proj' "$log" || fail "outside missed the session: $(cat "$log")"
grep -q 'new-session' "$log" && fail "outside recreated the session: $(cat "$log")"
grep -q 'new-window' "$log" && fail "second outside open added a window: $(cat "$log")"

echo ok

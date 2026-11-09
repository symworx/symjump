#!/bin/bash
# Sources the generated hook and checks dashboard size parsing.
# Copyright (c) 2026 Nathaniel T. Berry
# Licensed under the Apache License, Version 2.0.

set -euo pipefail

hook=$1
root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
bin=$root/bin
mkdir -p "$bin"

cat > "$bin/sjmp" << 'EOF'
#!/bin/bash
if [ "${1:-}" != config ] || [ "${2:-}" != show ]; then
  echo "unexpected sjmp: $*" >&2
  exit 1
fi
if [ "${SJMP_SHOW_RC:-0}" -ne 0 ]; then
  exit "$SJMP_SHOW_RC"
fi
printf '%s' "${SJMP_SHOW-}"
EOF
chmod +x "$bin/sjmp"

fail() { echo "FAIL: $*" >&2; exit 1; }

export PATH="$bin:$PATH"
unset INSIDE_EMACS
# shellcheck disable=SC1090
source "$hook"

sjmp_ok_size '34%' || fail '34% rejected'
sjmp_ok_size '1%' || fail '1% rejected'
sjmp_ok_size '99%' || fail '99% rejected'
sjmp_ok_size '1' || fail '1 rejected'
sjmp_ok_size '9999' || fail '9999 rejected'
sjmp_ok_size '100%' && fail '100% accepted'
sjmp_ok_size '0' && fail '0 accepted'
sjmp_ok_size '08' && fail '08 accepted'
sjmp_ok_size '34%;x' && fail 'injection accepted'
sjmp_ok_size '10000' && fail '10000 accepted'

expect() {
  local got
  got="$(sjmp_dash_sizes)"
  [ "$got" = "$1" ] || fail "sizes [$got] want [$1]"
}

export SJMP_SHOW SJMP_SHOW_RC

SJMP_SHOW=$'40%\n12'
expect $'40%\n12'

SJMP_SHOW=$'100%\n25%'
expect $'34%\n25%'

SJMP_SHOW=$'34%;touch\n50%'
expect $'34%\n50%'

SJMP_SHOW_RC=1
expect $'34%\n50%'
unset SJMP_SHOW_RC

SJMP_SHOW='40%'
expect $'34%\n50%'

echo ok

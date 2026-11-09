#!/bin/bash
# Drives sjmp_pick_fav with stub sjmp and fzf. The pin path contains a space.
# Copyright (c) 2026 Nathaniel T. Berry
# Licensed under the Apache License, Version 2.0.

set -euo pipefail

hook=$1
pin='/tmp/worx pin'
child='/tmp/worx pin/child proj'
row='w  worx  /tmp/worx pin'

root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT
bin=$root/bin
fake=$root/fake
mkdir -p "$bin"

cat > "$bin/sjmp" << 'EOF'
#!/bin/bash
printf '%s\n' "$1" >> "$SJMP_FAKE/cmds"
case "$1" in
  list)
    printf '%s\n' 'w  worx  /tmp/worx pin'
    ;;
  jump)
    if [ "${SJMP_JUMP_RC:-0}" -ne 0 ]; then
      echo "sjmp: jump failed" >&2
      exit "$SJMP_JUMP_RC"
    fi
    [ "$2" = w ] || { echo "bad jump key [$2]" >&2; exit 1; }
    printf '%s\n' '/tmp/worx pin'
    ;;
  kids)
    printf '%s' "$#" > "$SJMP_FAKE/kids-argc"
    printf '%s' "$2" > "$SJMP_FAKE/kids-path"
    if [ "${SJMP_KIDS_RC:-0}" -ne 0 ]; then
      echo "sjmp: kids failed" >&2
      exit "$SJMP_KIDS_RC"
    fi
    if [ "${SJMP_KIDS_EMPTY:-0}" = 1 ]; then
      exit 0
    fi
    if [ "${SJMP_KIDS_MANY:-0}" = 1 ]; then
      i=1
      while [ "$i" -le 12 ]; do
        printf '%s\n' "/tmp/worx pin/c$(printf '%02d' "$i")"
        i=$((i + 1))
      done
      exit 0
    fi
    printf '%s\n' '/tmp/worx pin/child proj' '/tmp/worx pin/other'
    ;;
  *)
    echo "unexpected sjmp: $*" >&2
    exit 1
    ;;
esac
EOF

cat > "$bin/fzf" << 'EOF'
#!/bin/bash
idx=1
if [ -f "$SJMP_FAKE/idx" ]; then
  idx=$(( $(cat "$SJMP_FAKE/idx") + 1 ))
fi
printf '%s' "$idx" > "$SJMP_FAKE/idx"
cat > "$SJMP_FAKE/fzf-in-$idx"
printf '%s\n' "$@" > "$SJMP_FAKE/fzf-args-$idx"
rc=0
[ -f "$SJMP_FAKE/rc-$idx" ] && rc=$(cat "$SJMP_FAKE/rc-$idx")
if [ -f "$SJMP_FAKE/out-$idx" ]; then
  cat "$SJMP_FAKE/out-$idx"
fi
exit "$rc"
EOF
chmod +x "$bin/sjmp" "$bin/fzf"

fail() { echo "FAIL: $*" >&2; exit 1; }

assert_eq() {
  [ "$2" = "$3" ] || fail "$1: got $(printf %q "$2") want $(printf %q "$3")"
}

assert_file() {
  local got
  got=$(cat "$1")
  [ "$got" = "$2" ] || fail "$3: got $(printf %q "$got") want $(printf %q "$2")"
}

has_arg() {
  grep -F -x -q -- "$2" "$fake/fzf-args-$1"
}

has_substr() {
  grep -F -q -- "$2" "$fake/fzf-args-$1"
}

new_case() {
  rm -rf "$fake"
  mkdir -p "$fake"
  SJMP_KIDS_EMPTY=0
  SJMP_KIDS_RC=0
  SJMP_KIDS_MANY=0
  SJMP_JUMP_RC=0
}

run_pick() {
  env -u INSIDE_EMACS \
    PATH="$bin:$PATH" \
    SJMP_FAKE="$fake" \
    SJMP_KIDS_EMPTY="$SJMP_KIDS_EMPTY" \
    SJMP_KIDS_RC="$SJMP_KIDS_RC" \
    SJMP_KIDS_MANY="$SJMP_KIDS_MANY" \
    SJMP_JUMP_RC="$SJMP_JUMP_RC" \
    bash --noprofile --norc -c '
      source "$1"
      rc=0
      out=$(sjmp_pick_fav 2>"$2/stderr") || rc=$?
      printf "%s" "$out" > "$2/stdout"
      printf "%s" "$rc" > "$2/rc"
    ' _ "$hook" "$fake"
}

# Enter uses the pin and does not list children.
new_case
printf '\n%s\n' "$row" > "$fake/out-1"
run_pick
assert_file "$fake/stdout" "$pin" "enter stdout"
assert_file "$fake/rc" "0" "enter rc"
assert_file "$fake/cmds" $'list\njump' "enter cmds"
assert_file "$fake/stderr" "" "enter stderr"
[ ! -e "$fake/kids-path" ] || fail "enter listed children"
has_arg 1 '--expect=tab' || fail "enter picker missing --expect=tab"
has_arg 1 'enter: this dir   tab: children   esc: back' || fail "enter header"
[ ! -e "$fake/fzf-args-2" ] || fail "enter opened a second picker"

# Tab returns the child. The number and the name are stripped. A space in
# the child name stays in the path. Numbers are 01, 02. Digits are not bound.
new_case
printf 'tab\n%s\n' "$row" > "$fake/out-1"
printf '%s\n' $'01\tchild proj\t/tmp/worx pin/child proj' > "$fake/out-2"
run_pick
assert_file "$fake/stdout" "$child" "tab stdout"
assert_file "$fake/rc" "0" "tab rc"
assert_file "$fake/cmds" $'list\njump\nkids' "tab cmds"
assert_file "$fake/stderr" "" "tab stderr"
assert_file "$fake/kids-argc" "2" "kids argc"
assert_file "$fake/kids-path" "$pin" "kids path"
has_arg 1 '--expect=tab' || fail "tab fav picker missing --expect"
has_arg 2 '--prompt=kid> ' || fail "child prompt"
has_arg 2 'number or name, then enter   esc: back' || fail "child header"
has_arg 2 '--exact' || fail "child picker missing --exact"
has_arg 2 '--nth=1,2' || fail "child picker missing --nth"
has_arg 2 '--with-nth=1,2' || fail "child picker missing --with-nth"
has_arg 2 $'--delimiter=\t' || fail "child picker missing tab delimiter"
if has_substr 2 'pos(1)+accept'; then
  fail "child list bound 1-9"
fi
if has_arg 2 '--expect=tab'; then
  fail "child picker must not use --expect"
fi
assert_file "$fake/fzf-in-2" $'01\tchild proj\t/tmp/worx pin/child proj\n02\tother\t/tmp/worx pin/other' "kids lines"

# Ten or more rows do not bind digits. Typing 10 selects that row.
new_case
SJMP_KIDS_MANY=1
printf 'tab\n%s\n' "$row" > "$fake/out-1"
printf '%s\n' $'10\tc10\t/tmp/worx pin/c10' > "$fake/out-2"
run_pick
assert_file "$fake/stdout" "/tmp/worx pin/c10" "many stdout"
assert_file "$fake/rc" "0" "many rc"
has_arg 2 'number or name, then enter   esc: back' || fail "long child header"
if has_substr 2 'pos(1)+accept'; then
  fail "long child list bound 1-9"
fi
many=$(i=1; while [ "$i" -le 12 ]; do printf '%02d\tc%02d\t/tmp/worx pin/c%02d\n' "$i" "$i" "$i"; i=$((i + 1)); done)
assert_file "$fake/fzf-in-2" "${many%$'\n'}" "many rows"

# No children: stay on the place list. The next Enter uses the pin.
new_case
SJMP_KIDS_EMPTY=1
printf 'tab\n%s\n' "$row" > "$fake/out-1"
printf '\n%s\n' "$row" > "$fake/out-2"
run_pick
assert_file "$fake/stdout" "$pin" "empty stdout"
assert_file "$fake/rc" "0" "empty rc"
assert_file "$fake/stderr" "sjmp: no children: $pin" "empty stderr"
assert_file "$fake/cmds" $'list\njump\nkids\nlist\njump' "empty cmds"
has_arg 2 '--expect=tab' || fail "empty list did not return to the place picker"
has_arg 2 'enter: this dir   tab: children   esc: back' || fail "empty return header"

# Esc on the child list returns to the pins. Enter then uses the pin.
new_case
printf 'tab\n%s\n' "$row" > "$fake/out-1"
printf '%s' "130" > "$fake/rc-2"
printf '\n%s\n' "$row" > "$fake/out-3"
run_pick
assert_file "$fake/stdout" "$pin" "esc-child stdout"
assert_file "$fake/rc" "0" "esc-child rc"
assert_file "$fake/cmds" $'list\njump\nkids\nlist\njump' "esc-child cmds"
has_arg 2 '--prompt=kid> ' || fail "esc-child did not open the child picker"
has_arg 3 '--expect=tab' || fail "esc-child did not return to the place picker"
assert_file "$fake/idx" "3" "esc-child fzf calls"

# Esc on the place list aborts. The action palette treats 130 as back.
new_case
printf '%s' "130" > "$fake/rc-1"
run_pick
assert_file "$fake/stdout" "" "esc-fav stdout"
assert_file "$fake/rc" "130" "esc-fav rc"
assert_file "$fake/cmds" "list" "esc-fav cmds"
[ ! -e "$fake/kids-path" ] || fail "esc-fav listed children"

# A kids failure stops. It does not open the child picker or say "no children".
new_case
SJMP_KIDS_RC=1
printf 'tab\n%s\n' "$row" > "$fake/out-1"
run_pick
assert_file "$fake/stdout" "" "kids-fail stdout"
assert_file "$fake/rc" "1" "kids-fail rc"
assert_file "$fake/stderr" "sjmp: kids failed" "kids-fail stderr"
assert_file "$fake/cmds" $'list\njump\nkids' "kids-fail cmds"
[ ! -e "$fake/fzf-args-2" ] || fail "kids-fail opened a child picker"

# A jump failure stops before children are listed.
new_case
SJMP_JUMP_RC=1
printf '\n%s\n' "$row" > "$fake/out-1"
run_pick
assert_file "$fake/stdout" "" "jump-fail stdout"
assert_file "$fake/rc" "1" "jump-fail rc"
assert_file "$fake/cmds" $'list\njump' "jump-fail cmds"
[ ! -e "$fake/kids-path" ] || fail "jump-fail listed children"

echo "ok"

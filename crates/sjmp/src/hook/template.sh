# Copyright (c) 2026 Nathaniel T. Berry
# Licensed under the Apache License, Version 2.0.
#
# sjmp / symjump
# Run in a normal shell and in Emacs vterm (INSIDE_EMACS=vterm).
# comint and other Emacs shells still skip the hook.

case "${INSIDE_EMACS:-}" in
  ""|vterm) ;;
  *) return 0 2>/dev/null || exit 0 ;;
esac
sjmp_bin="$(command -v sjmp 2>/dev/null || true)"
[ -z "$sjmp_bin" ] && return 0 2>/dev/null || true
# Numbered picks 1-9 only while the query is empty (digits filter once you type).
sjmp_fzf() {
  local prompt="${1:-fav> }"
  local height="${2:-40%}"
  local header="${3:-}"
  local expect="${4:-}"
  local -a hdr=() exp=()
  [ -n "$header" ] && hdr=(--header "$header")
  # --expect prints the key on its own first line. Enter and 1-9 leave it empty.
  [ -n "$expect" ] && exp=(--expect="$expect")
  fzf --height="$height" --reverse --prompt="$prompt" "${hdr[@]}" "${exp[@]}" \
    --bind '1:pos(1)+accept,2:pos(2)+accept,3:pos(3)+accept,4:pos(4)+accept,5:pos(5)+accept,6:pos(6)+accept,7:pos(7)+accept,8:pos(8)+accept,9:pos(9)+accept' \
    --bind 'change:transform:[[ -z "{q}" ]] && echo rebind(1,2,3,4,5,6,7,8,9) || echo unbind(1,2,3,4,5,6,7,8,9)'
}
# Places and kids. Enter cds. a opens the default agent (grok, claude, or codex).
# l opens the local agent. Both only while the query is empty, so the letters still filter.
sjmp_fzf_nav() {
  local prompt="${1:-fav> }"
  local height="${2:-40%}"
  local modefile="$3"
  local modeq
  : > "$modefile"
  modeq=$(printf '%q' "$modefile")
  fzf --height="$height" --reverse --prompt="$prompt" --header 'enter: cd   a: __SJMP_AI_NAME__   l: __SJMP_AI_LOCAL_NAME__   esc: close' \
    --bind '1:pos(1)+accept,2:pos(2)+accept,3:pos(3)+accept,4:pos(4)+accept,5:pos(5)+accept,6:pos(6)+accept,7:pos(7)+accept,8:pos(8)+accept,9:pos(9)+accept' \
    --bind "a:execute-silent(printf a > $modeq)+accept" \
    --bind "l:execute-silent(printf l > $modeq)+accept" \
    --bind 'change:transform:[[ -z "{q}" ]] && echo "rebind(1,2,3,4,5,6,7,8,9,a,l)" || echo "unbind(1,2,3,4,5,6,7,8,9,a,l)"'
}
# Action palette. A letter, while the query is empty, runs the row whose key
# is that letter. Search uses the key and the label, not the kind word.
# t is a plain tmux session unless favorites.toml already uses t.
sjmp_fzf_action() {
  local rows="$1" modefile="$2"
  local dir row key key_q modeq names binds header t_payload status
  dir="$(mktemp -d)"
  modeq=$(printf '%q' "$modefile")
  names="1,2,3,4,5,6,7,8,9"
  binds='1:pos(1)+accept,2:pos(2)+accept,3:pos(3)+accept,4:pos(4)+accept,5:pos(5)+accept,6:pos(6)+accept,7:pos(7)+accept,8:pos(8)+accept,9:pos(9)+accept'
  printf '%s\n' "$rows" > "$dir/rows"
  while IFS= read -r row; do
    [ -n "$row" ] || continue
    key=$(printf '%s\n' "$row" | awk '{print $2}')
    case "$key" in
      [a-zA-Z]) ;;
      *) continue ;;
    esac
    # First row for a key wins. "rows" is the list file, not a key.
    [ -e "$dir/$key" ] && continue
    printf '%s\n' "$row" > "$dir/$key"
    key_q=$(printf '%q' "$dir/$key")
    binds="${binds},${key}:execute-silent(cp ${key_q} ${modeq})+abort"
    names="${names},${key}"
  done < "$dir/rows"
  t_payload="$(awk '$2 == "t" { for (i = 4; i <= NF; i++) printf "%s%s", $i, (i < NF ? " " : ""); exit }' "$dir/rows")"
  if [ "$t_payload" = sjmp_tmux ]; then
    header='letter: that key    t: tmux session    esc: close'
  else
    header='letter: that key    esc: close'
  fi
  printf '%s\n' "$rows" | fzf --height=20% --reverse --prompt='action> ' --header "$header" \
    --nth=2,3 \
    --bind "$binds" \
    --bind "change:transform:[[ -z \"{q}\" ]] && echo \"rebind(${names})\" || echo \"unbind(${names})\""
  status=$?
  rm -rf "$dir"
  return "$status"
}
# One path per line in, "NN<TAB>name<TAB>path" out. Numbers are always two digits.
sjmp_kid_rows() {
  awk '{
    n = split($0, parts, "/")
    base = parts[n]
    if (base == "") base = $0
    printf "%02d\t%s\t%s\n", NR, base, $0
  }'
}
# Child list. Digits stay in the query so 01 can be typed.
# --exact makes 01 match row 01 and not row 10.
sjmp_fzf_kids() {
  fzf --height=40% --reverse --prompt='kid> ' \
    --header 'number or name, then enter   esc: back' \
    --exact --delimiter=$'\t' --nth=1,2 --with-nth=1,2
}
# Place list for an agent. Enter returns that pin. Tab lists one level of
# children (sjmp kids) and returns the chosen child. Esc here aborts (130)
# so the action palette can come back. Esc on the child list stays here.
# A pin with no children stays on this list.
sjmp_pick_fav() {
  local out pressed line favkey path kids kid status rows
  while true; do
    out="$("$sjmp_bin" list | sjmp_fzf 'fav> ' '40%' 'enter: this dir   tab: children   esc: back' tab)" || return
    # fzf --expect: line 1 is "tab" or empty. Line 2 is the favorite row.
    pressed=$(printf '%s\n' "$out" | awk 'NR==1 { print; exit }')
    line=$(printf '%s\n' "$out" | awk 'NR==2 { print; exit }')
    [ -z "$line" ] && return 0
    favkey=$(printf '%s\n' "$line" | awk '{print $1}')
    [ -z "$favkey" ] && return 0
    path="$("$sjmp_bin" jump "$favkey")" || return
    [ -z "$path" ] && return 0
    if [ "$pressed" != tab ]; then
      printf '%s\n' "$path"
      return 0
    fi
    status=0
    kids="$("$sjmp_bin" kids "$path")" || status=$?
    [ "$status" -ne 0 ] && return "$status"
    if [ -z "$kids" ]; then
      echo "sjmp: no children: $path" >&2
      continue
    fi
    rows=$(printf '%s\n' "$kids" | sjmp_kid_rows)
    status=0
    kid="$(printf '%s\n' "$rows" | sjmp_fzf_kids)" || status=$?
    [ "$status" -eq 130 ] && continue
    [ "$status" -ne 0 ] && return "$status"
    [ -z "$kid" ] && continue
    # Shortest prefix: drop the number, then the name. The path may contain tabs.
    kid="${kid#*$'\t'}"
    kid="${kid#*$'\t'}"
    [ -z "$kid" ] && continue
    printf '%s\n' "$kid"
    return 0
  done
}
# Bare `ollama` lists local models in the pane. Nothing is loaded until one is picked.
# `ollama run <name>` is left alone so an explicit model still starts.
sjmp_ollama_pick() {
  local models model
  if ! command -v ollama >/dev/null 2>&1; then
    echo "sjmp: ollama is not installed." >&2
    return 1
  fi
  if ! command -v fzf >/dev/null 2>&1; then
    echo "sjmp: fzf is not installed." >&2
    return 1
  fi
  if ! models="$(ollama list)"; then
    echo "sjmp: ollama list failed." >&2
    return 1
  fi
  models="$(printf '%s\n' "$models" | awk 'NR>1 && NF { print $1 }')"
  if [ -z "$models" ]; then
    echo "sjmp: no ollama models yet. Pull one in this pane:" >&2
    echo "  ollama pull <name>" >&2
    return 1
  fi
  model="$(printf '%s\n' "$models" | fzf --reverse --height=100% --prompt='model> ')" || return 0
  [ -z "$model" ] && return 0
  exec ollama run "$model"
}
# Left pane runs [ai] from favorites.toml. SJMP_AGENT_CMD overrides it for one shell.
sjmp_tmux_slug() {
  local slug
  slug="$(basename -- "$1" | tr -c 'A-Za-z0-9_-' '-')"
  slug="${slug#-}"
  slug="${slug%-}"
  [ -n "$slug" ] || slug=work
  printf '%s\n' "$slug"
}
# Historical dashboard: left | right 66/34, and the right pane split 50/50.
sjmp_layout_fallback() {
  printf '%s\n' 'vertical' '66 0' '34 1 50 50'
}
# vertical is left | right (tmux -h). horizontal is top / bottom (tmux -v).
sjmp_layout_ok() {
  local text="$1" flat rest line i sum size count subsum s
  local -a lines=() tok=()
  [ -n "$text" ] || return 1
  flat="${text//$'\n'/ }"
  rest="${flat//[abcdefghijklmnopqrstuvwxyz0123456789 ]/}"
  [ -z "$rest" ] || return 1
  mapfile -t lines <<< "$text"
  [ "${lines[0]}" = vertical ] || [ "${lines[0]}" = horizontal ] || return 1
  [ "${#lines[@]}" -ge 2 ] && [ "${#lines[@]}" -le 5 ] || return 1
  sum=0
  for ((i = 1; i < ${#lines[@]}; i++)); do
    read -r -a tok <<< "${lines[i]}"
    [ "${#tok[@]}" -ge 2 ] || return 1
    size="${tok[0]}"
    count="${tok[1]}"
    case "$size" in
      ''|0*|*[!0-9]*) return 1 ;;
    esac
    case "$count" in
      0|1|2|3) ;;
      *) return 1 ;;
    esac
    [ "$size" -ge 1 ] && [ "$size" -le 100 ] || return 1
    if [ "$count" -eq 0 ]; then
      [ "${#tok[@]}" -eq 2 ] || return 1
    else
      [ "${#tok[@]}" -eq $((2 + count + 1)) ] || return 1
      subsum=0
      for ((s = 2; s < ${#tok[@]}; s++)); do
        case "${tok[s]}" in
          ''|0*|*[!0-9]*) return 1 ;;
        esac
        [ "${tok[s]}" -ge 1 ] && [ "${tok[s]}" -le 99 ] || return 1
        subsum=$((subsum + tok[s]))
      done
      [ "$subsum" -eq 100 ] || return 1
    fi
    if [ "${#lines[@]}" -eq 2 ]; then
      [ "$size" -eq 100 ] || return 1
    elif [ "$size" -gt 99 ]; then
      return 1
    fi
    sum=$((sum + size))
  done
  [ "$sum" -eq 100 ]
}
sjmp_layout_fetch() {
  local raw
  raw="$("$sjmp_bin" config layout 2>/dev/null)" || raw=""
  if ! sjmp_layout_ok "$raw"; then
    raw="$(sjmp_layout_fallback)"
  fi
  printf '%s\n' "$raw"
}
# Give each percent a whole number of cells. Earlier panes win a tie.
# The caller passes the usable size, after subtracting one cell per divider.
sjmp_share_cells() {
  local total="$1"
  shift
  local -a pct=("$@") cells=() rem=()
  local n=${#pct[@]} i raw c sum leftover best
  [ "$n" -ge 1 ] && [ "$total" -ge "$n" ] || return 1
  sum=0
  for i in "${!pct[@]}"; do
    raw=$((total * pct[i]))
    c=$((raw / 100))
    [ "$c" -ge 1 ] || c=1
    cells[i]=$c
    rem[i]=$((raw % 100))
    sum=$((sum + c))
  done
  [ "$sum" -le "$total" ] || return 1
  leftover=$((total - sum))
  while [ "$leftover" -gt 0 ]; do
    best=-1
    for i in "${!rem[@]}"; do
      if [ "$best" -lt 0 ] || [ "${rem[i]}" -gt "${rem[best]}" ]; then
        best=$i
      fi
    done
    if [ "$best" -lt 0 ] || [ "${rem[best]}" -lt 0 ]; then
      cells[$((n - 1))]=$((cells[n - 1] + leftover))
      break
    fi
    cells[best]=$((cells[best] + 1))
    rem[best]=-1
    leftover=$((leftover - 1))
  done
  printf '%s\n' "${cells[@]}"
}
# tmux -h is left | right. tmux -v is top / bottom.
sjmp_split_flag() {
  case "$1" in
    vertical) printf '%s\n' -h ;;
    horizontal) printf '%s\n' -v ;;
    *) return 1 ;;
  esac
}
# Print pane ids in order. The first id is the pane that was split.
sjmp_split_pane() {
  local pane="$1" axis="$2" dir="$3"
  shift 3
  local -a cells=("$@")
  local n=${#cells[@]} flag tail remaining i rest new
  flag="$(sjmp_split_flag "$axis")" || return 1
  printf '%s\n' "$pane"
  [ "$n" -le 1 ] && return 0
  tail="$pane"
  remaining=0
  for i in "${cells[@]}"; do
    remaining=$((remaining + i))
  done
  for ((i = 0; i < n - 1; i++)); do
    rest=$((remaining - cells[i]))
    [ "$rest" -ge 1 ] || return 1
    new="$(tmux split-window -t "$tail" "$flag" -P -F '#{pane_id}' -c "$dir" -l "$rest")" || return 1
    printf '%s\n' "$new"
    tail="$new"
    remaining=$rest
  done
}
sjmp_layout_apply() {
  local target="$1" dir="$2" agent="$3" layout="$4"
  local axis other total usable n i s sc stotal susable id all agent_pane cells_text ids_text
  local -a lines=() psizes=() tok=() ids=() cells=() ss=() scells=()
  mapfile -t lines <<< "$layout"
  axis="${lines[0]}"
  if [ "$axis" = vertical ]; then
    other=horizontal
  else
    other=vertical
  fi
  n=$((${#lines[@]} - 1))
  tmux set-option -w -t "$target" pane-border-status top
  tmux set-option -w -t "$target" pane-border-format '#{pane_index} #{pane_title}'
  # pane-base-index may be 1, so .0/.1/.2 do not name these panes.
  agent_pane="$(tmux display-message -p -t "$target" '#{pane_id}')"
  if [ "$axis" = vertical ]; then
    total="$(tmux display-message -p -t "$agent_pane" '#{pane_width}')"
  else
    total="$(tmux display-message -p -t "$agent_pane" '#{pane_height}')"
  fi
  for ((i = 0; i < n; i++)); do
    read -r -a tok <<< "${lines[i + 1]}"
    psizes[i]="${tok[0]}"
  done
  usable=$((total - (n - 1)))
  cells_text="$(sjmp_share_cells "$usable" "${psizes[@]}")" || return 1
  mapfile -t cells <<< "$cells_text"
  ids_text="$(sjmp_split_pane "$agent_pane" "$axis" "$dir" "${cells[@]}")" || return 1
  mapfile -t ids <<< "$ids_text"
  for ((i = 0; i < n; i++)); do
    read -r -a tok <<< "${lines[i + 1]}"
    sc="${tok[1]}"
    [ "$sc" -eq 0 ] && continue
    ss=()
    for ((s = 2; s < ${#tok[@]}; s++)); do
      ss+=("${tok[s]}")
    done
    if [ "$other" = vertical ]; then
      stotal="$(tmux display-message -p -t "${ids[i]}" '#{pane_width}')"
    else
      stotal="$(tmux display-message -p -t "${ids[i]}" '#{pane_height}')"
    fi
    susable=$((stotal - (${#ss[@]} - 1)))
    cells_text="$(sjmp_share_cells "$susable" "${ss[@]}")" || return 1
    mapfile -t scells <<< "$cells_text"
    sjmp_split_pane "${ids[i]}" "$other" "$dir" "${scells[@]}" >/dev/null || return 1
  done
  all="$(tmux list-panes -t "$target" -F '#{pane_id}')"
  while IFS= read -r id; do
    [ -n "$id" ] || continue
    if [ "$id" = "$agent_pane" ]; then
      tmux select-pane -t "$id" -T "${agent%% *}"
    else
      tmux select-pane -t "$id" -T shell
    fi
  done <<< "$all"
  tmux select-pane -t "$agent_pane"
}
sjmp_tmux_layout() {
  local target="$1" path="$2" agent="$3" layout
  layout="$(sjmp_layout_fetch)"
  sjmp_layout_apply "$target" "$path" "$agent" "$layout"
}
# Show a session. Inside tmux, switch the client. Outside, attach when
# stdout is a tty. Never adds a window.
sjmp_tmux_show() {
  local session="$1"
  if [ -n "${TMUX:-}" ]; then
    tmux switch-client -t "=$session"
  elif [ -t 1 ]; then
    tmux attach-session -t "=$session"
  fi
}
sjmp_tmux_dash() {
  local path="$1" agent slug session target pane_cmd title
  [ -d "$path" ] || { echo "sjmp: not a directory: $path" >&2; return 1; }
  path="$(cd -- "$path" && pwd)"
  if ! command -v tmux >/dev/null 2>&1; then
    echo "sjmp: tmux is not installed. Install tmux, then press a or l again." >&2
    return 1
  fi
  if [ "${2:-}" = local ]; then
    agent=__SJMP_AI_LOCAL__
  elif [ -n "${SJMP_AGENT_CMD:-}" ]; then
    agent="$SJMP_AGENT_CMD"
  else
    agent=__SJMP_AI_CMD__
  fi
  [ -n "$agent" ] || agent=grok
  title="${agent%% *}"
  if [ "$agent" = ollama ]; then
    agent="bash -c $(printf '%q' "$(declare -f sjmp_ollama_pick); sjmp_ollama_pick")"
    title=ollama
  fi
  slug="$(sjmp_tmux_slug "$path")"
  session="sjmp-${slug}"
  pane_cmd="${agent}; exec bash"
  # Tag the window as an agent session for the status bar. The EXIT trap
  # clears @agent when the pane's shell exits, even after exec bash.
  pane_cmd="bash -c $(printf '%q' "trap 'tmux set-window-option -t \"\$TMUX_PANE\" @agent \"\"' EXIT; $pane_cmd")"
  # Inside tmux, add a window in the current session. Other windows stay.
  # Outside, add a window to sjmp-<directory>, creating that session the first time.
  if [ -n "${TMUX:-}" ]; then
    target="$(tmux new-window -P -F '#{session_name}:#{window_index}' -c "$path" -n "$slug" "$pane_cmd")" || return
  elif tmux has-session -t "=$session" 2>/dev/null; then
    target="$(tmux new-window -d -P -F '#{session_name}:#{window_index}' -t "${session}:" -c "$path" -n "$slug" "$pane_cmd")" || return
  else
    tmux new-session -d -s "$session" -n "$slug" -c "$path" "$pane_cmd" || return
    target="$(tmux display-message -p -t "${session}:" '#{session_name}:#{window_index}')" || return
  fi
  tmux set-window-option -t "$target" @agent 1
  sjmp_tmux_layout "$target" "$path" "$title"
  if [ -z "${TMUX:-}" ]; then
    tmux select-window -t "$target"
    sjmp_tmux_show "$session"
  fi
}
# Plain session for one directory. No agent and no extra panes.
# Session name is sjmp-<directory>. If it exists, switch to it.
# A second open does not add a window.
sjmp_tmux() {
  local path slug session
  path="${1:-$PWD}"
  [ -d "$path" ] || { echo "sjmp: not a directory: $path" >&2; return 1; }
  path="$(cd -- "$path" && pwd)"
  if ! command -v tmux >/dev/null 2>&1; then
    echo "sjmp: tmux is not installed. Install tmux, then press t again." >&2
    return 1
  fi
  slug="$(sjmp_tmux_slug "$path")"
  session="sjmp-${slug}"
  if ! tmux has-session -t "=$session" 2>/dev/null; then
    tmux new-session -d -s "$session" -n "$slug" -c "$path" || return
  fi
  sjmp_tmux_show "$session"
}
jmp() {
  if [ $# -eq 0 ]; then
    local modefile out mode key dest
    modefile="$(mktemp)"
    out="$("$sjmp_bin" list | sjmp_fzf_nav 'fav> ' '40%' "$modefile")" || { rm -f "$modefile"; return; }
    mode="$(cat "$modefile" 2>/dev/null || true)"
    rm -f "$modefile"
    [ -z "$out" ] && return
    key=$(printf '%s\n' "$out" | awk '{print $1}')
    [ -z "$key" ] && return
    dest="$("$sjmp_bin" jump "$key")" || return
    if [ "$mode" = a ]; then
      sjmp_tmux_dash "$dest"
    elif [ "$mode" = l ]; then
      sjmp_tmux_dash "$dest" local
    else
      cd -- "$dest"
    fi
    return
  fi
  local dest
  dest="$("$sjmp_bin" jump "$1")" || return
  cd -- "$dest"
}
sjmp_places() { jmp; }
sjmp_kids() {
  local modefile out mode
  modefile="$(mktemp)"
  out="$("$sjmp_bin" kids | sjmp_fzf_nav 'kids> ' '40%' "$modefile")" || { rm -f "$modefile"; return; }
  mode="$(cat "$modefile" 2>/dev/null || true)"
  rm -f "$modefile"
  [ -z "$out" ] && return
  if [ "$mode" = a ]; then
    sjmp_tmux_dash "$out"
  elif [ "$mode" = l ]; then
    sjmp_tmux_dash "$out" local
  else
    cd -- "$out"
  fi
}
sjmp_actions() {
  local rows row kind payload sel modefile saved status
  rows="$("$sjmp_bin" action list)"
  if ! printf '%s\n' "$rows" | awk '$2 == "t" { found = 1 } END { exit !found }'; then
    if [ -n "$rows" ]; then
      rows="${rows}"$'\n'"session  t  tmux  sjmp_tmux"
    else
      rows="session  t  tmux  sjmp_tmux"
    fi
  fi
  # Esc closes this first picker. Esc on the favorite picker comes back here.
  while true; do
    modefile="$(mktemp)"
    : > "$modefile"
    if ! row="$(sjmp_fzf_action "$rows" "$modefile")"; then
      saved="$(cat "$modefile" 2>/dev/null || true)"
      rm -f "$modefile"
      row="$saved"
    else
      rm -f "$modefile"
    fi
    [ -z "$row" ] && return
    kind=$(printf '%s\n' "$row" | awk '{print $1}')
    payload=$(printf '%s\n' "$row" | awk '{for (i = 4; i <= NF; i++) printf "%s%s", $i, (i < NF ? " " : "\n")}')
    case "$kind" in
      toolbox)
        if [ -n "$payload" ]; then
          if command -v toolbox >/dev/null 2>&1; then
            toolbox enter "$payload"
          elif command -v distrobox >/dev/null 2>&1; then
            distrobox enter "$payload"
          else
            echo "sjmp: neither toolbox nor distrobox found" >&2
            return 1
          fi
        fi
        return
        ;;
      session)
        sjmp_tmux
        return
        ;;
      agent)
        # Bare sjmp_tmux is the default: this directory, no favorite prompt.
        # sjmp_tmux '{path}' still picks a favorite, then opens that directory.
        if [ "$payload" = sjmp_tmux ]; then
          sjmp_tmux
          return
        fi
        status=0
        sel="$(sjmp_pick_fav)" || status=$?
        # 130 is Esc or Ctrl-C from fzf. Other failures stop.
        [ "$status" -eq 130 ] && continue
        [ "$status" -ne 0 ] && return
        [ -z "$sel" ] && continue
        eval "$("$sjmp_bin" exec "$sel" --cmd "$payload")"
        return
        ;;
      *)
        return
        ;;
    esac
  done
}
# Readline macros, not bind -x: bash reverts cd after a bind -x widget.
# Non-interactive shells have no line editing. bind would warn there.
if [ -n "${BASH_VERSION:-}" ] && case $- in *i*) true ;; *) false ;; esac; then
  bind '"\ep": "\C-ujmp\C-m"'
  bind '"\eP": "\C-usjmp_kids\C-m"'
  # M-x opens the palette when readline receives it. vterm keeps M-x
  # for Emacs, so the shared actions chord there is M-Enter.
  bind '"\ex": "\C-usjmp_actions\C-m"'
  # Terminals send Enter as CR or LF.
  bind '"\e\C-m": "\C-usjmp_actions\C-m"'
  bind '"\e\C-j": "\C-usjmp_actions\C-m"'


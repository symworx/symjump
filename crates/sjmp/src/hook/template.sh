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
  local -a hdr=()
  [ -n "$header" ] && hdr=(--header "$header")
  fzf --height="$height" --reverse --prompt="$prompt" "${hdr[@]}" \
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
sjmp_pick_fav() {
  local line key
  line="$("$sjmp_bin" list | sjmp_fzf 'fav> ' '40%' 'esc: back')" || return
  [ -z "$line" ] && return
  key=$(printf '%s\n' "$line" | awk '{print $1}')
  [ -n "$key" ] && "$sjmp_bin" jump "$key"
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
sjmp_tmux_layout() {
  local target="$1" path="$2" agent="$3" left right bottom
  # pane-base-index may be 1, so .0/.1/.2 do not name these panes.
  left="$(tmux display-message -p -t "$target" '#{pane_id}')"
  # Left stays full height. The new pane is the right column, 34% wide.
  right="$(tmux split-window -t "$left" -h -P -F '#{pane_id}' -c "$path" -l 34%)"
  # Two shells in that column, one above the other.
  bottom="$(tmux split-window -t "$right" -v -P -F '#{pane_id}' -c "$path" -l 50%)"
  tmux set-option -w -t "$target" pane-border-status top
  tmux set-option -w -t "$target" pane-border-format '#{pane_index} #{pane_title}'
  tmux select-pane -t "$left" -T "${agent%% *}"
  tmux select-pane -t "$right" -T shell
  tmux select-pane -t "$bottom" -T shell
  tmux select-pane -t "$left"
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
  # One session per directory. A second open switches to it.
  if tmux has-session -t "=$session" 2>/dev/null; then
    sjmp_tmux_show "$session"
    return
  fi
  tmux new-session -d -s "$session" -n "$slug" -c "$path" "$pane_cmd" || return
  target="$(tmux display-message -p -t "${session}:" '#{session_name}:#{window_index}')" || return
  tmux set-window-option -t "$target" @agent 1
  sjmp_tmux_layout "$target" "$path" "$title"
  sjmp_tmux_show "$session"
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


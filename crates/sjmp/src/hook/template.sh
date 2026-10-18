# sjmp / symjump — skip inside Emacs
if [ -n "${INSIDE_EMACS:-}" ]; then
  return 0 2>/dev/null || exit 0
fi
sjmp_bin="$(command -v sjmp 2>/dev/null || true)"
[ -z "$sjmp_bin" ] && return 0 2>/dev/null || true
# Numbered picks 1-9 only while the query is empty (digits filter once you type).
sjmp_fzf() {
  local prompt="${1:-fav> }"
  local height="${2:-40%}"
  fzf --height="$height" --reverse --prompt="$prompt" \
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
  fzf --height="$height" --reverse --prompt="$prompt" --header 'enter: cd   a: __SJMP_AI_NAME__   l: __SJMP_AI_LOCAL_NAME__' \
    --bind '1:pos(1)+accept,2:pos(2)+accept,3:pos(3)+accept,4:pos(4)+accept,5:pos(5)+accept,6:pos(6)+accept,7:pos(7)+accept,8:pos(8)+accept,9:pos(9)+accept' \
    --bind "a:execute-silent(printf a > $modeq)+accept" \
    --bind "l:execute-silent(printf l > $modeq)+accept" \
    --bind 'change:transform:[[ -z "{q}" ]] && echo "rebind(1,2,3,4,5,6,7,8,9,a,l)" || echo "unbind(1,2,3,4,5,6,7,8,9,a,l)"'
}
# Action palette. t, while the query is empty, runs the action whose key is t.
# That is a plain tmux session unless favorites.toml already uses t.
sjmp_fzf_action() {
  local rows="$1" modefile="$2" tfile t_row t_payload header t_q modeq status
  t_row="$(printf '%s\n' "$rows" | awk '$2 == "t" { print; exit }')"
  t_payload="$(printf '%s\n' "$t_row" | awk '{for (i = 4; i <= NF; i++) printf "%s%s", $i, (i < NF ? " " : "")}')"
  if [ "$t_payload" = sjmp_tmux ]; then
    header='t: tmux session'
  else
    header='t: key t'
  fi
  tfile="$(mktemp)"
  printf '%s\n' "$t_row" > "$tfile"
  t_q=$(printf '%q' "$tfile")
  modeq=$(printf '%q' "$modefile")
  printf '%s\n' "$rows" | fzf --height=20% --reverse --prompt='action> ' --header "$header" \
    --bind '1:pos(1)+accept,2:pos(2)+accept,3:pos(3)+accept,4:pos(4)+accept,5:pos(5)+accept,6:pos(6)+accept,7:pos(7)+accept,8:pos(8)+accept,9:pos(9)+accept' \
    --bind "t:execute-silent(cp $t_q $modeq)+abort" \
    --bind 'change:transform:[[ -z "{q}" ]] && echo "rebind(1,2,3,4,5,6,7,8,9,t)" || echo "unbind(1,2,3,4,5,6,7,8,9,t)"'
  status=$?
  rm -f "$tfile"
  return "$status"
}
sjmp_pick_fav() {
  local line key
  line="$("$sjmp_bin" list | sjmp_fzf 'fav> ')" || return
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
  local target="$1" path="$2" agent="$3"
  tmux split-window -t "$target" -h -c "$path" -l 34%
  tmux split-window -t "${target}.1" -v -c "$path" -l 50%
  tmux set-option -w -t "$target" pane-border-status top
  tmux set-option -w -t "$target" pane-border-format '#{pane_index} #{pane_title}'
  tmux select-pane -t "${target}.0" -T "${agent%% *}"
  tmux select-pane -t "${target}.1" -T shell
  tmux select-pane -t "${target}.2" -T shell
  tmux select-pane -t "${target}.0"
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
  if [ -n "${TMUX:-}" ]; then
    target="$(tmux new-window -P -F '#{session_name}:#{window_index}' -c "$path" -n "$slug" "$pane_cmd")" || return
    sjmp_tmux_layout "$target" "$path" "$title"
  else
    if tmux has-session -t "=$session" 2>/dev/null; then
      target="$(tmux new-window -d -P -F '#{session_name}:#{window_index}' -t "${session}:" -c "$path" -n "$slug" "$pane_cmd")" || return
    else
      tmux new-session -d -s "$session" -n "$slug" -c "$path" "$pane_cmd" || return
      target="$(tmux display-message -p -t "${session}:" '#{session_name}:#{window_index}')" || return
    fi
    sjmp_tmux_layout "$target" "$path" "$title"
    if [ -t 1 ]; then
      tmux attach-session -t "=$session"
    fi
  fi
}
# Plain session for one directory. No agent and no extra panes.
# Inside tmux: a new window in the current session. Outside: attach to
# sjmp-<directory>, or create that session.
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
  if [ -n "${TMUX:-}" ]; then
    tmux new-window -c "$path" -n "$slug"
  elif tmux has-session -t "=$session" 2>/dev/null; then
    if [ -t 1 ]; then
      tmux attach-session -t "=$session"
    fi
  elif [ -t 1 ]; then
    tmux new-session -s "$session" -n "$slug" -c "$path"
  else
    tmux new-session -d -s "$session" -n "$slug" -c "$path"
  fi
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
  local rows row kind payload sel modefile saved
  rows="$("$sjmp_bin" action list)"
  if ! printf '%s\n' "$rows" | awk '$2 == "t" { found = 1 } END { exit !found }'; then
    if [ -n "$rows" ]; then
      rows="agent  t  tmux  sjmp_tmux"$'\n'"$rows"
    else
      rows="agent  t  tmux  sjmp_tmux"
    fi
  fi
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
      [ -n "$payload" ] && toolbox enter "$payload"
      ;;
    agent)
      # Bare sjmp_tmux is the default: this directory, no favorite prompt.
      # sjmp_tmux '{path}' still picks a favorite, then opens that directory.
      if [ "$payload" = sjmp_tmux ]; then
        sjmp_tmux
      else
        sel="$(sjmp_pick_fav)" || return
        [ -z "$sel" ] && return
        eval "$("$sjmp_bin" exec "$sel" --cmd "$payload")"
      fi
      ;;
  esac
}
if [ -n "${BASH_VERSION:-}" ]; then
  # Readline macros, not bind -x: bash reverts cd after a bind -x widget.
  bind '"\ep": "\C-ujmp\C-m"'
  bind '"\eP": "\C-usjmp_kids\C-m"'
  bind '"\ex": "\C-usjmp_actions\C-m"'
  # M-Enter is the same palette. Terminals send Enter as CR or LF.
  # A tmux root binding can consume M-Enter before readline sees it.
  bind '"\e\C-m": "\C-usjmp_actions\C-m"'
  bind '"\e\C-j": "\C-usjmp_actions\C-m"'

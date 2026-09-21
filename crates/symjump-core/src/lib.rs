// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

//! Pure operations over a [`symjump_config::Config`].
//! The CLI prints paths; the shell hook is responsible for `cd`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use symjump_config::{AgentAction, Config, Favorite, ToolboxAction, expand_user};

/// GNU readline emacs-mode Meta letters we never steal for favorite jumps.
const READLINE_META_RESERVED: &[(char, &str)] = &[
    ('b', "readline M-b backward-word"),
    ('f', "readline M-f forward-word"),
    ('d', "readline M-d kill-word"),
    ('y', "readline M-y yank-pop"),
];

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("no favorite matching `{0}`")]
    UnknownFavorite(String),
    #[error("no agent action matching `{0}`")]
    UnknownAgent(String),
    #[error("no toolbox action matching `{0}`")]
    UnknownToolbox(String),
    #[error("no action matching `{0}`")]
    UnknownAction(String),
    #[error("ambiguous action `{0}` (agent and toolbox); pass agent or toolbox")]
    AmbiguousAction(String),
    #[error("path is empty")]
    EmptyPath,
    #[error("key `{0}` is reserved ({1})")]
    KeyReserved(String, String),
    #[error("key `{0}` already used by `{1}`")]
    KeyTaken(String, String),
    #[error("action command is empty")]
    EmptyCmd,
    #[error("toolbox name is empty")]
    EmptyToolboxName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    pub label: String,
    pub path: PathBuf,
    pub keys: Option<String>,
}

pub fn resolve_favorite(cfg: &Config, query: &str, home: &Path) -> Result<Resolved, CoreError> {
    let q = query.trim();
    if q.is_empty() {
        return Err(CoreError::UnknownFavorite(query.into()));
    }
    let fav = cfg
        .favorites
        .iter()
        .find(|f| f.label.eq_ignore_ascii_case(q) || f.keys.as_deref() == Some(q))
        .ok_or_else(|| CoreError::UnknownFavorite(q.into()))?;
    Ok(resolve_one(fav, home))
}

pub fn resolve_one(fav: &Favorite, home: &Path) -> Resolved {
    Resolved {
        label: fav.label.clone(),
        path: expand_user(&fav.path, home),
        keys: fav.keys.clone(),
    }
}

pub fn list_favorites(cfg: &Config, home: &Path) -> Vec<Resolved> {
    cfg.favorites.iter().map(|f| resolve_one(f, home)).collect()
}

pub fn list_lines(cfg: &Config, home: &Path) -> Vec<String> {
    let rows: Vec<(String, String, String)> = list_favorites(cfg, home)
        .into_iter()
        .map(|r| {
            (
                r.keys.as_deref().unwrap_or("-").to_string(),
                r.label,
                r.path.display().to_string(),
            )
        })
        .collect();
    let kw = rows
        .iter()
        .map(|(k, _, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    let lw = rows
        .iter()
        .map(|(_, l, _)| l.chars().count())
        .max()
        .unwrap_or(0);
    rows.into_iter()
        .map(|(k, l, p)| format!("{k:<kw$}  {l:<lw$}  {p}"))
        .collect()
}

/// `kind  key  label  payload` with leading columns padded like [`list_lines`].
/// Payload (cmd or toolbox name) is last so it may contain spaces.
pub fn action_list_lines(cfg: &Config) -> Vec<String> {
    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    for a in &cfg.actions.agent {
        rows.push((
            "agent".into(),
            a.keys.as_deref().unwrap_or("-").to_string(),
            a.label.clone(),
            a.cmd.clone(),
        ));
    }
    for t in &cfg.actions.toolbox {
        rows.push((
            "toolbox".into(),
            t.keys.as_deref().unwrap_or("-").to_string(),
            t.label.clone(),
            t.name.clone(),
        ));
    }
    let kind_w = rows
        .iter()
        .map(|(k, _, _, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    let key_w = rows
        .iter()
        .map(|(_, k, _, _)| k.chars().count())
        .max()
        .unwrap_or(0);
    let label_w = rows
        .iter()
        .map(|(_, _, l, _)| l.chars().count())
        .max()
        .unwrap_or(0);
    rows.into_iter()
        .map(|(kind, key, label, payload)| {
            format!("{kind:<kind_w$}  {key:<key_w$}  {label:<label_w$}  {payload}")
        })
        .collect()
}

pub fn meta_letter(spec: &str) -> Option<char> {
    let rest = spec.strip_prefix("M-")?;
    let mut chars = rest.chars();
    let c = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    Some(c)
}

pub fn reserved_meta_letters(cfg: &Config) -> BTreeSet<char> {
    let mut out = BTreeSet::new();
    for spec in [&cfg.keys.leader, &cfg.keys.kids, &cfg.keys.actions] {
        if let Some(c) = meta_letter(spec) {
            out.insert(c);
        }
    }
    out.extend(READLINE_META_RESERVED.iter().map(|(c, _)| *c));
    out
}

fn reserved_reason(cfg: &Config, c: char) -> Option<String> {
    if meta_letter(&cfg.keys.leader) == Some(c) {
        return Some(format!("{} places", cfg.keys.leader));
    }
    if meta_letter(&cfg.keys.kids) == Some(c) {
        return Some(format!("{} kids", cfg.keys.kids));
    }
    if meta_letter(&cfg.keys.actions) == Some(c) {
        return Some(format!("{} actions", cfg.keys.actions));
    }
    READLINE_META_RESERVED
        .iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, why)| (*why).to_string())
}

/// Single-letter keys become `M-<letter>`. Reject sjmp chords, readline
/// motion/kill letters, and keys already used by another favorite.
pub fn check_favorite_key(
    cfg: &Config,
    key: &str,
    except_label: Option<&str>,
) -> Result<(), CoreError> {
    let k = key.trim();
    if k.is_empty() {
        return Ok(());
    }
    let mut chars = k.chars();
    if let (Some(c), None) = (chars.next(), chars.next()) {
        if let Some(why) = reserved_reason(cfg, c) {
            return Err(CoreError::KeyReserved(k.into(), why));
        }
    }
    if let Some(other) = cfg.favorites.iter().find(|f| {
        f.keys.as_deref() == Some(k)
            && except_label.is_none_or(|lab| !f.label.eq_ignore_ascii_case(lab))
    }) {
        return Err(CoreError::KeyTaken(k.into(), other.label.clone()));
    }
    Ok(())
}

pub fn pin(
    cfg: &mut Config,
    label: String,
    path: String,
    keys: Option<String>,
) -> Result<(), CoreError> {
    if let Some(k) = keys.as_deref() {
        check_favorite_key(cfg, k, Some(&label))?;
    }
    if let Some(existing) = cfg
        .favorites
        .iter_mut()
        .find(|f| f.label.eq_ignore_ascii_case(&label))
    {
        existing.path = path;
        if keys.is_some() {
            existing.keys = keys;
        }
        return Ok(());
    }
    cfg.favorites.push(Favorite { label, path, keys });
    Ok(())
}

fn action_key_taken<'a>(cfg: &'a Config, key: &str, except_label: Option<&str>) -> Option<&'a str> {
    cfg.actions
        .agent
        .iter()
        .find(|a| {
            a.keys.as_deref() == Some(key)
                && except_label.is_none_or(|lab| !a.label.eq_ignore_ascii_case(lab))
        })
        .map(|a| a.label.as_str())
        .or_else(|| {
            cfg.actions
                .toolbox
                .iter()
                .find(|t| {
                    t.keys.as_deref() == Some(key)
                        && except_label.is_none_or(|lab| !t.label.eq_ignore_ascii_case(lab))
                })
                .map(|t| t.label.as_str())
        })
}

pub fn check_action_key(
    cfg: &Config,
    key: &str,
    except_label: Option<&str>,
) -> Result<(), CoreError> {
    let k = key.trim();
    if k.is_empty() {
        return Ok(());
    }
    if let Some(other) = action_key_taken(cfg, k, except_label) {
        return Err(CoreError::KeyTaken(k.into(), other.into()));
    }
    Ok(())
}

pub fn add_agent(
    cfg: &mut Config,
    label: String,
    cmd: String,
    keys: Option<String>,
) -> Result<(), CoreError> {
    if cmd.trim().is_empty() {
        return Err(CoreError::EmptyCmd);
    }
    if let Some(k) = keys.as_deref() {
        check_action_key(cfg, k, Some(&label))?;
    }
    if let Some(existing) = cfg
        .actions
        .agent
        .iter_mut()
        .find(|a| a.label.eq_ignore_ascii_case(&label))
    {
        existing.cmd = cmd;
        if keys.is_some() {
            existing.keys = keys;
        }
        return Ok(());
    }
    cfg.actions.agent.push(AgentAction { label, cmd, keys });
    Ok(())
}

pub fn add_toolbox(
    cfg: &mut Config,
    label: String,
    name: String,
    keys: Option<String>,
) -> Result<(), CoreError> {
    if name.trim().is_empty() {
        return Err(CoreError::EmptyToolboxName);
    }
    if let Some(k) = keys.as_deref() {
        check_action_key(cfg, k, Some(&label))?;
    }
    if let Some(existing) = cfg
        .actions
        .toolbox
        .iter_mut()
        .find(|t| t.label.eq_ignore_ascii_case(&label))
    {
        existing.name = name;
        if keys.is_some() {
            existing.keys = keys;
        }
        return Ok(());
    }
    cfg.actions
        .toolbox
        .push(ToolboxAction { label, name, keys });
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    Agent,
    Toolbox,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemovedAction {
    Agent(AgentAction),
    Toolbox(ToolboxAction),
}

impl RemovedAction {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Agent(_) => "agent",
            Self::Toolbox(_) => "toolbox",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Agent(a) => &a.label,
            Self::Toolbox(t) => &t.label,
        }
    }
}

fn action_matches_query(label: &str, keys: Option<&str>, query: &str) -> bool {
    label.eq_ignore_ascii_case(query) || keys == Some(query)
}

/// Remove an agent or toolbox action by label or key.
/// Pass `kind` when the same query hits both lists.
pub fn remove_action(
    cfg: &mut Config,
    query: &str,
    kind: Option<ActionKind>,
) -> Result<RemovedAction, CoreError> {
    let q = query.trim();
    if q.is_empty() {
        return Err(match kind {
            Some(ActionKind::Agent) => CoreError::UnknownAgent(query.into()),
            Some(ActionKind::Toolbox) => CoreError::UnknownToolbox(query.into()),
            None => CoreError::UnknownAction(query.into()),
        });
    }
    let agent_idx = cfg
        .actions
        .agent
        .iter()
        .position(|a| action_matches_query(&a.label, a.keys.as_deref(), q));
    let toolbox_idx = cfg
        .actions
        .toolbox
        .iter()
        .position(|t| action_matches_query(&t.label, t.keys.as_deref(), q));
    match kind {
        Some(ActionKind::Agent) => {
            let idx = agent_idx.ok_or_else(|| CoreError::UnknownAgent(q.into()))?;
            Ok(RemovedAction::Agent(cfg.actions.agent.remove(idx)))
        }
        Some(ActionKind::Toolbox) => {
            let idx = toolbox_idx.ok_or_else(|| CoreError::UnknownToolbox(q.into()))?;
            Ok(RemovedAction::Toolbox(cfg.actions.toolbox.remove(idx)))
        }
        None => match (agent_idx, toolbox_idx) {
            (Some(_), Some(_)) => Err(CoreError::AmbiguousAction(q.into())),
            (Some(i), None) => Ok(RemovedAction::Agent(cfg.actions.agent.remove(i))),
            (None, Some(i)) => Ok(RemovedAction::Toolbox(cfg.actions.toolbox.remove(i))),
            (None, None) => Err(CoreError::UnknownAction(q.into())),
        },
    }
}

pub fn unpin(cfg: &mut Config, query: &str) -> Result<Favorite, CoreError> {
    let q = query.trim();
    if q.is_empty() {
        return Err(CoreError::UnknownFavorite(query.into()));
    }
    let idx = cfg
        .favorites
        .iter()
        .position(|f| f.label.eq_ignore_ascii_case(q) || f.keys.as_deref() == Some(q))
        .ok_or_else(|| CoreError::UnknownFavorite(q.into()))?;
    Ok(cfg.favorites.remove(idx))
}

pub fn is_git_root(dir: &Path) -> bool {
    dir.join(".git").exists()
}

pub fn kids(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    const SKIP: &[&str] = &[
        "target",
        "node_modules",
        ".git",
        ".venv",
        "venv",
        "__pycache__",
    ];
    let mut out = Vec::new();
    if !dir.is_dir() {
        return Ok(out);
    }
    for ent in std::fs::read_dir(dir)? {
        let ent = ent?;
        let name = ent.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || SKIP.contains(&name.as_ref()) {
            continue;
        }
        let p = ent.path();
        if p.is_dir() {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}

pub fn render_agent_cmd(action: &AgentAction, path: &Path) -> String {
    action.cmd.replace("{path}", &path.display().to_string())
}

pub fn find_agent<'a>(cfg: &'a Config, query: &str) -> Result<&'a AgentAction, CoreError> {
    cfg.actions
        .agent
        .iter()
        .find(|a| a.label.eq_ignore_ascii_case(query) || a.keys.as_deref() == Some(query))
        .ok_or_else(|| CoreError::UnknownAgent(query.into()))
}

pub fn find_toolbox<'a>(cfg: &'a Config, query: &str) -> Result<&'a ToolboxAction, CoreError> {
    cfg.actions
        .toolbox
        .iter()
        .find(|a| a.label.eq_ignore_ascii_case(query) || a.keys.as_deref() == Some(query))
        .ok_or_else(|| CoreError::UnknownToolbox(query.into()))
}

pub fn toolbox_enter_cmd(action: &ToolboxAction) -> String {
    format!("toolbox enter {}", action.name)
}

pub fn exec_line(cmd: &str, path: &Path) -> Result<String, CoreError> {
    if path.as_os_str().is_empty() {
        return Err(CoreError::EmptyPath);
    }
    if cmd.contains("{path}") {
        Ok(cmd.replace("{path}", &path.display().to_string()))
    } else {
        Ok(format!("cd {} && {}", path.display(), cmd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use symjump_config::{Actions, AgentAction, Config, Favorite, ToolboxAction};

    fn cfg() -> Config {
        Config {
            favorites: vec![
                Favorite {
                    label: "src".into(),
                    path: "~/src".into(),
                    keys: Some("r".into()),
                },
                Favorite {
                    label: "symworx".into(),
                    path: "~/src/symworx".into(),
                    keys: Some("s".into()),
                },
            ],
            actions: Actions {
                toolbox: vec![ToolboxAction {
                    label: "python".into(),
                    name: "dev-python".into(),
                    keys: Some("p".into()),
                }],
                agent: vec![
                    AgentAction {
                        label: "grok-build".into(),
                        cmd: "grok".into(),
                        keys: Some("g".into()),
                    },
                    AgentAction {
                        label: "codex".into(),
                        cmd: "codex -C {path}".into(),
                        keys: Some("c".into()),
                    },
                ],
            },
            ..Config::default()
        }
    }

    #[test]
    fn resolve_by_label_and_key() {
        let home = Path::new("/home/user");
        let c = cfg();
        let r = resolve_favorite(&c, "s", home).unwrap();
        assert_eq!(r.label, "symworx");
        assert_eq!(r.path, PathBuf::from("/home/user/src/symworx"));
        assert!(matches!(
            resolve_favorite(&c, "nope", home),
            Err(CoreError::UnknownFavorite(_))
        ));
    }

    #[test]
    fn list_columns_align() {
        let lines = list_lines(&cfg(), Path::new("/h"));
        assert_eq!(lines[0], "r  src      /h/src");
        assert_eq!(lines[1], "s  symworx  /h/src/symworx");
        let col = lines[0].find("/h/").unwrap();
        assert_eq!(lines[1].find("/h/").unwrap(), col);
    }

    #[test]
    fn action_list_columns_align() {
        let lines = action_list_lines(&cfg());
        assert_eq!(lines[0], "agent    g  grok-build  grok");
        assert_eq!(lines[1], "agent    c  codex       codex -C {path}");
        assert_eq!(lines[2], "toolbox  p  python      dev-python");
        let col = lines[0].len() - "grok".len();
        assert_eq!(&lines[1][col..], "codex -C {path}");
        assert_eq!(&lines[2][col..], "dev-python");
    }

    #[test]
    fn pin_updates_or_appends() {
        let mut c = cfg();
        pin(&mut c, "symworx".into(), "~/new".into(), None).unwrap();
        assert_eq!(c.favorites[1].path, "~/new");
        pin(&mut c, "lab".into(), "~/lab".into(), Some("l".into())).unwrap();
        assert_eq!(c.favorites.len(), 3);
        let gone = unpin(&mut c, "s").unwrap();
        assert_eq!(gone.label, "symworx");
        assert_eq!(c.favorites.len(), 2);
        assert!(matches!(
            unpin(&mut c, "nope"),
            Err(CoreError::UnknownFavorite(_))
        ));
    }

    #[test]
    fn pin_rejects_reserved_and_taken_keys() {
        let mut c = cfg();
        assert!(matches!(
            pin(&mut c, "nope".into(), "~/x".into(), Some("x".into())),
            Err(CoreError::KeyReserved(k, _)) if k == "x"
        ));
        assert!(matches!(
            pin(&mut c, "nope".into(), "~/x".into(), Some("p".into())),
            Err(CoreError::KeyReserved(k, _)) if k == "p"
        ));
        assert!(matches!(
            pin(&mut c, "nope".into(), "~/x".into(), Some("b".into())),
            Err(CoreError::KeyReserved(k, _)) if k == "b"
        ));
        assert!(matches!(
            pin(&mut c, "lab".into(), "~/lab".into(), Some("s".into())),
            Err(CoreError::KeyTaken(k, lab)) if k == "s" && lab == "symworx"
        ));
        pin(
            &mut c,
            "symworx".into(),
            "~/src/symworx".into(),
            Some("s".into()),
        )
        .unwrap();
    }

    #[test]
    fn add_action_updates_and_rejects_taken_keys() {
        let mut c = cfg();
        add_agent(&mut c, "grok-build".into(), "grok --foo".into(), None).unwrap();
        assert_eq!(c.actions.agent[0].cmd, "grok --foo");
        assert!(matches!(
            add_agent(&mut c, "other".into(), "echo".into(), Some("g".into())),
            Err(CoreError::KeyTaken(k, lab)) if k == "g" && lab == "grok-build"
        ));
        add_toolbox(&mut c, "rust".into(), "dev-rust".into(), Some("r".into())).unwrap();
        assert_eq!(c.actions.toolbox.len(), 2);
        assert!(matches!(
            add_agent(&mut c, "x".into(), " ".into(), None),
            Err(CoreError::EmptyCmd)
        ));
    }

    #[test]
    fn remove_action_by_label_key_and_kind() {
        let mut c = cfg();
        let gone = remove_action(&mut c, "g", None).unwrap();
        assert_eq!(gone.kind(), "agent");
        assert_eq!(gone.label(), "grok-build");
        assert_eq!(c.actions.agent.len(), 1);
        let tb = remove_action(&mut c, "python", None).unwrap();
        assert_eq!(
            tb,
            RemovedAction::Toolbox(ToolboxAction {
                label: "python".into(),
                name: "dev-python".into(),
                keys: Some("p".into()),
            })
        );
        assert!(c.actions.toolbox.is_empty());
        assert!(matches!(
            remove_action(&mut c, "nope", None),
            Err(CoreError::UnknownAction(q)) if q == "nope"
        ));

        add_agent(&mut c, "python".into(), "echo".into(), Some("a".into())).unwrap();
        add_toolbox(
            &mut c,
            "python".into(),
            "dev-python".into(),
            Some("t".into()),
        )
        .unwrap();
        assert!(matches!(
            remove_action(&mut c, "python", None),
            Err(CoreError::AmbiguousAction(q)) if q == "python"
        ));
        let agent = remove_action(&mut c, "python", Some(ActionKind::Agent)).unwrap();
        assert_eq!(agent.kind(), "agent");
        let toolbox = remove_action(&mut c, "t", Some(ActionKind::Toolbox)).unwrap();
        assert_eq!(toolbox.label(), "python");
        assert!(matches!(
            remove_action(&mut c, "python", Some(ActionKind::Agent)),
            Err(CoreError::UnknownAgent(_))
        ));
    }

    #[test]
    fn agent_and_toolbox_lookup() {
        let c = cfg();
        assert_eq!(find_agent(&c, "g").unwrap().label, "grok-build");
        assert_eq!(
            toolbox_enter_cmd(find_toolbox(&c, "p").unwrap()),
            "toolbox enter dev-python"
        );
        assert_eq!(
            render_agent_cmd(find_agent(&c, "c").unwrap(), Path::new("/r")),
            "codex -C /r"
        );
    }

    #[test]
    fn exec_line_cd_or_substitute() {
        assert_eq!(
            exec_line("grok", Path::new("/proj")).unwrap(),
            "cd /proj && grok"
        );
        assert_eq!(
            exec_line("codex -C {path}", Path::new("/proj")).unwrap(),
            "codex -C /proj"
        );
    }
}

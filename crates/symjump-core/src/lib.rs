//! Pure operations over a [`symjump_config::Config`].
//! The CLI prints paths; the shell hook is responsible for `cd`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use symjump_config::{expand_user, AgentAction, Config, Favorite, ToolboxAction};

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
    #[error("path is empty")]
    EmptyPath,
    #[error("key `{0}` is reserved ({1})")]
    KeyReserved(String, String),
    #[error("key `{0}` already used by `{1}`")]
    KeyTaken(String, String),
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
    let kw = rows.iter().map(|(k, _, _)| k.chars().count()).max().unwrap_or(0);
    let lw = rows.iter().map(|(_, l, _)| l.chars().count()).max().unwrap_or(0);
    rows.into_iter()
        .map(|(k, l, p)| format!("{k:<kw$}  {l:<lw$}  {p}"))
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
pub fn check_favorite_key(cfg: &Config, key: &str, except_label: Option<&str>) -> Result<(), CoreError> {
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

pub fn pin(cfg: &mut Config, label: String, path: String, keys: Option<String>) -> Result<(), CoreError> {
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
    const SKIP: &[&str] = &["target", "node_modules", ".git", ".venv", "venv", "__pycache__"];
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
        assert!(matches!(resolve_favorite(&c, "nope", home), Err(CoreError::UnknownFavorite(_))));
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
    fn pin_updates_or_appends() {
        let mut c = cfg();
        pin(&mut c, "symworx".into(), "~/new".into(), None).unwrap();
        assert_eq!(c.favorites[1].path, "~/new");
        pin(&mut c, "lab".into(), "~/lab".into(), Some("l".into())).unwrap();
        assert_eq!(c.favorites.len(), 3);
        let gone = unpin(&mut c, "s").unwrap();
        assert_eq!(gone.label, "symworx");
        assert_eq!(c.favorites.len(), 2);
        assert!(matches!(unpin(&mut c, "nope"), Err(CoreError::UnknownFavorite(_))));
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
        pin(&mut c, "symworx".into(), "~/src/symworx".into(), Some("s".into())).unwrap();
    }

    #[test]
    fn agent_and_toolbox_lookup() {
        let c = cfg();
        assert_eq!(find_agent(&c, "g").unwrap().label, "grok-build");
        assert_eq!(toolbox_enter_cmd(find_toolbox(&c, "p").unwrap()), "toolbox enter dev-python");
        assert_eq!(render_agent_cmd(find_agent(&c, "c").unwrap(), Path::new("/r")), "codex -C /r");
    }

    #[test]
    fn exec_line_cd_or_substitute() {
        assert_eq!(exec_line("grok", Path::new("/proj")).unwrap(), "cd /proj && grok");
        assert_eq!(exec_line("codex -C {path}", Path::new("/proj")).unwrap(), "codex -C /proj");
    }
}

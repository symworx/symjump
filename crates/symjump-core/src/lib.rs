//! Pure operations over a [`symjump_config::Config`].
//! The CLI prints paths; the shell hook is responsible for `cd`.

use std::path::{Path, PathBuf};
use symjump_config::{expand_user, AgentVerb, Config, Favorite, ToolboxVerb};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("no favorite matching `{0}`")]
    UnknownFavorite(String),
    #[error("no agent verb matching `{0}`")]
    UnknownAgent(String),
    #[error("no toolbox verb matching `{0}`")]
    UnknownToolbox(String),
    #[error("path is empty")]
    EmptyPath,
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
    list_favorites(cfg, home)
        .into_iter()
        .map(|r| {
            format!(
                "{}\t{}\t{}",
                r.keys.as_deref().unwrap_or("-"),
                r.label,
                r.path.display()
            )
        })
        .collect()
}

pub fn pin(cfg: &mut Config, label: String, path: String, keys: Option<String>) {
    if let Some(existing) = cfg
        .favorites
        .iter_mut()
        .find(|f| f.label.eq_ignore_ascii_case(&label))
    {
        existing.path = path;
        if keys.is_some() {
            existing.keys = keys;
        }
        return;
    }
    cfg.favorites.push(Favorite { label, path, keys });
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

pub fn render_agent_cmd(verb: &AgentVerb, path: &Path) -> String {
    verb.cmd.replace("{path}", &path.display().to_string())
}

pub fn find_agent<'a>(cfg: &'a Config, query: &str) -> Result<&'a AgentVerb, CoreError> {
    cfg.verbs
        .agent
        .iter()
        .find(|v| v.label.eq_ignore_ascii_case(query) || v.keys.as_deref() == Some(query))
        .ok_or_else(|| CoreError::UnknownAgent(query.into()))
}

pub fn find_toolbox<'a>(cfg: &'a Config, query: &str) -> Result<&'a ToolboxVerb, CoreError> {
    cfg.verbs
        .toolbox
        .iter()
        .find(|v| v.label.eq_ignore_ascii_case(query) || v.keys.as_deref() == Some(query))
        .ok_or_else(|| CoreError::UnknownToolbox(query.into()))
}

pub fn toolbox_enter_cmd(verb: &ToolboxVerb) -> String {
    format!("toolbox enter {}", verb.name)
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
    use symjump_config::{AgentVerb, Config, Favorite, ToolboxVerb, Verbs};

    fn cfg() -> Config {
        Config {
            favorites: vec![
                Favorite {
                    label: "worx".into(),
                    path: "~/worx".into(),
                    keys: Some("w".into()),
                },
                Favorite {
                    label: "symworx".into(),
                    path: "~/worx/symworx".into(),
                    keys: Some("s".into()),
                },
            ],
            verbs: Verbs {
                toolbox: vec![ToolboxVerb {
                    label: "python".into(),
                    name: "dev-python".into(),
                    keys: Some("p".into()),
                }],
                agent: vec![
                    AgentVerb {
                        label: "grok-build".into(),
                        cmd: "grok".into(),
                        keys: Some("g".into()),
                    },
                    AgentVerb {
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
        let home = Path::new("/home/nate");
        let c = cfg();
        let r = resolve_favorite(&c, "s", home).unwrap();
        assert_eq!(r.label, "symworx");
        assert_eq!(r.path, PathBuf::from("/home/nate/worx/symworx"));
        assert!(matches!(resolve_favorite(&c, "nope", home), Err(CoreError::UnknownFavorite(_))));
    }

    #[test]
    fn list_is_stable_tsv() {
        let lines = list_lines(&cfg(), Path::new("/h"));
        assert_eq!(lines[0], "w\tworx\t/h/worx");
        assert_eq!(lines[1], "s\tsymworx\t/h/worx/symworx");
    }

    #[test]
    fn pin_updates_or_appends() {
        let mut c = cfg();
        pin(&mut c, "symworx".into(), "~/new".into(), None);
        assert_eq!(c.favorites[1].path, "~/new");
        pin(&mut c, "lab".into(), "~/lab".into(), Some("l".into()));
        assert_eq!(c.favorites.len(), 3);
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

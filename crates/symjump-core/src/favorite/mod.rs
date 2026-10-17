// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

mod list;

use std::path::{Path, PathBuf};

use symjump_config::{Config, Favorite, expand_user};

use crate::error::CoreError;
use crate::keys::check_favorite_key;

pub use list::list_lines;

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

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::error::CoreError;
    use crate::test_support::cfg;

    use super::{pin, resolve_favorite, unpin};

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
}

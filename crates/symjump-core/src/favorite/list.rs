// Copyright (c) 2026 Nathaniel T. Berry
// Licensed under the Apache License, Version 2.0.

use std::path::Path;

use symjump_config::Config;

use super::list_favorites;

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::test_support::cfg;

    use super::list_lines;

    #[test]
    fn list_columns_align() {
        let lines = list_lines(&cfg(), Path::new("/h"));
        assert_eq!(lines[0], "r  src      /h/src");
        assert_eq!(lines[1], "s  symworx  /h/src/symworx");
        let col = lines[0].find("/h/").unwrap();
        assert_eq!(lines[1].find("/h/").unwrap(), col);
    }
}

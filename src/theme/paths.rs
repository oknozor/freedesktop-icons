use crate::theme;
use crate::theme::error::ThemeError;
use dirs::{data_dir, home_dir};
use ini::Ini;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub(crate) static BASE_PATHS: Lazy<Vec<PathBuf>> = Lazy::new(icon_theme_base_paths);

/// Look in $HOME/.icons (for backwards compatibility), in $XDG_DATA_DIRS/icons and in /usr/share/pixmaps (in that order).
/// Paths that are not found are filtered out.
fn icon_theme_base_paths() -> Vec<PathBuf> {
    let home_icon_dir = home_dir().expect("No $HOME directory").join(".icons");
    let usr_data_dir = data_dir().expect("No $XDG_DATA_DIR").join("icons");
    let xdg_data_dirs_local = PathBuf::from("/usr/local/share/").join("icons");
    let xdg_data_dirs = PathBuf::from("/usr/share/").join("icons");

    [
        home_icon_dir,
        usr_data_dir,
        xdg_data_dirs_local,
        xdg_data_dirs,
    ]
    .into_iter()
    .filter(|p| p.exists())
    .collect()
}

#[derive(Debug)]
pub struct ThemePath(pub PathBuf);

impl ThemePath {
    pub(super) fn index(&self) -> theme::Result<Ini> {
        let index = self.0.join("index.theme");

        if !index.exists() {
            return Err(ThemeError::ThemeIndexNotFound(index));
        }

        Ok(Ini::load_from_file(index)?)
    }
}

#[cfg(test)]
mod test {
    use crate::theme::paths::icon_theme_base_paths;
    use crate::theme::{get_all_themes, Theme};
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn should_get_all_themes() {
        let themes = get_all_themes().unwrap();
        assert_that!(themes.get("hicolor")).is_some();
    }

    #[test]
    fn should_get_theme_paths_ordered() {
        let base_paths = icon_theme_base_paths();
        assert_that!(base_paths).is_not_empty()
    }

    #[test]
    fn should_read_theme_index() -> Result<()> {
        let themes = get_all_themes()?;
        let themes: Vec<&Theme> = themes.values().collect();
        assert_that!(themes).is_not_empty();
        Ok(())
    }
}

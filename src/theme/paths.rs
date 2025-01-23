use std::path::PathBuf;

use dirs::home_dir;
use once_cell::sync::Lazy;
use xdg::BaseDirectories;

use crate::theme;
use crate::theme::error::ThemeError;

pub(crate) static BASE_PATHS: Lazy<Vec<PathBuf>> = Lazy::new(icon_theme_base_paths);

/// Look in $HOME/.icons (for backwards compatibility), in $XDG_DATA_DIRS/icons, in $XDG_DATA_DIRS/pixmaps and in /usr/share/pixmaps (in that order).
/// Paths that are not found are filtered out.
fn icon_theme_base_paths() -> Vec<PathBuf> {
    let mut data_dirs: Vec<_> = BaseDirectories::new()
        .map(|bd| {
            let mut data_dirs: Vec<_> = bd
                .get_data_dirs()
                .into_iter()
                .flat_map(|p| [p.join("icons"), p.join("pixmaps")])
                .collect();
            let data_home = bd.get_data_home();
            data_dirs.push(data_home.join("icons"));
            data_dirs.push(data_home.join("pixmaps"));
            data_dirs
        })
        .unwrap_or_default();
    match home_dir().map(|home| home.join(".icons")) {
        Some(home_icon_dir) => data_dirs.push(home_icon_dir),
        None => tracing::warn!("No $HOME directory found"),
    }
    data_dirs.into_iter().filter(|p| p.exists()).collect()
}

#[derive(Clone, Debug)]
pub struct ThemePath(pub PathBuf);

impl ThemePath {
    pub(super) fn index(&self) -> theme::Result<PathBuf> {
        let index = self.0.join("index.theme");

        if !index.exists() {
            return Err(ThemeError::ThemeIndexNotFound(index));
        }

        Ok(index)
    }
}

#[cfg(test)]
mod test {
    use crate::theme::paths::icon_theme_base_paths;
    use crate::theme::{get_all_themes, Theme};
    use speculoos::prelude::*;

    #[test]
    fn should_get_all_themes() {
        let themes = get_all_themes();
        assert_that!(themes.get("hicolor")).is_some();
    }

    #[test]
    fn should_get_theme_paths_ordered() {
        let base_paths = icon_theme_base_paths();
        assert_that!(base_paths).is_not_empty()
    }

    #[test]
    fn should_read_theme_index() {
        let themes = get_all_themes();
        let themes: Vec<&Theme> = themes.values().flatten().collect();
        assert_that!(themes).is_not_empty();
    }
}

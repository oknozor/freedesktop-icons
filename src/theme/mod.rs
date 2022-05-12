use crate::theme::error::ThemeError;
use dirs::{data_dir, home_dir};
use ini::Ini;
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::io;
use std::path::PathBuf;

pub mod error;

type Result<T> = std::result::Result<T, error::ThemeError>;

const HICOLOR: &str = "/usr/share/pixmaps";

#[derive(Debug)]
struct ThemePath(PathBuf);

struct ThemeIndex(Ini);

impl Debug for ThemeIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut content = vec![];
        self.0.write_to(&mut content).expect("Write error");
        let content = String::from_utf8_lossy(&content);
        writeln!(f, "ThemeIndex({content:?})")
    }
}

impl ThemePath {
    fn name(&self) -> Cow<'_, str> {
        // Unwrapping is safe here, we just got the path from [`list_icon_themes`]
        self.0.file_name().unwrap().to_string_lossy()
    }

    fn index(&self) -> Result<ThemeIndex> {
        let index = self.0.join("index.theme");

        if !index.exists() {
            return Err(ThemeError::ThemeIndexNotFound(index));
        }

        let index = Ini::load_from_file(index)?;

        Ok(ThemeIndex(index))
    }
}
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

// Iter through the base paths and get all theme directories
fn list_icon_themes() -> io::Result<Vec<ThemePath>> {
    let mut icon_theme_path = vec![];
    for theme_base_dir in icon_theme_base_paths().iter() {
        for entry in theme_base_dir.read_dir()? {
            let entry = entry?;
            let has_index = entry.path().join("index.theme").exists();
            if entry.path().is_dir() && has_index {
                icon_theme_path.push(ThemePath(entry.path()));
            }
        }
    }
    Ok(icon_theme_path)
}

#[cfg(test)]
mod test {
    use crate::theme::{icon_theme_base_paths, list_icon_themes};
    use anyhow::Result;
    use speculoos::prelude::*;

    #[test]
    fn should_get_theme_paths_ordered() {
        let base_paths = icon_theme_base_paths();

        assert_that!(base_paths).is_not_empty()
    }

    #[test]
    fn should_get_icon_theme_paths_ordered() -> Result<()> {
        let themes = list_icon_themes()?;

        assert_that!(themes).is_not_empty();
        Ok(())
    }

    #[test]
    fn should_read_theme_index() -> Result<()> {
        let paths = list_icon_themes()?;

        for theme_path in paths {
            assert_that!(theme_path.index()).is_ok();
        }

        Ok(())
    }

    #[test]
    fn should_get_theme_name() -> Result<()> {
        let paths = list_icon_themes()?;

        for theme_path in paths {
            assert_that!(theme_path.name().len()).is_greater_than(0);
        }

        Ok(())
    }
}

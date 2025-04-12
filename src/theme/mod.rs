use crate::theme::error::ThemeError;
use crate::theme::paths::ThemePath;
use once_cell::sync::Lazy;
pub(crate) use paths::BASE_PATHS;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

mod directories;
pub mod error;
mod parse;
mod paths;

type Result<T> = std::result::Result<T, ThemeError>;

pub static THEMES: Lazy<BTreeMap<String, Vec<Theme>>> = Lazy::new(get_all_themes);

pub fn read_ini_theme(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

#[derive(Debug)]
pub struct Theme {
    pub path: ThemePath,
    pub index: PathBuf,
}

impl Theme {
    pub fn try_get_icon(
        &self,
        name: &str,
        size: u16,
        scale: u16,
        force_svg: bool,
    ) -> Option<PathBuf> {
        let file = read_ini_theme(&self.index);
        self.try_get_icon_exact_size(file.as_str(), name, size, scale, force_svg)
            .or_else(|| self.try_get_icon_closest_size(file.as_str(), name, size, scale, force_svg))
    }

    fn try_get_icon_exact_size(
        &self,
        file: &str,
        name: &str,
        size: u16,
        scale: u16,
        force_svg: bool,
    ) -> Option<PathBuf> {
        self.match_size(file, size, scale)
            .find_map(|path| try_build_icon_path(name, path, force_svg))
    }

    fn match_size<'a>(
        &'a self,
        file: &'a str,
        size: u16,
        scale: u16,
    ) -> impl Iterator<Item = PathBuf> + 'a {
        let dirs = self.get_all_directories(file);

        dirs.filter(move |directory| directory.match_size(size, scale))
            .map(|dir| dir.name)
            .map(|dir| self.path().join(dir))
    }

    fn try_get_icon_closest_size(
        &self,
        file: &str,
        name: &str,
        size: u16,
        scale: u16,
        force_svg: bool,
    ) -> Option<PathBuf> {
        self.closest_match_size(file, size, scale)
            .iter()
            .find_map(|path| try_build_icon_path(name, path, force_svg))
    }

    fn closest_match_size(&self, file: &str, size: u16, scale: u16) -> Vec<PathBuf> {
        let dirs = self.get_all_directories(file);

        let mut dirs: Vec<_> = dirs
            .filter_map(|directory| {
                let distance = directory.directory_size_distance(size, scale);
                if distance < i16::MAX {
                    Some((directory, distance.abs()))
                } else {
                    None
                }
            })
            .collect();

        dirs.sort_by(|(_, a), (_, b)| a.cmp(b));

        dirs.iter()
            .map(|(dir, _)| dir)
            .map(|dir| dir.name)
            .map(|dir| self.path().join(dir))
            .collect()
    }

    fn path(&self) -> &PathBuf {
        &self.path.0
    }
}

pub(super) fn try_build_icon_path<P: AsRef<Path>>(
    name: &str,
    path: P,
    force_svg: bool,
) -> Option<PathBuf> {
    if force_svg {
        try_build_svg(name, path.as_ref())
            .or_else(|| try_build_png(name, path.as_ref()))
            .or_else(|| try_build_xmp(name, path.as_ref()))
    } else {
        try_build_png(name, path.as_ref())
            .or_else(|| try_build_svg(name, path.as_ref()))
            .or_else(|| try_build_xmp(name, path.as_ref()))
    }
}

fn try_build_svg<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let svg = path.join(format!("{name}.svg"));

    if svg.exists() {
        Some(svg)
    } else {
        None
    }
}

fn try_build_png<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let png = path.join(format!("{name}.png"));

    if png.exists() {
        Some(png)
    } else {
        None
    }
}

fn try_build_xmp<P: AsRef<Path>>(name: &str, path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    let xmp = path.join(format!("{name}.xmp"));
    if xmp.exists() {
        Some(xmp)
    } else {
        None
    }
}

// Iter through the base paths and get all theme directories
pub(super) fn get_all_themes() -> BTreeMap<String, Vec<Theme>> {
    let mut icon_themes = BTreeMap::<_, Vec<_>>::new();
    let mut found_indices = BTreeMap::new();
    let mut to_revisit = Vec::new();

    for theme_base_dir in BASE_PATHS.iter() {
        let dir_iter = match theme_base_dir.read_dir() {
            Ok(dir) => dir,
            Err(why) => {
                tracing::error!(?why, dir = ?theme_base_dir, "unable to read icon theme directory");
                continue;
            }
        };

        for entry in dir_iter.filter_map(std::io::Result::ok) {
            let name = entry.file_name();
            let fallback_index = found_indices.get(&name);
            if let Some(theme) = Theme::from_path(entry.path(), fallback_index) {
                if fallback_index.is_none() {
                    found_indices.insert(name.clone(), theme.index.clone());
                }
                let name = name.to_string_lossy().to_string();
                icon_themes.entry(name).or_default().push(theme);
            } else if entry.path().is_dir() {
                to_revisit.push(entry);
            }
        }
    }

    for entry in to_revisit {
        let name = entry.file_name();
        let fallback_index = found_indices.get(&name);
        if let Some(theme) = Theme::from_path(entry.path(), fallback_index) {
            let name = name.to_string_lossy().to_string();
            icon_themes.entry(name).or_default().push(theme);
        }
    }

    icon_themes
}

impl Theme {
    pub(crate) fn from_path<P: AsRef<Path>>(path: P, index: Option<&PathBuf>) -> Option<Self> {
        let path = path.as_ref();

        let has_index = path.join("index.theme").exists() || index.is_some();

        if !has_index || !path.is_dir() {
            return None;
        }

        let path = ThemePath(path.into());

        match (index, path.index()) {
            (Some(index), _) => Some(Theme {
                path,
                index: index.clone(),
            }),
            (None, Ok(index)) => Some(Theme { path, index }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::THEMES;
    use speculoos::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn get_one_icon() {
        let themes = THEMES.get("Adwaita").unwrap();
        println!(
            "{:?}",
            themes.iter().find_map(|t| {
                let file = crate::theme::read_ini_theme(&t.index);
                t.try_get_icon_exact_size(file.as_str(), "edit-delete-symbolic", 24, 1, false)
            })
        );
    }

    #[test]
    fn should_get_png_first() {
        let themes = THEMES.get("hicolor").unwrap();
        let icon = themes.iter().find_map(|t| {
            let file = crate::theme::read_ini_theme(&t.index);
            t.try_get_icon_exact_size(file.as_str(), "blueman", 24, 1, true)
        });
        assert_that!(icon).is_some().is_equal_to(PathBuf::from(
            "/usr/share/icons/hicolor/scalable/apps/blueman.svg",
        ));
    }

    #[test]
    fn should_get_svg_first() {
        let themes = THEMES.get("hicolor").unwrap();
        let icon = themes.iter().find_map(|t| {
            let file = crate::theme::read_ini_theme(&t.index);
            t.try_get_icon_exact_size(file.as_str(), "blueman", 24, 1, false)
        });
        assert_that!(icon).is_some().is_equal_to(PathBuf::from(
            "/usr/share/icons/hicolor/22x22/apps/blueman.png",
        ));
    }
}

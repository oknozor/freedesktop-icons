use crate::theme::IconTheme;
use std::path::PathBuf;

pub fn lookup(icon: &str) -> Lookup {
    Lookup {
        theme: "hicolor".to_string(),
        size: 24,
        scale: 1,
        icon: icon.to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct Lookup {
    theme: String,
    size: u16,
    scale: u16,
    icon: String,
}

impl Lookup {
    pub fn theme(&self, theme: &str) -> Self {
        let mut lookup = self.clone();
        lookup.theme = theme.to_string();
        lookup
    }

    pub fn size(&self, size: u16) ->  Self  {
        if size == 0 {
            panic!("Icon size cannot be zero")
        }
        let mut lookup = self.clone();
        lookup.size = size;
        lookup
    }

    pub fn scale(&self, scale: u16) -> Self {
        if scale == 0 {
            panic!("Icon scale cannot be zero")
        }
        let mut lookup = self.clone();
        lookup.scale = scale;
        lookup
    }

    pub fn execute(&self) -> Result<Option<PathBuf>, crate::Error> {
        let lookup_paths = lookup_paths();

        let theme_paths: Vec<PathBuf> = lookup_paths
            .iter()
            .map(|path| path.join(&self.theme))
            .filter(|path| path.exists())
            .collect();

        // Lookup for a dir matching the requested size
        for theme_path in &theme_paths {
            let theme = IconTheme::from_path(theme_path.join("index.theme"))?;
            // Try to match the exact requested size
            let matching_dirs: Vec<&String> = theme
                .entries
                .iter()
                .filter(|(path, meta)| meta.match_size(self.size, self.scale))
                .map(|(dir, _)| dir)
                .collect();

            for dir in matching_dirs {
                if let Some(dir) = self.find_icon_in_dir(theme_path.clone(), dir) {
                    return Ok(Some(dir));
                }
            }

            // Try to match the closest size instead
            let matching_dirs: Vec<&String> = theme
                .entries
                .iter()
                .filter(|(path, meta)| {
                    meta.match_size_distance(self.size as i16, self.scale as i16) < i16::MAX
                })
                .map(|(dir, _)| dir)
                .collect();

            for dir in matching_dirs {
                if let Some(dir) = self.find_icon_in_dir(theme_path.clone(), dir) {
                    return Ok(Some(dir));
                }
            }

            // Recursively lookup in parent themes
            if let Some(parent) = theme.data.inherits {
                let lookup = self.theme(&parent);
                if let Some(icon) = lookup.execute()? {
                    return Ok(Some(icon));
                }
            }
        }

        // Fallback to default hicolor theme
        self.theme("hicolor");
        if let Some(icon) = self.execute()? {
            return Ok(Some(icon));
        }

        Ok(None)
    }

    fn find_icon_in_dir(&self, theme_path: PathBuf, matching_dir: &str) -> Option<PathBuf> {
        let icon_path = theme_path.join(matching_dir);
        let icon_path_png = icon_path.join(format!("{}.png", self.icon));

        if icon_path_png.exists() {
            return Some(icon_path_png);
        }

        let icon_path_svg = icon_path.join(format!("{}.svg", self.icon));
        if icon_path_svg.exists() {
            return Some(icon_path_png);
        }

        let icon_path_xmp = icon_path.join(format!("{}.xmp", self.icon));
        if icon_path_svg.exists() {
            return Some(icon_path_xmp);
        }

        None
    }
}

fn lookup_paths() -> Vec<PathBuf> {
    let mut paths = Vec::with_capacity(3);

    let home_icons = dirs::home_dir().map(|home| home.join(".icons"));
    if let Some(home_icons) = home_icons {
        paths.push(home_icons);
    }

    let xdg_data_user_dir_icons = dirs::home_dir().map(|home| home.join("icons"));
    if let Some(xdg_data_user_dir_icons) = xdg_data_user_dir_icons {
        paths.push(xdg_data_user_dir_icons);
    }

    let xdg_data_dir_icons = PathBuf::from("/usr/share/icons");
    if xdg_data_dir_icons.exists() {
        paths.push(xdg_data_dir_icons);
    }

    paths.push(PathBuf::from("/usr/share/pixmaps"));

    paths
}

#[cfg(test)]
mod test {
    use speculoos::prelude::*;

    use crate::lookup::lookup;

    #[test]
    fn default_lookup() {
        let icon = lookup("firefox").execute().unwrap();

        let filename = icon
            .as_ref()
            .map(|icon| icon.file_name().unwrap().to_str())
            .flatten();

        assert_that!(filename).is_some().is_equal_to("firefox.png");
    }

    #[test]
    fn scaled_lookup() {
        let icon = lookup("firefox").scale(2).execute().unwrap();

        let filename = icon
            .as_ref()
            .map(|icon| icon.file_name().unwrap().to_str())
            .flatten();

        assert_that!(filename).is_some().is_equal_to("firefox.png");
    }

    #[test]
    fn big_lookup() {
        let icon = lookup("firefox")
            .theme("hicolor")
            .size(128)
            .scale(15)
            .execute()
            .unwrap();

        let filename = icon
            .as_ref()
            .map(|icon| icon.file_name().unwrap().to_str())
            .flatten();

        assert_that!(filename).is_some().is_equal_to("firefox.png");
    }
}

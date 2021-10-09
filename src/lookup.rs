use std::path::PathBuf;
use crate::theme::Theme;


pub fn lookup(icon: &str) -> Lookup<'_> {
    Lookup {
        theme: None,
        size: 24,
        scale: 24,
        icon,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lookup<'a> {
    theme: Option<&'a str>,
    size: u16,
    scale: u16,
    icon: &'a str,
}

impl<'a> Lookup<'a> {
    pub fn theme(&mut self, theme: &'a str) -> Self {
        self.theme = Some(theme);
        *self
    }

    pub fn size(&mut self, size: u16) -> Self {
        self.size = size;
        *self
    }

    pub fn scale(&mut self, scale: u16) -> Self {
        self.scale = scale;
        *self
    }

    pub fn execute(&self) -> Option<PathBuf> {
        let lookup_paths = lookup_paths();
        let theme_paths: Vec<PathBuf> = lookup_paths.iter().map(|path| path.join(self.theme.unwrap()))
            .filter(|path| path.exists())
            .collect();

        println!("{:?}", theme_paths);

        // Lookup for a dir matching the requested size
        for theme_path in &theme_paths {
            let theme = Theme::from_file(theme_path.join("index.theme")).unwrap();
            let matching_dirs: Vec<&String> = theme.directories.iter()
                .filter(|(path, meta)| meta.match_size(self.size, self.scale))
                .map(|(dir, _)| dir)
                .collect();

            for dir in matching_dirs {
                println!("{}", dir);
                if let Some(dir) = self.find_icon_in_dir(theme_path.clone(), dir) {
                    return Some(dir)
                }
            }

            let matching_dirs: Vec<&String> = theme.directories.iter()
                .filter(|(path, meta)| meta.match_size_distance(self.size, self.scale) < u16::MAX)
                .map(|(dir, _)| dir)
                .collect();

            for dir in matching_dirs {
                println!("{}", dir);
                if let Some(dir) = self.find_icon_in_dir(theme_path.clone(), dir) {
                    return Some(dir)
                }
            }
        };

        None
    }

    fn find_icon_in_dir(&self, theme_path: PathBuf, matching_dir: &String) -> Option<PathBuf> {
        let icon_path = theme_path.join(matching_dir);
        let icon_path_png = icon_path.join(format!("{}.png", self.icon));

        if icon_path_png.exists() {
            return Some(icon_path_png)
        }

        let icon_path_svg = icon_path.join(format!("{}.svg", self.icon));
        if icon_path_svg.exists() {
            return Some(icon_path_png)
        }

        let icon_path_xmp = icon_path.join(format!("{}.xmp", self.icon));
        if icon_path_svg.exists() {
            return Some(icon_path_xmp)
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
    use crate::lookup::lookup;
    use speculoos::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn simple_lookup() {
        let icon = lookup("firefox")
            .theme("hicolor")
            .size(24)
            .scale(0)
            .execute();

        assert_that!(icon)
            .is_some()
            .is_equal_to(PathBuf::from("/usr/share/icons/hicolor/24x24/apps/firefox.png"));
    }

    #[test]
    fn big_lookup() {
        let icon = lookup("firefox")
            .theme("hicolor")
            .size(128)
            .scale(15)
            .execute();

        assert_that!(icon)
            .is_some()
            .is_equal_to(PathBuf::from("toto"));
    }
}


use crate::theme::{try_build_icon_path, THEMES};
use std::path::PathBuf;

pub mod theme;

pub fn lookup(name: &str) -> LookupBuilder {
    LookupBuilder::new(name)
}

pub struct LookupBuilder<'a> {
    name: &'a str,
    scale: u16,
    size: u16,
    theme: Option<&'a str>,
}

impl<'a> LookupBuilder<'a> {
    pub fn with_size(mut self, scale: u16) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_scale(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    pub fn with_theme<'b: 'a>(mut self, theme: &'b str) -> Self {
        self.theme = Some(theme);
        self
    }

    fn new<'b: 'a>(name: &'b str) -> Self {
        Self {
            name,
            scale: 1,
            size: 24,
            theme: None,
        }
    }

    pub fn find_one(self) -> Option<PathBuf> {
        let name = self.name;
        let size = self.size;
        let scale = self.scale;

        // We have a theme name, lookup -> fallback - exit
        if let Some(theme) = self.theme.and_then(|theme| THEMES.get(theme)) {
            let icon = theme.try_get_icon(name, size, scale);
            if icon.is_some() {
                return icon;
            }

            // hicolor fallback
            if let Some(fallback) = THEMES.get("hicolor") {
                let icon = fallback.try_get_icon(name, size, scale);
                if icon.is_some() {
                    return icon;
                }
            }
        } else {
            // No theme let's look everywhere
            for theme in THEMES.values() {
                let icon = theme.try_get_icon(name, size, scale);
                if icon.is_some() {
                    return icon;
                }
            }
        }

        // Last chance
        try_build_icon_path(name, "/usr/share/pixmaps")
    }
}

#[cfg(test)]
mod test {
    use crate::lookup;
    use speculoos::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn simple_lookup() {
        let firefox = lookup("firefox").find_one();
        assert_that!(firefox).is_some();
    }

    #[test]
    fn compare_to_linincon_with_theme() {
        let lin_wireshark = linicon::lookup_icon("wireshark")
            .next()
            .unwrap()
            .unwrap()
            .path;

        let wireshark = lookup("wireshark")
            .with_size(16)
            .with_scale(1)
            .with_theme("Papirus")
            .find_one();

        assert_that!(wireshark).is_some().is_equal_to(lin_wireshark)
    }

    #[test]
    fn compare_to_linicon_in_pixmap() {
        let archlinux_logo = lookup("archlinux-logo")
            .with_size(16)
            .with_scale(1)
            .with_theme("Papirus")
            .find_one();

        assert_that!(archlinux_logo)
            .is_some()
            .is_equal_to(PathBuf::from("/usr/share/pixmaps/archlinux-logo.png"));
    }
}

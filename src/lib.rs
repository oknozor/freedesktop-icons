use crate::theme::{try_build_icon_path, FALL_BACK_THEMES, THEMES};
use std::path::PathBuf;

pub mod theme;

pub fn lookup(name: &str, size: u16, scale: u16, theme: &str) -> Option<PathBuf> {
    if let Some(theme) = THEMES.get(theme) {
        let icon = theme.try_get_icon(name, size, scale);
        if icon.is_some() {
            return icon;
        }
    }

    for theme in FALL_BACK_THEMES.iter() {
        let icon = theme.try_get_icon(name, size, scale);
        if icon.is_some() {
            return icon;
        }
    }

    try_build_icon_path(name, "/usr/share/pixmaps")
}

#[cfg(test)]
mod test {
    use crate::lookup;
    use speculoos::prelude::*;

    #[test]
    fn compare_to_linincon() {
        let lin_wireshark = linicon::lookup_icon("wireshark")
            .next()
            .unwrap()
            .unwrap()
            .path;

        let wireshark = lookup("wireshark", 16, 1, "Papirus");

        assert_that!(wireshark).is_some().is_equal_to(lin_wireshark)
    }

    #[test]
    fn compare_to_linicon_in_pixmap() {
        let archlinux_logo = linicon::lookup_icon("archlinux-logo").next();

        assert_that!(archlinux_logo).is_some();

        let archlinux_logo = lookup("archlinux-logo", 16, 1, "Papirus");

        assert_that!(archlinux_logo)
            .is_some()
            .has_file_name("/usr/share/pixmaps/archlinux-logo.png");
    }
}

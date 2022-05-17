use freedesktop_icons::lookup;
use gtk4::{IconLookupFlags, IconTheme, TextDirection};
use speculoos::prelude::*;
use std::path::PathBuf;

#[test]
fn gtk_lookup() {
    gtk4::init().unwrap();
    let theme = IconTheme::new();

    let x = theme.lookup_icon(
        "firefox",
        &[],
        24,
        1,
        TextDirection::None,
        IconLookupFlags::empty(),
    );

    assert!(x.icon_name().is_some())
}

// Linicon sometimes fails with theme that have unknown parents
// This test only ensure we are running the correct function in the benchmarks
// And results are identical.
#[test]
fn linicon() {
    // Current theme
    let lin_user_home = linicon::lookup_icon("user-home")
        .from_theme("Adwaita")
        .with_size(24)
        .with_scale(1)
        .next();

    let user_home = lookup("user-home")
        .with_theme("Adwaita")
        .with_size(24)
        .with_scale(1)
        .find();

    asserting!("Linicon return some icon")
        .that(&lin_user_home.unwrap())
        .is_ok()
        .map(|icon| &icon.path)
        .is_equal_to(PathBuf::from(
            "/usr/share/icons/Adwaita/24x24/places/user-home.png",
        ));

    asserting!("Our implementation should return the same result as linicon")
        .that(&user_home)
        .is_some()
        .is_equal_to(PathBuf::from(
            "/usr/share/icons/Adwaita/24x24/places/user-home.png",
        ));

    // Fallback to hicolor
    let lin_firefox = linicon::lookup_icon("firefox")
        .from_theme("Adwaita")
        .with_size(24)
        .with_scale(1)
        .next();

    let firefox = lookup("firefox")
        .with_theme("Adwaita")
        .with_size(24)
        .with_scale(1)
        .find();

    asserting!("Linicon return some icon")
        .that(&lin_firefox.unwrap())
        .is_ok()
        .map(|icon| &icon.path)
        .is_equal_to(PathBuf::from(
            "/usr/share/icons/hicolor/22x22/apps/firefox.png",
        ));

    asserting!("Our implementation should return the same result as linicon")
        .that(&firefox)
        .is_some()
        .is_equal_to(PathBuf::from(
            "/usr/share/icons/hicolor/22x22/apps/firefox.png",
        ));

    // pixmaps
    let lin_archlinux = linicon::lookup_icon("archlinux-logo")
        .from_theme("Adwaita")
        .with_size(24)
        .with_scale(1)
        .next();

    let archlinux = lookup("archlinux-logo")
        .with_theme("Adwaita")
        .with_size(24)
        .with_scale(1)
        .find();

    asserting!("Linicon fails to fallback to pixmaps")
        .that(&lin_archlinux)
        .is_none();

    asserting!("But we succeed")
        .that(&archlinux)
        .is_some()
        .is_equal_to(PathBuf::from("/usr/share/pixmaps/archlinux-logo.png"));
}

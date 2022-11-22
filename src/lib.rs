//! # freedesktop-icons
//!
//! This crate provides a [freedesktop icon](https://specifications.freedesktop.org/icon-theme-spec/icon-theme-spec-latest.html#implementation_notes) lookup implementation.
//!
//! It exposes a single lookup function to find icons based on their `name`, `theme`, `size` and `scale`.
//!
//! ## Example
//!
//! **Simple lookup:**
//!
//! The following snippet get an icon from the default 'hicolor' theme
//! with the default scale (`1`) and the default size (`24`).
//!
//! ```rust
//! # fn main() {
//! use freedesktop_icons::lookup;
//!
//! let icon = lookup("firefox").find();
//! # }
//!```
//!
//! **Complex lookup:**
//!
//! If you have specific requirements for your lookup you can use the provided builder functions:
//!
//! ```rust
//! # fn main() {
//! use freedesktop_icons::lookup;
//!
//! let icon = lookup("firefox")
//!     .with_size(48)
//!     .with_scale(2)
//!     .with_theme("Arc")
//!     .find();
//! # }
//!```
//! **Cache:**
//!
//! If your application is going to repeat the same icon lookups multiple times
//! you can use the internal cache to improve performance.
//!
//! ```rust
//! # fn main() {
//! use freedesktop_icons::lookup;
//!
//! let icon = lookup("firefox")
//!     .with_size(48)
//!     .with_scale(2)
//!     .with_theme("Arc")
//!     .with_cache()
//!     .find();
//! # }
//! ```
use theme::{Theme, BASE_PATHS};

use crate::cache::{CacheEntry, CACHE};
use crate::theme::{try_build_icon_path, THEMES};
use std::path::PathBuf;

mod cache;
mod theme;

/// Return the list of installed themes on the system
///
/// ## Example
/// ```rust
/// # fn main() {
/// use freedesktop_icons::list_themes;
///
/// let themes: Vec<&str> = list_themes();
///
/// assert_eq!(themes, vec![
///     "Adwaita", "Arc", "Breeze Light", "HighContrast", "Papirus", "Papirus-Dark",
///     "Papirus-Light", "Breeze", "Breeze Dark", "Breeze", "ePapirus", "ePapirus-Dark", "Hicolor"
/// ])
/// # }
pub fn list_themes() -> Vec<&'static str> {
    THEMES
        .values()
        .map(|path| &path.index)
        .filter_map(|index| {
            index
                .section(Some("Icon Theme"))
                .and_then(|section| section.get("Name"))
        })
        .collect()
}

/// The lookup builder struct, holding all the lookup query parameters.
pub struct LookupBuilder<'a> {
    name: &'a str,
    cache: bool,
    force_svg: bool,
    scale: u16,
    size: u16,
    theme: &'a str,
}

/// Build an icon lookup for the given icon name.
///
/// ## Example
/// ```rust
/// # fn main() {
/// use freedesktop_icons::lookup;
///
/// let icon = lookup("firefox").find();
/// # }
pub fn lookup(name: &str) -> LookupBuilder {
    LookupBuilder::new(name)
}

impl<'a> LookupBuilder<'a> {
    /// Restrict the lookup to the given icon size.
    ///
    /// ## Example
    /// ```rust
    /// # fn main() {
    /// use freedesktop_icons::lookup;
    ///
    /// let icon = lookup("firefox")
    ///     .with_size(48)
    ///     .find();
    /// # }
    pub fn with_size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Restrict the lookup to the given scale.
    ///
    /// ## Example
    /// ```rust
    /// # fn main() {
    /// use freedesktop_icons::lookup;
    ///
    /// let icon = lookup("firefox")
    ///     .with_scale(2)
    ///     .find();
    /// # }
    pub fn with_scale(mut self, scale: u16) -> Self {
        self.scale = scale;
        self
    }

    /// Add the given theme to the current lookup :
    /// ## Example
    /// ```rust
    /// # fn main() {
    /// use freedesktop_icons::lookup;
    ///
    /// let icon = lookup("firefox")
    ///     .with_theme("Papirus")
    ///     .find();
    /// # }
    pub fn with_theme<'b: 'a>(mut self, theme: &'b str) -> Self {
        self.theme = theme;
        self
    }

    /// Store the result of the lookup in cache, subsequent
    /// lookup will first try to get the cached icon.
    /// This can drastically increase lookup performances for application
    /// that repeat the same lookups, an application launcher for instance.
    ///
    /// ## Example
    /// ```rust
    /// # fn main() {
    /// use freedesktop_icons::lookup;
    ///
    /// let icon = lookup("firefox")
    ///     .with_scale(2)
    ///     .with_cache()
    ///     .find();
    /// # }
    pub fn with_cache(mut self) -> Self {
        self.cache = true;
        self
    }

    /// By default [`find`] will prioritize Png over Svg icon.
    /// Use this if you need to prioritize Svg icons. This could be useful
    /// if you need a modifiable icon, to match a user theme for instance.
    ///
    /// ## Example
    /// ```rust
    /// # fn main() {
    /// use freedesktop_icons::lookup;
    ///
    /// let icon = lookup("firefox")
    ///     .force_svg()
    ///     .find();
    /// # }
    pub fn force_svg(mut self) -> Self {
        self.force_svg = true;
        self
    }

    /// Execute the current lookup
    /// if no icon is found in the current theme fallback to
    /// `/usr/share/icons/hicolor` theme and then to `/usr/share/pixmaps`.
    pub fn find(self) -> Option<PathBuf> {
        // Lookup for an icon in the given theme and fallback to 'hicolor' default theme
        self.lookup_in_theme()
    }

    fn new<'b: 'a>(name: &'b str) -> Self {
        Self {
            name,
            cache: false,
            force_svg: false,
            scale: 1,
            size: 24,
            theme: "hicolor",
        }
    }

    // Recursively lookup for icon in the given theme and its parents
    fn lookup_in_theme(&self) -> Option<PathBuf> {
        // If cache is activated, attempt to get the icon there first
        // If the icon was previously search but not found, we return
        // `None` early, otherwise, attempt to perform a lookup
        if self.cache {
            match self.cache_lookup(self.theme) {
                CacheEntry::Found(icon) => {
                    return Some(icon);
                }
                CacheEntry::NotFound => {
                    return None;
                }
                CacheEntry::Unknown => {}
            };
        }

        // Then lookup in the given theme
        THEMES.get(self.theme).and_then(|icon_theme| {
            let icon = icon_theme
                .try_get_icon(self.name, self.size, self.scale, self.force_svg)
                .or_else(|| {
                    // Fallback to the parent themes recursively
                    icon_theme.inherits().into_iter().find_map(|parent| {
                        THEMES.get(parent).and_then(|parent| {
                            parent.try_get_icon(self.name, self.size, self.scale, self.force_svg)
                        })
                    })
                })
                .or_else(|| {
                    for theme_base_dir in BASE_PATHS.iter() {
                        let theme = Theme::from_path(theme_base_dir.join("hicolor"));
                        if let Some(icon) = theme.and_then(|theme| {
                            theme.try_get_icon(self.name, self.size, self.scale, self.force_svg)
                        }) {
                            return Some(icon);
                        }
                    }
                    None
                })
                .or_else(|| try_build_icon_path(self.name, "/usr/share/pixmaps", self.force_svg))
                .or_else(|| {
                    let p = PathBuf::from(&self.name);
                    if let (Some(name), Some(parent)) = (p.file_name(), p.parent()) {
                        try_build_icon_path(&name.to_string_lossy(), parent, self.force_svg)
                    } else {
                        None
                    }
                });

            if self.cache {
                self.store(self.theme, icon)
            } else {
                icon
            }
        })
    }

    #[inline]
    fn cache_lookup(&self, theme: &str) -> CacheEntry {
        CACHE.get(theme, self.size, self.scale, self.name)
    }

    #[inline]
    fn store(&self, theme: &str, icon: Option<PathBuf>) -> Option<PathBuf> {
        CACHE.insert(theme, self.size, self.scale, self.name, &icon);
        icon
    }
}

// WARNING: these test are highly dependent on your installed icon-themes.
// If you want to run them, make sure you have 'Papirus' and 'Arc' icon-themes installed.
#[cfg(test)]
mod test {
    use crate::{lookup, CacheEntry, CACHE};
    use speculoos::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn simple_lookup() {
        let firefox = lookup("firefox").find();

        asserting!("Lookup with no parameters should return an existing icon")
            .that(&firefox)
            .is_some()
            .is_equal_to(PathBuf::from(
                "/usr/share/icons/hicolor/22x22/apps/firefox.png",
            ));
    }

    #[test]
    fn theme_lookup() {
        let firefox = lookup("firefox").with_theme("Papirus").find();

        asserting!("Lookup with no parameters should return an existing icon")
            .that(&firefox)
            .is_some()
            .is_equal_to(PathBuf::from(
                "/usr/share/icons/Papirus/24x24/apps/firefox.svg",
            ));
    }

    #[test]
    fn should_fallback_to_parent_theme() {
        let icon = lookup("video-single-display-symbolic")
            .with_theme("Arc")
            .find();

        asserting!("Lookup for an icon in the Arc theme should find the icon in its parent")
            .that(&icon)
            .is_some()
            .is_equal_to(PathBuf::from(
                "/usr/share/icons/Adwaita/scalable/devices/video-single-display-symbolic.svg",
            ));
    }

    #[test]
    fn should_fallback_to_pixmaps_utlimately() {
        let archlinux_logo = lookup("archlinux-logo")
            .with_size(16)
            .with_scale(1)
            .with_theme("Papirus")
            .find();

        asserting!("When lookup fail in theme, icon should be found in '/usr/share/pixmaps'")
            .that(&archlinux_logo)
            .is_some()
            .is_equal_to(PathBuf::from("/usr/share/pixmaps/archlinux-logo.png"));
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
            .find();

        asserting!("Given the same input parameter, lookup should ouput be the same as linincon")
            .that(&wireshark)
            .is_some()
            .is_equal_to(lin_wireshark);
    }

    #[test]
    fn should_not_attempt_to_lookup_a_not_found_cached_icon() {
        let not_found = lookup("not-found").with_cache().find();

        assert_that!(not_found).is_none();

        let expected_cache_result = CACHE.get("hicolor", 24, 1, "not-found");

        asserting!("When lookup fails a first time, subsequent attempts should fail from cache")
            .that(&expected_cache_result)
            .is_equal_to(CacheEntry::NotFound);
    }
}

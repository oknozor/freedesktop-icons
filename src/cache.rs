use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub(crate) static CACHE: Lazy<Cache> = Lazy::new(Cache::default);
type IconMap = BTreeMap<(String, u16, u16), PathBuf>;
type ThemeMap = BTreeMap<String, IconMap>;

#[derive(Default)]
pub(crate) struct Cache(Mutex<ThemeMap>);

impl Cache {
    pub fn insert(&self, theme: &str, size: u16, scale: u16, icon_name: &str, icon_path: &Path) {
        let mut theme_map = self.0.lock().unwrap();

        match theme_map.get_mut(theme) {
            Some(icon_map) => {
                icon_map.insert(
                    (icon_name.to_string(), size, scale),
                    icon_path.to_path_buf(),
                );
            }
            None => {
                let mut icon_map = BTreeMap::new();
                icon_map.insert(
                    (icon_name.to_string(), size, scale),
                    icon_path.to_path_buf(),
                );
                theme_map.insert(theme.to_string(), icon_map);
            }
        }
    }

    pub fn get(&self, theme: &str, size: u16, scale: u16, icon_name: &str) -> Option<PathBuf> {
        let theme_map = self.0.lock().unwrap();

        theme_map
            .get(theme)
            .map(|icon_map| icon_map.get(&(icon_name.to_string(), size, scale)))
            .and_then(|path| path.cloned())
    }
}

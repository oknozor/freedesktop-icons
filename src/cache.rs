use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

pub(crate) static CACHE: Lazy<Cache> = Lazy::new(Cache::default);

#[derive(Default)]
pub(crate) struct Cache(Mutex<BTreeMap<String, BTreeMap<(String, u16, u16), PathBuf>>>);

impl Cache {
    pub fn insert(&self, theme: &str, size: u16, scale: u16, icon_name: &str, icon_path: &PathBuf) {
        let mut theme_map = self.0.lock().unwrap();

        match theme_map.get_mut(theme) {
            Some(icon_map) => {
                icon_map.insert((icon_name.to_string(), size, scale), icon_path.clone());
            }
            None => {
                let mut icon_map = BTreeMap::new();
                icon_map.insert((icon_name.to_string(), size, scale), icon_path.clone());
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

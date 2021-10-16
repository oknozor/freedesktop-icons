use core::convert::AsRef;
use core::option::Option;
use core::result::Result;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IconTheme {
    #[serde(rename = "Icon Theme")]
    pub data: ThemeDefinition,
    #[serde(flatten)]
    pub entries: HashMap<String, IconDirectory>,
}

impl IconTheme {
    pub fn from_path<S: AsRef<Path>>(path: S) -> Result<Self, crate::Error> {
        let string = std::fs::read_to_string(path).map_err(crate::Error::from)?;
        serde_ini::from_str(&string).map_err(crate::Error::from)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DirEntry {
    pub size: String,
    pub r#type: DirectoryType,
}

/// https://specifications.freedesktop.org/icon-theme-spec/latest/ar01s04.html
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThemeDefinition {
    pub name: String,
    pub comment: String,
    pub inherits: Option<String>,
    pub directories: String,
    pub scaled_directories: Option<String>,
    pub hidden: Option<String>,
    pub example: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct IconDirectory {
    #[serde(deserialize_with = "IconDirectory::deserialize_u16")]
    size: u16,
    #[serde(deserialize_with = "IconDirectory::deserialize_u16")]
    scale: u16,
    context: Option<String>,
    r#type: DirectoryType,
    #[serde(deserialize_with = "IconDirectory::deserialize_u16")]
    max_size: u16,
    #[serde(deserialize_with = "IconDirectory::deserialize_u16")]
    min_size: u16,
    #[serde(deserialize_with = "IconDirectory::deserialize_u16")]
    threshold: u16,
}

impl IconDirectory {
    fn size(&self) -> i16 {
        self.size as i16
    }

    fn scale(&self) -> i16 {
        self.scale as i16
    }

    fn max_size(&self) -> i16 {
        self.max_size as i16
    }

    fn min_size(&self) -> i16 {
        self.min_size as i16
    }

    fn threshold(&self) -> i16 {
        self.threshold as i16
    }
}

impl IconDirectory {
    pub fn deserialize_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        str::parse::<u16>(&s).map_err(serde::de::Error::custom)
    }
}
impl Default for IconDirectory {
    fn default() -> Self {
        IconDirectory {
            size: 0,
            scale: 1,
            context: None,
            r#type: DirectoryType::Threshold,
            max_size: 0,
            min_size: 0,
            threshold: 2,
        }
    }
}

impl IconDirectory {
    pub fn match_size(&self, size: u16, scale: u16) -> bool {
        if self.scale != scale {
            return false;
        }

        match self.r#type {
            DirectoryType::Fixed => self.size == size,
            DirectoryType::Scalable => self.min_size <= size && size <= self.max_size,
            DirectoryType::Threshold => {
                size - self.threshold <= self.size && self.size <= size + self.threshold
            }
        }
    }

    pub fn match_size_distance(&self, size: i16, scale: i16) -> i16 {
        match self.r#type {
            DirectoryType::Fixed => ((self.size() * self.scale()) - (size * scale)).abs(),
            DirectoryType::Scalable => {
                if size * scale < self.min_size() * self.scale() {
                    self.min_size() * self.scale() - size * scale
                } else if size * scale > self.max_size() * self.scale() {
                    size * scale - self.max_size() * self.scale()
                } else {
                    0
                }
            }
            DirectoryType::Threshold => {
                if size * scale < (self.size() - self.threshold()) * self.scale() {
                    self.min_size() * self.scale() - size * scale
                } else if size * size > (self.size() + self.threshold()) * self.scale() {
                    size * size - self.max_size() * self.scale()
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum DirectoryType {
    Fixed,
    Scalable,
    Threshold,
}

#[cfg(test)]
mod test {
    use speculoos::assert_that;

    use crate::theme::IconTheme;

    #[test]
    fn should_parse_theme() {
        let hicolor = IconTheme::from_path("/usr/share/icons/hicolor/index.theme").unwrap();

        assert_that!(hicolor)
            .map(|theme| &theme.data.name)
            .is_equal_to("Hicolor".to_string());
    }
}

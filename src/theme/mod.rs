use core::convert::{AsRef, From};
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::Ok;
use std::collections::HashMap;
use std::path::Path;

use ini::{Ini, Properties};

use crate::error;

/// A freedesktop theme file
#[derive(Debug, Default)]
pub struct Theme {
    pub icon_theme: ThemeDefinition,
    pub directories: HashMap<String, IconDirectory>,
    pub icons_data: HashMap<String, IconData>,
}

impl Theme {
    pub fn from_file<S: AsRef<Path>>(path: S) -> Result<Self, error::Error> {
        let ini = Ini::load_from_file(path).unwrap();
        let mut theme = Theme::default();

        for (key, value) in ini.iter() {
            match key {
                Some("Icon Theme") => theme.icon_theme = ThemeDefinition::from(value),
                Some(key) if key.starts_with("X-") => {
                    theme.icons_data.insert(key.to_string(), IconData::from(value));
                }
                Some(key) => {
                    theme.directories.insert(key.to_string(), IconDirectory::from(value));
                }
                None => ()
            }
        }

        Ok(theme)
    }
}

/// https://specifications.freedesktop.org/icon-theme-spec/latest/ar01s04.html
#[derive(Debug, Default)]
pub struct ThemeDefinition {
    pub name: String,
    pub comment: String,
    pub inherits: Option<String>,
    pub directories: String,
    pub scaled_directories: Option<String>,
    pub hidden: Option<String>,
    pub example: Option<String>,
}

impl From<&Properties> for ThemeDefinition {
    fn from(props: &Properties) -> Self {
        let mut theme = ThemeDefinition::default();

        props.iter().for_each(|(key, value)| {
            match key {
                "Name" => theme.name = value.to_string(),
                "Comment" => theme.comment = value.to_string(),
                "Inherits" => theme.inherits = Some(value.to_string()),
                "Directories" => theme.directories = value.to_string(),
                "ScaledDirectories" => theme.scaled_directories = Some(value.to_string()),
                "Hidden" => theme.hidden = Some(value.to_string()),
                "Example" => theme.example = Some(value.to_string()),
                _ => unreachable!()
            }
        });

        theme
    }
}


#[derive(Debug, Default)]
pub struct IconDirectory {
    size: u16,
    scale: u16,
    context: Option<String>,
    r#type: Option<DirectoryType>,
    max_size: Option<u16>,
    min_size: Option<u16>,
    threshold: Option<u16>,
}

impl IconDirectory {
    pub fn match_size(&self, size: u16, scale: u16) -> bool {
        if self.scale != scale {
            return false;
        }

        match self.r#type {
            Some(DirectoryType::Fixed) => {
                self.size == size
            }
            Some(DirectoryType::Scalable) => {
                self.min_size.unwrap() <= size && size <= self.max_size.unwrap()
            }
            Some(DirectoryType::Threshold) => {
                let threshold = match self.threshold {
                    None => { 0 }
                    Some(threshold) => threshold
                };

                size - threshold <= self.size && self.size <= size + threshold
            }
            None => {
                panic!("Expected icon dir to have a type")
            }
        }
    }

    pub fn match_size_distance(&self, size: u16, scale: u16) -> u16 {
        match self.r#type {
            Some(DirectoryType::Fixed) => {
                ((self.size * self.scale) as i16 - (size * scale) as i16).abs() as u16
            }
            Some(DirectoryType::Scalable) => {
                let min_size = self.min_size.unwrap_or(0);
                let max_size = self.max_size.unwrap_or(u16::MAX);
                if self.size * self.scale < min_size * self.scale {
                    min_size * self.scale - size * scale
                } else if size * scale > max_size * self.scale {
                    size * scale - max_size * self.scale
                } else {
                    0
                }
            }
            Some(DirectoryType::Threshold) => {
                let threshhold = self.threshold.unwrap_or(0);
                if size * scale < (self.size - threshhold) * self.scale {
                    let min_size = self.min_size.unwrap_or(0);
                    min_size * self.scale - size * scale
                } else if size * size > (self.size + threshhold) * self.scale {
                    let max_size = self.max_size.unwrap_or(u16::MAX);
                    size * size - max_size * self.scale
                } else {
                    0
                }
            }
            None => {
                panic!("Expected icon dir to have a type")
            }
        }
    }
}

impl From<&Properties> for IconDirectory {
    fn from(props: &Properties) -> Self {
        let mut directory = IconDirectory::default();

        props.iter().for_each(|(key, value)| {
            match key {
                "Size" => directory.size = str::parse(value).unwrap(),
                "Scale" => directory.scale = str::parse(value).unwrap(),
                "Context" => directory.context = Some(value.to_string()),
                "Type" => directory.r#type = Some(DirectoryType::from(value)),
                "MaxSize" => directory.max_size = Some(str::parse(value).unwrap()),
                "MinSize" => directory.min_size = Some(str::parse(value).unwrap()),
                "Threshold" => directory.threshold = Some(str::parse(value).unwrap()),
                _ => unreachable!()
            }
        });

        directory
    }
}

#[derive(Debug, Default)]
pub struct IconData {
    pub display_name: Option<String>,
    pub embedded_text_rectangle: Option<String>,
    pub attach_points: Option<String>,
}

impl From<&Properties> for IconData {
    fn from(props: &Properties) -> Self {
        let mut icon_data = IconData::default();

        props.iter().for_each(|(key, value)| {
            match key {
                "DisplayName" => icon_data.display_name = Some(value.to_string()),
                "EmbeddedTextRectangle" => icon_data.embedded_text_rectangle = Some(value.to_string()),
                "AttachPoints" => icon_data.attach_points = Some(value.to_string()),
                _ => unreachable!()
            }
        });

        icon_data
    }
}


#[derive(Debug)]
pub enum DirectoryType {
    Fixed,
    Scalable,
    Threshold,
}

impl From<&str> for DirectoryType {
    fn from(value: &str) -> Self {
        match value {
            "Fixed" => DirectoryType::Fixed,
            "Scalable" => DirectoryType::Scalable,
            "Threshold" => DirectoryType::Threshold,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use speculoos::assert_that;

    use crate::theme::Theme;

    #[test]
    fn should_parse_theme() {
        let hicolor = Theme::from_file("/usr/share/icons/hicolor/index.theme").unwrap();

        assert_that!(hicolor)
            .map(|theme| &theme.icon_theme.name)
            .is_equal_to("Hicolor".to_string());
    }
}

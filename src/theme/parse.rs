use crate::theme::directories::{Directory, DirectoryType};
use crate::theme::Theme;
use ini::Properties;

impl Theme {
    pub(super) fn get_all_directories(&self) -> Vec<Directory> {
        self.directories()
            .iter()
            .filter_map(|name| self.get_directory(name))
            .collect()
    }

    // TODO: use me
    fn scaled_directories(&self) -> Vec<&str> {
        self.get_icon_theme_section()
            .and_then(|props| props.get("ScaledDirectories"))
            .map(|dirs| dirs.split(',').collect())
            .unwrap_or(vec![])
    }

    fn get_icon_theme_section(&self) -> Option<&Properties> {
        self.index.section(Some("Icon Theme"))
    }

    pub fn inherits(&self) -> Option<&str> {
        self.get_icon_theme_section()
            .and_then(|props| props.get("Inherits"))
    }

    fn directories(&self) -> Vec<&str> {
        self.index
            .section(Some("Icon Theme"))
            .and_then(|props| props.get("Directories"))
            .map(|dirs| dirs.split(',').collect())
            .unwrap_or(vec![])
    }

    fn get_directory<'a>(&'a self, name: &'a str) -> Option<Directory> {
        self.index.section(Some(name)).map(|props| {
            let size = props
                .get("Size")
                .and_then(|size| str::parse(size).ok())
                .expect("Size not found for icon");
            Directory {
                name,
                size,
                scale: props
                    .get("Scale")
                    .and_then(|scale| str::parse(scale).ok())
                    .unwrap_or(1),
                context: props.get("Context"),
                type_: props
                    .get("Type")
                    .map(DirectoryType::from)
                    .unwrap_or_default(),
                maxsize: props
                    .get("MaxSize")
                    .and_then(|max| str::parse(max).ok())
                    .unwrap_or(size),
                minsize: props
                    .get("MinSize")
                    .and_then(|min| str::parse(min).ok())
                    .unwrap_or(size),
                threshold: props
                    .get("Threshold")
                    .and_then(|thrsh| str::parse(thrsh).ok())
                    .unwrap_or(2),
            }
        })
    }
}

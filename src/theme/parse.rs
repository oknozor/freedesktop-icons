use crate::theme::directories::{Directory, DirectoryType};
use crate::theme::Theme;

fn icon_theme_section(file: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    ini_core::Parser::new(file)
        .skip_while(|item| *item != ini_core::Item::Section("Icon Theme"))
        .take_while(|item| match item {
            ini_core::Item::Section(value) => *value == "Icon Theme",
            _ => true,
        })
        .filter_map(|item| {
            if let ini_core::Item::Property(key, value) = item {
                Some((key, value?))
            } else {
                None
            }
        })
}

#[derive(Debug)]
enum DirectorySection<'a> {
    Property(&'a str, &'a str),
    EndSection,
    Section(&'a str),
}

fn sections(file: &str) -> impl Iterator<Item = DirectorySection> {
    ini_core::Parser::new(file).filter_map(move |item| match item {
        ini_core::Item::Property(key, Some(value)) => Some(DirectorySection::Property(key, value)),
        ini_core::Item::Section(section) => Some(DirectorySection::Section(section)),
        ini_core::Item::SectionEnd => Some(DirectorySection::EndSection),
        _ => None,
    })
}

impl Theme {
    pub(super) fn get_all_directories<'a>(
        &'a self,
        file: &'a str,
    ) -> impl Iterator<Item = Directory<'a>> + 'a {
        let mut iterator = sections(file);

        std::iter::from_fn(move || {
            let mut name = "";
            let mut size = None;
            let mut max_size = None;
            let mut min_size = None;
            let mut threshold = None;
            let mut scale = None;
            // let mut context = None;
            let mut dtype = DirectoryType::default();

            #[allow(clippy::while_let_on_iterator)]
            while let Some(event) = iterator.next() {
                match event {
                    DirectorySection::Property(key, value) => {
                        if name.is_empty() || name == "Icon Theme" {
                            continue;
                        }

                        match key {
                            "Size" => size = str::parse(value).ok(),
                            "Scale" => scale = str::parse(value).ok(),
                            // "Context" => context = Some(value),
                            "Type" => dtype = DirectoryType::from(value),
                            "MaxSize" => max_size = str::parse(value).ok(),
                            "MinSize" => min_size = str::parse(value).ok(),
                            "Threshold" => threshold = str::parse(value).ok(),
                            _ => (),
                        }
                    }

                    DirectorySection::Section(new_name) => {
                        name = new_name;
                        size = None;
                        max_size = None;
                        min_size = None;
                        threshold = None;
                        scale = None;
                        dtype = DirectoryType::default();
                    }

                    DirectorySection::EndSection => {
                        if name.is_empty() || name == "Icon Theme" {
                            continue;
                        }

                        let size = size.take()?;

                        return Some(Directory {
                            name,
                            size,
                            scale: scale.unwrap_or(1),
                            // context,
                            type_: dtype,
                            maxsize: max_size.unwrap_or(size),
                            minsize: min_size.unwrap_or(size),
                            threshold: threshold.unwrap_or(2),
                        });
                    }
                }
            }

            None
        })
    }

    pub fn inherits<'a>(&self, file: &'a str) -> Vec<&'a str> {
        icon_theme_section(file)
            .find(|&(key, _)| key == "Inherits")
            .map(|(_, parents)| {
                parents
                    .split(',')
                    // Filtering out 'hicolor' since we are going to fallback there anyway
                    .filter(|parent| parent != &"hicolor")
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use crate::THEMES;
    use speculoos::prelude::*;

    #[test]
    fn should_get_theme_parents() {
        for theme in THEMES.get("Arc").unwrap() {
            let file = crate::theme::read_ini_theme(&theme.index);
            let parents = theme.inherits(&file);

            assert_that!(parents).does_not_contain("hicolor");

            assert_that!(parents).is_equal_to(vec![
                "Moka",
                "Faba",
                "elementary",
                "Adwaita",
                "gnome",
            ]);
        }
    }
}

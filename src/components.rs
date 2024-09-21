use std::path::{Path, PathBuf};

use iced::{Element, Length};

pub fn app_icon<'a, T>(icon_path: &Path) -> iced::Element<'a, T> {
    if icon_path.extension().is_some_and(|x| x == "svg") {
        Element::from(
            iced::widget::svg(icon_path)
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill),
        )
    } else {
        Element::from(
            iced::widget::image(icon_path)
                .content_fit(iced::ContentFit::Contain)
                .width(Length::Fill)
                .height(Length::Fill),
        )
    }
}

pub fn default_icon_path() -> PathBuf {
    freedesktop_icons::lookup("wayland")
        .with_theme("breeze")
        .with_cache()
        .find()
        .unwrap()
}

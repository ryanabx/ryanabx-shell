use std::{collections::HashMap, path::PathBuf};

use desktop_entry::DesktopEntryCache;
use freedesktop_desktop_entry::DesktopEntry;
use iced::{
    widget::{button, column, row, Container, Rule},
    Background, Border, Color, Length, Radius, Theme,
};

use crate::{
    compositor::{WaylandOutgoing, WindowHandle, WindowInfo},
    Message,
};

pub mod desktop_entry;

#[derive(Clone, Debug)]
pub struct AppTray<'a> {
    pub de_cache: DesktopEntryCache<'a>,
    pub active_toplevels: HashMap<String, ApplicationGroup>,
}

impl<'a> Default for AppTray<'a> {
    fn default() -> Self {
        Self {
            de_cache: DesktopEntryCache::new(),
            active_toplevels: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ApplicationGroup {
    pub toplevels: HashMap<WindowHandle, WindowInfo>,
}

impl<'a> AppTray<'a> {
    pub fn get_desktop_entry(&mut self, app_id: &str) -> Option<DesktopEntry<'a>> {
        self.de_cache.0.get(app_id).cloned()
    }
}

pub fn get_tray_widget<'a>(
    app_id: &str,
    desktop_entry: Option<&DesktopEntry<'a>>,
    app_info: ApplicationGroup,
) -> iced::widget::Button<'a, crate::Message> {
    // println!("app_id: {}", app_id);
    // if app_id == "Google-chrome" || app_id == "com.google.chrome" {
    //     println!("ODIUAOIDUWIO");
    // }
    let icon_path = desktop_entry
        .and_then(|entry| entry.icon())
        .and_then(|icon| freedesktop_icons::lookup(icon).with_cache().find())
        .or_else(|| get_default_icon());
    match icon_path {
        Some(path) => {
            if path.extension().is_some_and(|x| x == "svg") {
                iced::widget::button(column![
                    iced::widget::svg(path)
                        .content_fit(iced::ContentFit::Contain)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    get_horizontal_rule(&app_info)
                ])
            } else {
                iced::widget::button(column![
                    iced::widget::image(path)
                        .content_fit(iced::ContentFit::Contain)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    get_horizontal_rule(&app_info)
                ])
            }
        }
        None => iced::widget::button(iced::widget::Space::new(Length::Fill, Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill),
    }
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(8)
    .style(move |theme, status| tray_button_style(theme, status, &app_info))
    .on_press_maybe(desktop_entry.and_then(|entry| entry.exec()).map(|exec| {
        Message::WaylandOut(WaylandOutgoing::Exec(app_id.to_string(), exec.to_string()))
    }))
    // .on_press_maybe(if toplevels.is_empty() {
    //     launch_on_preferred_gpu(desktop_info, gpus)
    // } else if toplevels.len() == 1 {
    //     toplevels.first().map(|t| Message::Toggle(t.0.clone()))
    // } else {
    //     Some(Message::TopLevelListPopup((*id).into(), window_id))
    // })
}

fn get_horizontal_rule<'a>(app_info: &ApplicationGroup) -> Container<'a, Message> {
    if app_info.toplevels.is_empty() {
        iced::widget::container(iced::widget::Space::new(
            Length::Fixed(8.0),
            Length::Fixed(2.0),
        ))
    } else {
        iced::widget::container(
            iced::widget::horizontal_rule(1)
                .style(|_| iced::widget::rule::Style {
                    color: Color::WHITE,
                    width: 2,
                    radius: 4.into(),
                    fill_mode: iced::widget::rule::FillMode::Full,
                })
                .width(Length::Fixed(8.0)),
        )
    }
    .center_x(Length::Fill)
}

fn get_default_icon() -> Option<PathBuf> {
    freedesktop_icons::lookup("wayland").with_cache().find()
}

fn tray_button_style(
    _theme: &Theme,
    status: button::Status,
    app_info: &ApplicationGroup,
) -> button::Style {
    if app_info.toplevels.is_empty() {
        button::Style {
            background: Some(Background::Color(Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                    0.1
                } else {
                    0.0
                },
            })),
            border: Border {
                radius: Radius::from(8.0),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                        0.3
                    } else {
                        0.0
                    },
                },
                width: 1.0,
            },
            ..Default::default()
        }
    } else {
        button::Style {
            background: Some(Background::Color(Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                    0.2
                } else {
                    0.1
                },
            })),
            border: Border {
                radius: Radius::from(8.0),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: if matches!(status, button::Status::Hovered | button::Status::Pressed) {
                        0.4
                    } else {
                        0.2
                    },
                },
                width: 1.0,
            },
            ..Default::default()
        }
    }
}

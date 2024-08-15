use app_tray::{AppTray, ApplicationGroup};
use cctk::wayland_client::protocol::wl_seat::WlSeat;
use compositor::{CompositorBackend, WaylandIncoming, WaylandOutgoing, WindowHandle};
use config::PanelConfig;
use iced::{
    application::{
        actions::layer_surface::SctkLayerSurfaceSettings, layer_surface::Anchor, InitialSurface,
    },
    event::{self, listen_with},
    widget::{column, container::Style},
    Application, Background, Color, Command, Element, Radius, Settings, Subscription, Theme,
};

mod app_tray;
mod compositor;
mod config;

fn main() -> Result<(), iced::Error> {
    let settings = SctkLayerSurfaceSettings {
        anchor: Anchor::BOTTOM.union(Anchor::LEFT).union(Anchor::RIGHT),
        size: Some((None, Some(48))),
        layer: iced::application::layer_surface::Layer::Top,
        exclusive_zone: 48,
        ..Default::default()
    };
    Panel::run(Settings {
        initial_surface: InitialSurface::LayerSurface(settings),
        ..Settings::default()
    })
}

#[derive(Clone, Debug)]
struct Panel<'a> {
    panel_config: PanelConfig,
    app_tray: AppTray<'a>,
    backend: CompositorBackend,
}

impl<'a> Default for Panel<'a> {
    fn default() -> Self {
        Self {
            panel_config: PanelConfig::default(),
            app_tray: AppTray::default(),
            backend: CompositorBackend::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Panic,
    WaylandIn(WaylandIncoming),
    WaylandOut(WaylandOutgoing),
    NewSeat(WlSeat),
    RemovedSeat(WlSeat),
}

#[derive(Clone, Debug)]
pub enum AppTrayRequest {
    Window(WindowOperationMessage),
}

#[derive(Clone, Debug)]
pub enum WindowOperationMessage {
    Activate(WindowHandle),
    Minimize(WindowHandle),
    Launch(String),
}

impl<'a> Application for Panel<'a> {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Renderer = iced::Renderer;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Panel::default(), Command::<self::Message>::none())
    }

    fn title(&self, _id: iced::window::Id) -> String {
        "Window".into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Panic => {
                panic!("Panic button pressed hehe");
            }
            Message::WaylandIn(evt) => self
                .backend
                .handle_message(&mut self.app_tray, evt)
                .unwrap_or(Command::none()),
            Message::WaylandOut(evt) => self
                .backend
                .handle_outgoing_message(&mut self.app_tray, evt)
                .unwrap_or(Command::none()),
            Message::NewSeat(_) => {
                println!("New seat!");
                Command::none()
            }
            Message::RemovedSeat(_) => {
                println!("Removed seat!");
                Command::none()
            }
        }
    }

    fn view(
        &self,
        _id: iced::window::Id,
    ) -> iced::Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        // Get app tray apps
        let app_tray_apps = self
            .panel_config
            .favorites
            .iter()
            .map(|x| {
                let app_id = x.clone();
                (
                    app_id,
                    self.app_tray
                        .active_toplevels
                        .get(x)
                        .cloned()
                        .unwrap_or(ApplicationGroup::default()),
                )
            })
            .chain(
                self.app_tray
                    .active_toplevels
                    .iter()
                    .filter_map(|(app_id, info)| {
                        if self.panel_config.favorites.contains(app_id) {
                            None
                        } else {
                            Some((app_id.clone(), info.clone()))
                        }
                    }),
            )
            .map(|(app_id, group)| {
                let entry = self.app_tray.de_cache.0.get(&app_id);
                // println!("{}, {:?}", &app_id, entry.map(|x| x.appid.clone()));
                app_tray::get_tray_widget(&app_id, entry, group)
            })
            .map(|x| Element::from(iced::widget::container(x).width(48).height(48).padding(2)));

        let panel_items = iced::widget::row(app_tray_apps);
        iced::widget::container(column![
            iced::widget::horizontal_rule(1).style(|_| iced::widget::rule::Style {
                color: Color::WHITE,
                width: 1,
                radius: Radius::from(0),
                fill_mode: iced::widget::rule::FillMode::Full
            }),
            panel_items
        ])
        .style(|theme| self.panel_style(theme))
        .fill()
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::batch(vec![
            self.backend.wayland_subscription().map(Message::WaylandIn),
            listen_with(|e, _, _| match e {
                iced::Event::PlatformSpecific(event::PlatformSpecific::Wayland(
                    event::wayland::Event::Seat(e, seat),
                )) => match e {
                    event::wayland::SeatEvent::Enter => Some(Message::NewSeat(seat)),
                    event::wayland::SeatEvent::Leave => Some(Message::RemovedSeat(seat)),
                },
                _ => None,
            }),
        ])
    }
}

impl<'a> Panel<'a> {
    fn panel_style(&self, _theme: &Theme) -> Style {
        Style {
            background: Some(Background::Color(Color {
                r: 30.0 / 256.0,
                g: 30.0 / 256.0,
                b: 30.0 / 256.0,
                a: 1.0,
            })),
            ..Default::default()
        }
    }
}

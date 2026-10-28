// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::fl;
use crate::tema::{Cor, Tema};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::Length;
use cosmic::iced::{Limits, Subscription, window::Id};
use cosmic::prelude::*;
use cosmic::widget;
use cosmic::widget::{column, row};
use meowsystem_drives::{DeviceType, get_all_devices, run_command};

#[derive(Default)]
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,
    tema: Tema,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),
    Unmount(String),
    Open(String),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.github.meowsystem.Drives";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = AppModel {
            core,
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => config,
                })
                .unwrap_or_default(),
            tema: Tema::load(Self::APP_ID),
            ..Default::default()
        };

        let tema = app.aplicar_tema(cosmic::theme::active().cosmic().is_dark, false);
        (app, tema)
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        crate::tema::botao(&self.core.applet, "media-eject-symbolic", self.tinta())
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        // Build applet view
        let devices = get_all_devices().unwrap_or_default();
        let mut content_list = widget::column(vec![]).padding(8).spacing(0);
        if devices.is_empty() {
            content_list = content_list.push(row!(
                column!(
                    widget::button::text(fl!("no-devices-mounted"))
                        .on_press(Message::Open(String::new()))
                )
                .width(Length::Fill)
            ));
        } else {
            for device in devices {
                content_list = content_list.push(row!(
                    column!(
                        widget::icon(
                            widget::icon::from_name(match device.device_type() {
                                DeviceType::USB => "drive-harddisk-usb-symbolic",
                                DeviceType::Disk => "disks-symbolic",
                                DeviceType::Network => "network-workgroup-symbolic",
                            })
                            .symbolic(true)
                            .into()
                        )
                        .class(cosmic::theme::Svg::Custom(std::rc::Rc::new(
                            |theme: &cosmic::Theme| cosmic::iced::widget::svg::Style {
                                color: Some(theme.cosmic().accent_color().into()),
                            }
                        )))
                    )
                    .padding([7, 5]),
                    column!(
                        widget::button::text(device.label())
                            .on_press(Message::Open(device.mountpoint()))
                            .width(Length::Fill)
                            .padding(5),
                    )
                    .width(Length::Fill),
                    column!(
                        widget::button::icon(widget::icon::from_name("media-eject-symbolic"))
                            .on_press(Message::Unmount(device.mountpoint()))
                    )
                ));
            }
        }

        self.core.applet.popup_container(content_list).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
        ])
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::Unmount(mountpoint) => {
                // TODO: Unmounting is at its most basic right now.
                // This could be expanded to be more robust
                run_command("umount", &mountpoint);
            }
            Message::Open(mountpoint) => {
                // TODO: Launch the default file browser
                run_command("cosmic-files", &mountpoint);
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    cosmic::surface::surface_task(cosmic::surface::action::destroy_popup(p))
                } else {
                    let tema = self.reler_tema();
                    let abrir = cosmic::surface::surface_task(cosmic::surface::action::app_popup(
                        |_| Default::default(),
                        |app: &mut AppModel| {
                            let new_id = Id::unique();
                            app.popup.replace(new_id);
                            let mut popup_settings = app.core.applet.get_popup_settings(
                                app.core.main_window_id().unwrap(),
                                new_id,
                                None,
                                None,
                                None,
                            );
                            popup_settings.positioner.size_limits = Limits::NONE
                                .max_width(372.0)
                                .min_width(300.0)
                                .min_height(200.0)
                                .max_height(1080.0);
                            popup_settings
                        },
                        None,
                    ));
                    Task::batch([tema, abrir])
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }
        Task::none()
    }

    fn system_theme_update(
        &mut self,
        _keys: &[&'static str],
        new_theme: &cosmic::cosmic_theme::Theme,
    ) -> Task<cosmic::Action<Self::Message>> {
        self.aplicar_tema(new_theme.is_dark, false)
    }

    fn system_theme_mode_update(
        &mut self,
        _keys: &[&'static str],
        mode: &cosmic::cosmic_theme::ThemeMode,
    ) -> Task<cosmic::Action<Self::Message>> {
        self.aplicar_tema(mode.is_dark, false)
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl AppModel {
    fn aplicar_tema(&self, dark: bool, antes: bool) -> Task<cosmic::Action<Message>> {
        match self.tema.cosmic(dark) {
            Some(theme) => cosmic::command::set_theme(theme),
            None if antes => cosmic::command::set_theme(self.core.system_theme().clone()),
            None => Task::none(),
        }
    }

    fn reler_tema(&mut self) -> Task<cosmic::Action<Message>> {
        let novo = Tema::load(<Self as cosmic::Application>::APP_ID);
        if novo == self.tema {
            return Task::none();
        }
        let antes = self.tema.cosmic(true).is_some();
        self.tema = novo;
        self.aplicar_tema(cosmic::theme::active().cosmic().is_dark, antes)
    }

    fn tinta(&self) -> Option<Cor> {
        self.tema.cor("tinta")
    }
}

// SPDX-License-Identifier: MIT

use std::time::Duration;

use cosmic::{
    app,
    iced::{widget::Row, Alignment, Length, Subscription},
    widget::icon,
    Element, Task,
};

use crate::lock_state;

const CAPS_LOCK_SVG: &[u8] = include_bytes!("../data/icons/caps-lock-symbolic.svg");
const NUM_LOCK_SVG: &[u8] = include_bytes!("../data/icons/num-lock-symbolic.svg");

/// Opacity used for the icon of a lock key that is currently inactive.
const INACTIVE_OPACITY: f32 = 0.35;
/// How often the sysfs LED state is polled.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<LockKeysApplet>(())
}

#[derive(Default)]
struct LockKeysApplet {
    core: cosmic::app::Core,
    caps_lock: bool,
    num_lock: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl cosmic::Application for LockKeysApplet {
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    const APP_ID: &'static str = "com.lucianosb.CosmicAppletLockKeys";

    fn init(core: cosmic::app::Core, _flags: Self::Flags) -> (Self, app::Task<Message>) {
        let app = Self {
            core,
            caps_lock: lock_state::caps_lock(),
            num_lock: lock_state::num_lock(),
        };
        (app, Task::none())
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }

    fn update(&mut self, message: Message) -> app::Task<Message> {
        match message {
            Message::Tick => {
                self.caps_lock = lock_state::caps_lock();
                self.num_lock = lock_state::num_lock();
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(POLL_INTERVAL).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        let (icon_size, _) = self.core.applet.suggested_size(true);
        let spacing = self.core.applet.spacing;

        let caps_icon: Element<_> = icon::from_svg_bytes(CAPS_LOCK_SVG)
            .symbolic(true)
            .icon()
            .size(icon_size)
            .opacity(if self.caps_lock {
                1.0
            } else {
                INACTIVE_OPACITY
            })
            .into();

        let num_icon: Element<_> = icon::from_svg_bytes(NUM_LOCK_SVG)
            .symbolic(true)
            .icon()
            .size(icon_size)
            .opacity(if self.num_lock { 1.0 } else { INACTIVE_OPACITY })
            .into();

        let content = Row::with_children(vec![caps_icon, num_icon])
            .align_y(Alignment::Center)
            .height(Length::Shrink)
            .width(Length::Shrink)
            .spacing(spacing as f32);

        self.core.applet.autosize_window(content).into()
    }
}

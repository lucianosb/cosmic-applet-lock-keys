// SPDX-License-Identifier: GPL-3.0-only

mod app;
mod lock_state;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt::init();
    tracing::info!("starting cosmic-applet-lock-keys");
    app::run()
}

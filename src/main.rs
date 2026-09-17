// SPDX-License-Identifier: MIT

mod app;
mod lock_state;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt::init();
    tracing::info!("starting cosmic-applet-lock-keys");
    app::run()
}

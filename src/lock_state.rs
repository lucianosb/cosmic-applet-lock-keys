// SPDX-License-Identifier: GPL-3.0-only

//! Reads Num Lock / Caps Lock indicator state from the kernel LED sysfs
//! interface (`/sys/class/leds/*::{capslock,numlock}/brightness`). This
//! reflects the keyboard LED state maintained by the input subsystem
//! regardless of whether a physical LED is present, so it works under
//! Wayland/COSMIC without needing access to a specific keyboard device.

const LEDS_DIR: &str = "/sys/class/leds";

fn any_led_on(suffix: &str) -> bool {
    let Ok(entries) = std::fs::read_dir(LEDS_DIR) else {
        return false;
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if !name.ends_with(suffix) {
            continue;
        }

        let brightness_path = entry.path().join("brightness");
        if let Ok(contents) = std::fs::read_to_string(&brightness_path) {
            if contents.trim().parse::<u32>().unwrap_or(0) > 0 {
                return true;
            }
        }
    }

    false
}

pub fn caps_lock() -> bool {
    any_led_on("::capslock")
}

pub fn num_lock() -> bool {
    any_led_on("::numlock")
}

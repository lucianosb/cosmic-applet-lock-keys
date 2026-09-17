// SPDX-License-Identifier: MIT

//! Reads Num Lock / Caps Lock indicator state from the kernel LED sysfs
//! interface (`/sys/class/leds/*::{capslock,numlock}/brightness`). This
//! reflects the keyboard LED state maintained by the input subsystem
//! regardless of whether a physical LED is present, so it works under
//! Wayland/COSMIC without needing access to a specific keyboard device.

const LEDS_DIR: &str = "/sys/class/leds";

fn is_led_on(name: &str, suffix: &str, brightness: &str) -> bool {
    name.ends_with(suffix) && brightness.trim().parse::<u32>().unwrap_or(0) > 0
}

fn any_led_on(suffix: &str) -> bool {
    let Ok(entries) = std::fs::read_dir(LEDS_DIR) else {
        return false;
    };

    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        let brightness_path = entry.path().join("brightness");
        if let Ok(contents) = std::fs::read_to_string(&brightness_path) {
            if is_led_on(name, suffix, &contents) {
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

#[cfg(test)]
mod tests {
    use super::is_led_on;

    #[test]
    fn reports_active_matching_led() {
        assert!(is_led_on("input0::capslock", "::capslock", "1\n"));
        assert!(is_led_on("input0::numlock", "::numlock", "255"));
    }

    #[test]
    fn reports_inactive_zero_brightness() {
        assert!(!is_led_on("input0::capslock", "::capslock", "0\n"));
    }

    #[test]
    fn ignores_unrelated_led_names() {
        assert!(!is_led_on("input0::scrolllock", "::capslock", "1\n"));
    }

    #[test]
    fn treats_invalid_brightness_as_inactive() {
        assert!(!is_led_on("input0::capslock", "::capslock", "not-a-number"));
        assert!(!is_led_on("input0::capslock", "::capslock", ""));
    }
}

# COSMIC Applet: Lock Keys

A small [COSMIC](https://github.com/pop-os/cosmic-epoch) panel applet that
shows two icons in the top toolbar — one for **Caps Lock** and one for
**Num Lock**. Each icon is dimmed when its key is inactive and becomes fully
opaque when the key is toggled on.

State is read from the kernel LED sysfs interface
(`/sys/class/leds/*::capslock` / `*::numlock`), which reflects the keyboard
LED state maintained by the input subsystem, so it works under Wayland
without needing to query a specific keyboard device.

## Build

Requires a Rust toolchain (`rustup` recommended) and the system dependencies
needed by `libcosmic` (Wayland client libraries, etc., same as any other
COSMIC applet).

```sh
cargo build --release
```

## Install

```sh
just install
```

This installs the binary to `/usr/bin/cosmic-applet-lock-keys` and a
`.desktop` entry with `X-CosmicApplet=true` so it can be added to the panel
via COSMIC Settings > Panel > Applets.

## Uninstall

```sh
just uninstall
```

## AI Disclaimer

Most of the code was written with the assistance of Github Copilot.

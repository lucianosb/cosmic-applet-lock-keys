name := 'cosmic-applet-lock-keys'
export APPID := 'com.lucianosb.CosmicAppletLockKeys'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))
target-dir := env('CARGO_TARGET_DIR', absolute_path('.') / 'target')
bin-src := target-dir / 'release' / name
bin-dst := base-dir / 'bin' / name
desktop-src := 'data' / APPID + '.desktop'
desktop-dst := base-dir / 'share' / 'applications' / APPID + '.desktop'

default: build-release

build-release:
    cargo build --release

install: build-release
    install -Dm0755 {{bin-src}} {{bin-dst}}
    install -Dm0644 {{desktop-src}} {{desktop-dst}}

uninstall:
    rm -f {{bin-dst}} {{desktop-dst}}

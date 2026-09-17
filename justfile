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
metainfo-src := 'data' / APPID + '.metainfo.xml'
metainfo-dst := base-dir / 'share' / 'metainfo' / APPID + '.metainfo.xml'

default: build-release

build-release:
    cargo build --release

test:
    cargo test

install: build-release
    sudo install -Dm0755 {{bin-src}} {{bin-dst}}
    sudo install -Dm0644 {{desktop-src}} {{desktop-dst}}
    sudo install -Dm0644 {{metainfo-src}} {{metainfo-dst}}

uninstall:
    sudo rm -f {{bin-dst}} {{desktop-dst}} {{metainfo-dst}}

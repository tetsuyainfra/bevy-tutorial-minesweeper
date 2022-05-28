#!/bin/bash

set -x -e -u

# for bevy on Linux
apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
apt-get install libwayland-dev libxkbcommon-dev

# for egui on Linux
apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# for "cargo make" command
# cargo-build https://github.com/sagiegurari/cargo-make
cargo install --force cargo-make

# for WASM(WebBrowser)
# trunk https://trunkrs.dev/#install
cargo install --locked trunk

# for Windows x64
#rustup target add x86_64-pc-windows-msvc
#cargo install xwin
#xwin --accept-license 1 splat --output /opt/xwin


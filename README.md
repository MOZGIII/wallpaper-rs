# wallpaper-rs

[![Build status](https://ci.appveyor.com/api/projects/status/90mny46vxj4ko1w7?svg=true)](https://ci.appveyor.com/project/MOZGIII/wallpaper-rs)

The goal of this project is to provide tools for working with desktop images.

The design is to have multiple crates for low-level implementations and a single
high-level crate that provides a unified interface.
It is possible that low-level crates expose more features than required for the
unified interface.

## Crates

| Name | crates.io | docs.rs |
| -----|-----------|-------- |
| wallpaper-windows-user32 | https://crates.io/crates/wallpaper-windows-user32 | https://docs.rs/wallpaper-windows-user32
| wallpaper-windows-shobjidl | https://crates.io/crates/wallpaper-windows-shobjidl | https://docs.rs/wallpaper-windows-shobjidl

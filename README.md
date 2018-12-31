# wallpaper-rs

The goal of this project is to provide tools for working with desktop images.

The design is to have multiple crates for low-level implementations and a single
high-level crate that provides a unified interface.
It is possible that low-level crates expose more features than required for the
unified interface.

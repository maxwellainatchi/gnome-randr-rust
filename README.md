# gnome-randr-rust

A reimplementation of `xrandr` for Gnome on Wayland, especially for systems that don't support `wlr-output-management-unstable-v1`  (e.g. Manjaro). Written ground-up in rust for performance. This is also my first project in rust, so any suggestions are welcome!

## Installation

Installation requires `cargo`, part of the Rust toolchain. [Cargo/Rust installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html).

To install this tool, run `cargo install gnome-randr`. A library is also exposed for use in other Rust programs.

## Method

`gnome-randr-rust` uses the `dbus` object `org.gnome.Mutter.DisplayConfig`. See https://wiki.gnome.org/Initiatives/Wayland/Gaps/DisplayConfig for the original proposal, although the specification listed there is somewhat out of date (checked via `dbus introspect` on Gnome shell 40.5). A better (commented) XML file is listed in the `gnome-monitor-config` project [here](https://github.com/jadahl/gnome-monitor-config/blob/master/src/org.gnome.Mutter.DisplayConfig.xml).

The `GetCurrentState` method is used to list information about the displays, while `ApplyMonitorsConfig` is used to modify the current configuration.

## Inspiration

This project was heavily inspired by `xrandr` (obviously) and also [`gnome-randr`](https://gitlab.com/Oschowa/gnome-randr/). Sadly, `gnome-randr.py` appears to be broken as of my gnome version (40.5) when trying to modify display configurations. 

`gnome-randr.py` is also slower than my rust reimplementation: querying the python script takes about 30ms on my 3-monitor system, while the rust implementation takes about 3ms (`xrandr` takes about 1.5ms, but is also displaying different information due to limitations in `xrandr`'s bridge.)

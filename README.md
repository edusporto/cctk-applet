# cctk-applet

Menu bar indicator to set configurations specific to Dell computers.

## Requirements

Requires `cctk` (Dell Client Configuration Toolkit) installed.

### Arch Linux

Install `dell-command-configure` from the AUR. For example, using `yay`:

```
yay -S dell-command-configure
```

## Building

The Rust toolchain needs to be installed to build this project.
Installation instructions can be found on https://rustup.rs/.

Once the toolchain is installed, the program can be built with

```
cargo build --release
```

The resulting binary can be found at `target/release/cctk-applet`.

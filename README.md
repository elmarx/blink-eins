# blink-one: blink(1) rust library

Rust library for [blink(1)](https://blink1.thingm.com/). See [GitHub for libraries in other languages](https://github.com/todbot/blink1#blink1-usb-rgb-led).

This crate implements all blink(1) [HID commands](https://github.com/todbot/blink1/blob/main/docs/blink1-hid-commands.md).

# High level API

It exposes a high-level API for easy use.

## Usage

See [the examples](./examples).

Run `cargo run --example fade`.

# Low level commands

HID commands are available behind the `commands` feature flag through [WriteCmd](./src/cmd.rs) and [QueryCmd](./src/cmd.rs).

Commands may be sent to a `HidDevice` via [HidDeviceExt](./src/hid_device_ext.rs).

## Usage

For low-level usage, refer to the [upstream documentation](https://github.com/todbot/blink1/tree/main/docs) while enjoying the idiomatic Rust bindings.

# Implementation

The commands have been implemented directly from [blink1-hid-commands.md](https://github.com/todbot/blink1/blob/main/docs/blink1-hid-commands.md), I also peeked into [the C implementation](https://github.com/todbot/blink1-tool/blob/main/blink1-lib.h) for some details.
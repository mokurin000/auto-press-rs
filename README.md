# auto-press-rs

[Interception]: https://github.com/oblitum/Interception

Lua based flexible keyboard/mouse automation framework, implemented with [Interception].

- [Scan code](#scan-code)
- [Delay](#delay)
- [Build](#build)

## Scan code

### Documentation

See [MSDN](https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#scan-codes), scan codes were called `Scan 1 Make`.

### Scanner

- [kbdkeyinfo](https://dennisbabkin.com/kbdkeyinfo/)

## Delay

- Hold delay: duration between when a key is pressed and when it is released.
- Press delay: duration between when one key is released last time and when the next key is pressed.

## Build

By default, `device-info` is enabled. Then USB vid/pid database will be used, making the executable bloated, while provide possibly meaningful name of devices.

Disable the `device-info` feature if not planning to prompt for device selection.

Note: Don't hard-code the device number. It varies between each boot.

### Default

```bash
cargo build --release
```

### LuaJIT

```bash
cargo build --release --no-default-features -F luajit -F device-info
```

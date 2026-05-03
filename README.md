# auto-press-rs

[Interception]: https://github.com/oblitum/Interception

Lua based flexible keyboard/mouse automation framework, implemented with [Interception].

- [Why?](#why)
- [What it won't help with](#what-it-wont-help-with)
- [Scan code](#scan-code)
- [Delay](#delay)
- [Build](#build)

## Why?

[^0]: https://github.com/Chaoses-Ib/IbInputSimulator#supported-drivers

Interception is broadly compatible, requiring neither vendor-specific drivers (e.g., Logitech) nor legacy OEM control panels[^0].

### Pros

* Compared to the bare Win32 API (even with hooks), Interception is generally harder to detect.
* Provides excellent compatibility for UI/game automation, especially for applications not integrated with Win32 UI Automation.

### Cons

[^1]: https://www.youtube.com/watch?v=NGBhmuWpRnk

* Games protected by Easy Anti-Cheat will not start[^1]
* Driver-based anti-cheat solutions may still detect injected input events
* The Interception driver creates publicly accessible device interfaces such as `\\.\interceptionXX`
* The driver itself is not hidden

## What it won't help with

Interception was not designed for game cheating or automation.

It does not care whether your long-term input behavior gets flagged as "high-risk."

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

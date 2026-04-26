# auto-press-rs

[Interception]: https://github.com/oblitum/Interception

Lua 5.5 based flexible keyboard/mouse automation framework, implemented with [Interception].

## Scan code

### Documentation

See [MSDN](https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#scan-codes), scan codes were called `Scan 1 Make`.

### Scanner

- [kbdkeyinfo](https://dennisbabkin.com/kbdkeyinfo/)

## Delay

- Hold delay: duration between when a key is pressed and when it is released.
- Press delay: duration between when one key is released last time and when the next key is pressed.

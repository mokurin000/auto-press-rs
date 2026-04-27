---@diagnostic disable: lowercase-global

---@class Driver
--- Sleep for a random duration between min and max milliseconds (normally distributed)
---@field delay fun(self: Driver, min_millsec: integer, max_millsec: integer): nil
--- [Scan 1 Make]: https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#scan-codes
--- Click the keyboard key once. See [Scan 1 Make] for Scan codes.
---@field key_press fun(self: Driver, scan_code: integer): nil
--- Start holding the key by Scan 1 Make code
---@field key_down fun(self: Driver, scan_code: integer): nil
--- Release the key by Scan 1 Make code
---@field key_up fun(self: Driver, scan_code: integer): nil
--- Click the mouse button once
---
--- button:
---
--- - "left" -> Left button
---
--- - "right" -> Right button
---
--- - "middle" -> Middle button
---
--- - "backward" -> Button 4
---
--- - "forward" -> Button 5
---@field mouse_press fun(self: Driver, button: string): nil
--- List devices and number in engine side
---@field scan_devices fun(self: Driver): boolean
--- Keyboard device to use (1~10)
---@field keyboard integer
--- Mouse device to use (11~20)
---@field mouse integer
input_driver = _G.input_driver

--- Unix timestamp in milliseconds
---@type fun(): number
time_utc = _G.time_utc
--- Precise monotonic time for measuring duration passed, in  milliseconds
---@type fun(): number
time_mono = _G.time_mono

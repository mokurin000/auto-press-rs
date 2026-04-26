---@class Driver
---@field delay fun(self: Driver, min_millsec: integer, max_millsec: integer): nil
---@field key_press fun(self: Driver, scan_code: integer): nil
---@field key_down fun(self: Driver, scan_code: integer): nil
---@field key_up fun(self: Driver, scan_code: integer): nil
---@field mouse_press fun(self: Driver, button: string): nil
---@field scan_devices fun(self: Driver): boolean
---@field keyboard integer
---@field mouse integer
input_driver = input_driver

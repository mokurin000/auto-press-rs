-- show devices
local succ = input_driver:scan_devices()

if succ ~= 0 then
    print("Device scan failed!")
else
    -- prompt for device id here...
    input_driver.keyboard = 1
    input_driver.mouse = 12
end

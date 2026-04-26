-- show devices
local succ = input_driver:scan_devices()

if succ ~= true then
    print("Device scan failed!")
else
    -- prompt for device set here...
end

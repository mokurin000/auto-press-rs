local action_min = 8000
local action_max = 9000

local ops_min = 5000
local ops_max = 6000

local pet_place_min = 1500
local pet_place_max = 3000

input_driver:delay(action_min, action_max)

local adjust_time = {
    0x01, -- Esc
    0x21, -- F
    0x02, -- 1 消磨时间
    0x02, -- 1 到早上
    0x02, -- 2 休息并离开
}

while true do
    for _ = 1, 100 do
        input_driver:key_press(0x04) -- '3', 开心
        input_driver:delay(action_min, action_max)
    end

    for _, scan_code in adjust_time
    do
        input_driver:key_press(scan_code)
        input_driver:delay(ops_min, ops_max)
    end
    for i = 1, 6 do
        input_driver:key_press(i + 1) -- keyboard num
        input_driver:delay(pet_place_min, pet_place_max)
        input_driver:mouse_press("left")
        input_driver:delay(pet_place_min, pet_place_max)
    end

    input_driver:key_press(0x000F)
    input_driver:delay(pet_place_min, pet_place_max)
end

local action_times = 100 -- 15 * 60 / 9 = 100
local action_key = 0x4   -- 开心 3

local action_min = 8000
local action_max = 9000

local ops_min = 5000
local ops_max = 6000

local pet_place_min = 1500
local pet_place_max = 3000

input_driver:delay(action_min, action_max)

local adjust_time = {
    0x01, -- Esc, 首次启动时应该在星盘或动作界面
    0x21, -- F, 交互移动魔力之源
    0x02, -- 1 消磨时间
    0x02, -- 1 到早上
    0x02, -- 2 休息并离开
}

while true do
    for _, scan_code in ipairs(adjust_time)
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

    input_driver:key_press(0x000F) -- Tab
    input_driver:delay(pet_place_min, pet_place_max)

    for _ = 1, action_times do
        input_driver:key_press(action_key)
        input_driver:delay(action_min, action_max)
    end
end

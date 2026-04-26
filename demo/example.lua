-- "left" - Left button
-- "right" - Right button
-- "middle" - Middle button
-- "backward" - Button 4
-- "forward" - Button 5
input_driver:mouse_press("left")

-- delay for 1000~3000 milliseconds
-- the delay duration is normal distributed
input_driver:delay(1000, 3000)

-- send keyboard input by Scan 1 Make code
-- https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#scan-codes
input_driver:key_press(0x0023) -- press 'H'
input_driver:key_press(0x0012) -- press 'E'
input_driver:key_press(0x0026) -- press 'L'
input_driver:key_press(0x0026) -- press 'L'
input_driver:key_press(0x0018) -- press 'O'

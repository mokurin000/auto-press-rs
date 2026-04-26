input_driver:key_down(0x001D) -- Start holding LCtrl
input_driver:key_down(0x002A) -- Start holding LShift

input_driver:delay(50, 100)
input_driver:key_press(0x0001) -- Esc - Ctrl-Shit-Esc, open taskmgr

input_driver:key_up(0x001D)    -- Release LCtrl
input_driver:key_up(0x002A)    -- Release LShift

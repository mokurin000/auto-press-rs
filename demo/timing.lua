---@param name string
---@param clock_fn fun(): number | integer
---@param unit string
local function measure(name, clock_fn, unit)
    print("\n==", name, "==")

    local t0 = clock_fn()

    -- sleep for 456ms
    input_driver:delay(456, 456)

    local t1 = clock_fn()

    print("t0   =", t0)
    print("t1   =", t1)
    print("delta=", tostring(t1 - t0) .. unit)
end


--- system time (unix timestamp milliseconds)
measure("time_utc (system time)", time_utc, "ms")

--- monotonic clock (recommended)
measure("time_mono (monotonic)", time_mono, "ms")

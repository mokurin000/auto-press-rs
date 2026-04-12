use argh::FromArgs;
use std::ops::RangeInclusive;

/// keyboard delay config
#[derive(FromArgs, Debug)]
#[argh(description = "Demo program to simulate keyboard input")]
#[argh(help_triggers("-h", "--help"))]
pub struct Config {
    /// scan 1 make code
    #[argh(option)]
    pub scan_code: u16,

    /// press delay range start (milliseconds)
    #[argh(option)]
    pub press_delay_min: u32,

    /// press delay range end (milliseconds)
    #[argh(option)]
    pub press_delay_max: u32,

    /// hold delay range start (milliseconds)
    #[argh(option, default = "50")]
    pub hold_delay_min: u32,

    /// hold delay range end (milliseconds)
    #[argh(option, default = "120")]
    pub hold_delay_max: u32,
}

impl Config {
    pub fn press_delay(&self) -> RangeInclusive<u32> {
        self.press_delay_min..=self.press_delay_max
    }

    pub fn hold_delay(&self) -> RangeInclusive<u32> {
        self.hold_delay_min..=self.hold_delay_max
    }
}

use argh::FromArgs;
use std::ops::RangeInclusive;

#[derive(FromArgs, Debug)]
/// keyboard delay config
pub struct Config {
    /// scan 1 make code
    #[argh(option)]
    pub scan_code: u16,

    /// press delay range start (milliseconds)
    #[argh(option)]
    pub press_delay_from: u32,

    /// press delay range end (milliseconds, inclusive)
    #[argh(option)]
    pub press_delay_to: u32,

    /// hold delay range start (milliseconds)
    #[argh(option, default = "50")]
    pub hold_delay_from: u32,

    /// hold delay range end (milliseconds, inclusive)
    #[argh(option, default = "120")]
    pub hold_delay_to: u32,
}

impl Config {
    pub fn press_delay(&self) -> RangeInclusive<u32> {
        self.press_delay_from..=self.press_delay_to
    }

    pub fn hold_delay(&self) -> RangeInclusive<u32> {
        self.hold_delay_from..=self.hold_delay_to
    }
}

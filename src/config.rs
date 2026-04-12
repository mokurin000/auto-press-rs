use argh::FromArgs;
use std::ops::RangeInclusive;

/// keyboard delay config
#[derive(FromArgs, Debug)]
#[argh(description = "Demo program to simulate keyboard input")]
#[argh(help_triggers("-h", "--help"))]
pub struct Config {
    /// scan 1 make code
    #[argh(option, from_str_fn(parse_int))]
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

fn parse_int(s: &str) -> Result<u16, String> {
    if let Some(x) = s.strip_prefix("0x") {
        u16::from_str_radix(x, 16)
    } else if let Some(x) = s.strip_prefix("0b") {
        u16::from_str_radix(x, 2)
    } else if let Some(x) = s.strip_prefix("0o") {
        u16::from_str_radix(x, 8)
    } else {
        s.parse()
    }
    .map_err(|e| e.to_string())
}

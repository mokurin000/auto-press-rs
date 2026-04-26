use argh::FromArgs;
use std::ops::RangeInclusive;

/// keyboard delay config
#[derive(FromArgs, Debug, Clone)]
#[argh(description = "Demo program to simulate keyboard input")]
#[argh(help_triggers("-h", "--help"))]
pub struct Config {
    /// scan 1 make code
    #[argh(option, from_str_fn(parse_int))]
    pub scan_code: u16,

    /// minimum press delay in milliseconds
    #[argh(option)]
    pub min_interval: u32,

    /// maximium press delay in milliseconds
    #[argh(option)]
    pub max_interval: u32,

    /// minimum hold duration in milliseconds
    #[argh(option, default = "50")]
    pub min_hold_duration: u32,

    /// maximum hold duration in milliseconds
    #[argh(option, default = "120")]
    pub max_hold_duration: u32,

    /// stop after these seconds
    #[argh(option)]
    pub run_duration: Option<u64>,
}

impl Config {
    pub fn hold_duration(&self) -> RangeInclusive<u32> {
        self.min_hold_duration..=self.max_hold_duration
    }
}

pub fn parse_int(s: &str) -> Result<u16, String> {
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

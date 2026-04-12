use std::ops::{Bound, RangeBounds};

use fastrand::Rng;

pub trait NormalInRange {
    fn norm_rand(&mut self, range: impl RangeBounds<u32>) -> u32;
}

impl NormalInRange for Rng {
    fn norm_rand(&mut self, range: impl RangeBounds<u32>) -> u32 {
        let start = range.start_bound();
        let end = range.end_bound();

        normal_in_range(
            self,
            bound_to_num(start).unwrap_or(u32::MIN) as _,
            bound_to_num(end).unwrap_or(u32::MAX) as _,
        ) as _
    }
}

fn normal_in_range(rng: &mut Rng, min: f64, max: f64) -> f64 {
    let mu = (min + max) / 2.0;
    let sigma = (max - min) / 6.0;

    loop {
        let u1 = rng.f64().max(1e-12);
        let u2 = rng.f64();

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();

        let x = mu + sigma * z;

        if x >= min && x <= max {
            return x;
        }
    }
}

fn bound_to_num(bound: Bound<&u32>) -> Option<u32> {
    match bound {
        Bound::Included(&num) => Some(num),
        Bound::Excluded(num) => Some(num.saturating_sub(1)),
        Bound::Unbounded => None,
    }
}

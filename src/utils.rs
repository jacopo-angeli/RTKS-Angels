#[allow(dead_code)]
use crate::types::time::TimeStamp;
use rtic_monotonics::Monotonic;

use crate::app::Mono;

pub fn get_instant() -> TimeStamp {
    cortex_m::interrupt::free(|_| Mono::now())
}

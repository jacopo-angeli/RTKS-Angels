use rtic_monotonics::{fugit::MillisDurationU32, Monotonic};
use crate::utils::get_instant::*;

use cortex_m_semihosting::hprintln;
use crate::app::{self, Mono};

pub async fn external_event_server(cx: app::external_event_server::Context<'_>) {
    const MIN_SEP: MillisDurationU32 = MillisDurationU32::millis(5000);
    loop {
        let instant = get_instant();
        hprintln!("push button server started at {}", instant);

        cortex_m::interrupt::free(|cs| cx.shared.actv_log.write(cs));

        let final_instant = get_instant();
        hprintln!("push button server finished at {}", final_instant);

        Mono::delay_until(instant + MIN_SEP).await;
    }
}

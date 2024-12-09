#![allow(unsafe_code)]

use crate::{
    app::{self, Mono},
    config::*,
    utils::get_instant::*,
};
use cortex_m_semihosting::hprintln;
use rtic::Mutex;
use rtic_monotonics::Monotonic;

pub async fn external_event_server(mut cx: app::external_event_server::Context<'_>) {
    loop {
        let instant = get_instant();
        hprintln!("EES; start; {}; ; ;", instant);

        cx.shared.event_queue.lock(|event_queue| {
            while let Some(_event) = event_queue.dequeue() {
                // manage event
            }
        });
        
        cx.shared.actv_log.lock(|actv_log| {
            
            *actv_log = *actv_log + 1;
        });

        let final_instant = get_instant();
        hprintln!(
            "EES; finished; {}; {}; {};",
            final_instant,
            final_instant - instant,
            if (final_instant - instant) > EXTERNAL_EVENT_SERVER_DEADLINE {
                "x"
            } else {
                ""
            }
        );

        Mono::delay_until(instant + EXTERNAL_EVENT_SERVER_MIAP).await;
    }
}

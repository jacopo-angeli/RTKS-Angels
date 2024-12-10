use cortex_m_semihosting::hprintln;
use rtic_monotonics::Monotonic;
use rtic_sync::channel::Receiver;

use crate::{
    app::{self, Mono},
    config::*,
};
use crate::{types::production_workload::ProductionWorkload, utils::get_instant::*};
use rtic::Mutex;

pub async fn activation_log_reader(
    mut cx: app::activation_log_reader::Context<'_>,
    mut actv_recv: Receiver<'static, u32, 1>,
) {
    let mut production_workload: ProductionWorkload = Default::default();

    while let Ok(_) = actv_recv.recv().await {
        // as on_call_producer here task can be preempted
        let instant = get_instant();
        hprintln!("ALR; start; {}; ; ;", instant);

        production_workload.small_whetstone(ACTIVATION_LOG_READER_WORKLOAD);

        cx.shared.actv_log.lock(|_actv_log| {
            // hprintln!("ALR; read; {}", actv_log);
        });

        let final_instant = get_instant();
        hprintln!(
            "ALR; finished; {}; {}; {};",
            final_instant,
            final_instant - instant,
            if (final_instant - instant) > ACTIVATION_LOG_READER_DEADLINE {
                "x"
            } else {
                ""
            }
        );

        Mono::delay_until(instant + ACTIVATION_LOG_READER_MIAP).await;
    }
}

use cortex_m_semihosting::hprintln;
use rtic_monotonics::Monotonic;
use rtic_sync::channel::Receiver;

use crate::{
    app::{self, Mono},
    config::*,
};
use crate::{types::production_workload::ProductionWorkload, utils::get_instant::*};

pub async fn on_call_producer(
    _: app::on_call_producer::Context<'_>,
    mut receiver: Receiver<'static, u32, 5>,
) {
    let mut production_workload: ProductionWorkload = Default::default();

    while let Ok(work) = receiver.recv().await {
        // here task can be preempted, in that case it suffers jitter
        let instant = get_instant();
        hprintln!("OCP; start; {}; ; ;", instant);

        production_workload.small_whetstone(work);

        let final_instant = get_instant();
        hprintln!(
            "OCP; finished; {}; {}; {};",
            final_instant,
            final_instant - instant,
            if (final_instant - instant) > ON_CALL_PRODUCER_DEADLINE {
                "x"
            } else {
                ""
            }
        );

        Mono::delay_until(instant + ON_CALL_PRODUCER_MIAP).await
    }
}

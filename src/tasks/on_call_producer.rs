use cortex_m_semihosting::hprintln;
use rtic_monotonics::{fugit::MillisDurationU32, Monotonic};
use rtic_sync::channel::Receiver;

use crate::app::{self, Mono};
use crate::{types::production_workload::ProductionWorkload, utils::get_instant::*};

pub async fn on_call_producer(
    _: app::on_call_producer::Context<'_>,
    mut receiver: Receiver<'static, u32, 5>,
) {
    const MIN_SEP: MillisDurationU32 = MillisDurationU32::millis(3000);
    let mut production_workload: ProductionWorkload = Default::default();

    while let Ok(work) = receiver.recv().await {
        // here task can be preempted, in that case it suffers jitter
        let instant = get_instant();
        hprintln!("on call producer started at {}", instant);

        production_workload.small_whetstone(work);

        let final_instant = get_instant();
        hprintln!("on call producer finished at {}", final_instant);

        Mono::delay_until(instant + MIN_SEP).await
    }
}

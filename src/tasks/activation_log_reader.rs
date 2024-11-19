use cortex_m_semihosting::hprintln;
use rtic_sync::channel::Receiver;
use rtic_monotonics::{fugit::MillisDurationU32, Monotonic};

use crate::app::{self, Mono};
use crate::{
    types::production_workload::ProductionWorkload,
    utils::get_instant::*,
};

pub async fn activation_log_reader(
    cx: app::activation_log_reader::Context<'_>,
    mut actv_recv: Receiver<'static, u32, 1>,
) {
    const MIN_SEP: MillisDurationU32 = MillisDurationU32::millis(3000);
    const WORKLOAD: u32 = 139;
    let mut production_workload: ProductionWorkload = Default::default();

    while let Ok(_) = actv_recv.recv().await {
        // as on_call_producer here task can be preempted
        let instant = get_instant();
        hprintln!("ALR: started at {}", instant);

        production_workload.small_whetstone(WORKLOAD);

        match cx.shared.actv_log.read() {
            Ok((last_actv_counter, last_actv_instant)) => hprintln!(
                "ALR: activation number {} logged at time {}",
                last_actv_counter,
                last_actv_instant
            ),
            Err(err) => hprintln!("{}", err),
        }

        let final_instant = get_instant();
        hprintln!("ALR: finished at {}", final_instant);

        Mono::delay_until(instant + MIN_SEP).await;
    }
}

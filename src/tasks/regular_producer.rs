use crate::{
    app::{self, Mono},
    config::*,
    types::production_workload::ProductionWorkload,
    utils::{
        get_instant::get_instant, log_reader_activation::log_activation_condition,
        on_call_activation::prod_activation_condition,
    },
};
use cortex_m_semihosting::hprintln;
use rtic_monotonics::Monotonic;
use rtic_sync::channel::Sender;

pub async fn regular_producer(
    _: app::regular_producer::Context<'_>,
    mut on_call_prod_sender: Sender<'static, u32, 5>,
    mut activation_log_reader_sender: Sender<'static, u32, 1>,
) {
    let mut production_workload = ProductionWorkload::default();

    loop {
        let start_instant = get_instant();
        hprintln!("RP; start; {}; ; ;", start_instant);

        // Execute workload
        production_workload.small_whetstone(REGULAR_PRODUCER_WORKLOAD);

        // Handle on-call producer activation
        if prod_activation_condition::on_call_prod_activation_condition() {
            if let Err(_) = on_call_prod_sender.try_send(ON_CALL_PRODUCER_WORKLOAD) {
                hprintln!(
                    "RP; error(on_call_producer's buffer full); {}; ; ;",
                    start_instant
                );
            }
        }

        // Handle log reader activation
        if log_activation_condition::activation_log_reader_condition() {
            if let Err(_) = activation_log_reader_sender.try_send(0) {
                hprintln!(
                    "RP; error(activation_log_reader's buffer full); {}; ; ;",
                    start_instant
                );
            }
        }

        // Log end time
        let end_instant = get_instant();
        hprintln!(
            "RP; finished; {}; {}; ;",
            end_instant,
            end_instant - start_instant
        );

        // Wait until the next period
        Mono::delay_until(start_instant + REGULAR_PRODUCER_PERIOD).await;
    }
}

use cortex_m_semihosting::hprintln;
use rtic_sync::channel::Sender;
use crate::app::{self, Mono};
use rtic_monotonics::{fugit::MillisDurationU32, Monotonic};
use crate::{
    types::production_workload::ProductionWorkload,
    utils::get_instant,
    on_call_prod_activation::prod_activation_condition,
    log_reader_activation::log_activation_condition,
};

pub async fn regular_producer(
    _: app::regular_producer::Context<'_>,
    mut on_call_prod_sender: Sender<'static, u32, 5>,
    mut activation_log_reader_sender: Sender<'static, u32, 1>,
) {
    const REGULAR_PRODUCER_WORKLOAD: u32 = 756;
    const ON_CALL_PRODUCER_WORKLOAD: u32 = 278;
    const PERIOD: MillisDurationU32 = MillisDurationU32::millis(1000);
    let mut production_workload: ProductionWorkload = Default::default();

    loop {
        let instant = get_instant();
        hprintln!("regular producer started at { }", instant);
        production_workload.small_whetstone(REGULAR_PRODUCER_WORKLOAD);

        if prod_activation_condition::on_call_prod_activation_condition()
            && let Err(_) = on_call_prod_sender.try_send(ON_CALL_PRODUCER_WORKLOAD)
        {
            hprintln!("on call producer activation failed due to full buffer")
        }

        if log_activation_condition::activation_log_reader_condition()
            && let Err(_) = activation_log_reader_sender.try_send(0)
        {
            hprintln!("activation log reader failed due to full buffer")
        }

        let final_instant = get_instant();
        hprintln!("regular producer finished at { }", final_instant);

        Mono::delay_until(instant + PERIOD).await;
    }
}
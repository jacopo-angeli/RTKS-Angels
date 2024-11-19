use cortex_m_semihosting::hprintln;
use rtic_sync::channel::Sender;
use crate::app::{self, Mono};
use rtic_monotonics::{fugit::MillisDurationU32, Monotonic};
use crate::{
    types::production_workload::ProductionWorkload,
    utils::get_instant::*,
    utils::on_call_activation::prod_activation_condition,
    utils::log_reader_activation::log_activation_condition,
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
        hprintln!("RP: started at { }", instant);
        production_workload.small_whetstone(REGULAR_PRODUCER_WORKLOAD);

        if prod_activation_condition::on_call_prod_activation_condition()
        {
            match on_call_prod_sender.try_send(ON_CALL_PRODUCER_WORKLOAD)
            {
                Ok(_) => hprintln!("RP: on call producer buffer updated"),
                Err(_) => hprintln!("RP: on call producer buffer full")
            }
        }

        if log_activation_condition::activation_log_reader_condition()
        {
            match activation_log_reader_sender.try_send(0)
            {
                Ok(_) => hprintln!("RP: activation log reader buffer updated"),
                Err(_) => hprintln!("RP: activation log reader buffer full")
            }
        }

        let final_instant = get_instant();
        hprintln!("RP: finished at { }", final_instant);

        Mono::delay_until(instant + PERIOD).await;
    }
}
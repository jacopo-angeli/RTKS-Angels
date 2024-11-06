#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]
#![feature(proc_macro_hygiene)]
#![feature(let_chains)]

mod tasks;
mod types;
mod utils;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1, USART2, USART3, TIM3, TIM2])]
mod app {

    use crate::tasks::regular_producer::*;
    use crate::tasks::on_call_producer::*;
    use crate::tasks::activation_log_reader::*;
    use crate::tasks::external_event_server::*;
    use crate::types::activation_log::ActivationLog;

    use cortex_m_semihosting::hprintln;
    use panic_semihosting as _;
    use rtic_monotonics::systick::prelude::*;
    use rtic_sync::{
        channel::{Receiver, Sender},
        make_channel,
    };

    systick_monotonic!(Mono, 1000); // Mono is a monotonic timer that interrupts with rate 1khz, a.k.a 1 ms

    // shared resources
    #[shared]
    struct Shared {
        actv_log: ActivationLog,
    }

    // local resources
    #[local]
    struct Local {}

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle");
        loop {}
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // 12 MHz is the clock rate (in QEMU) associated with Systick timer
        Mono::start(cx.core.SYST, 12_000_000);

        let (on_call_prod_sender, on_call_prod_recv) = make_channel!(u32, 5);
        let (actv_log_reader_sender, actv_log_reader_recv) = make_channel!(u32, 1);

        regular_producer::spawn(on_call_prod_sender, actv_log_reader_sender).ok();
        on_call_producer::spawn(on_call_prod_recv).ok();
        external_event_server::spawn().ok();
        activation_log_reader::spawn(actv_log_reader_recv).ok();

        (
            Shared {
                actv_log: ActivationLog::build(),
            },
            Local {},
        )
    }

    extern "Rust" {

        #[task(priority = 6)]
        async fn regular_producer(
            cx: regular_producer::Context,
            mut send1: Sender<'static, u32, 5>,
            mut send2: Sender<'static, u32, 1>,
        );

        #[task(priority = 4)]
        async fn on_call_producer(
            cx: on_call_producer::Context,
            mut recv: Receiver<'static, u32, 5>,
        );

        // this task is a sporadic task that serve an aperiodic (hardware) interrupt
        #[task(priority = 7, shared = [&actv_log])]
        async fn external_event_server(mut cx: external_event_server::Context);

        #[task(priority = 2, shared = [&actv_log])]
        async fn activation_log_reader(
            mut cx: activation_log_reader::Context,
            mut recv1: Receiver<'static, u32, 1>,
        );
    }
}

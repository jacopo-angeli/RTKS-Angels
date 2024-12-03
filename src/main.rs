#![no_main]
#![no_std]
#![deny(warnings)]
#![deny(unsafe_code)]
#![feature(proc_macro_hygiene)]
#![feature(let_chains)]

mod tasks;
mod types;
mod utils;
mod config;


#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1, USART2, USART3, TIM3])]
mod app {
    use heapless::spsc::Queue;
    use stm32f4xx_hal::rtc::Event;
    use stm32f4xx_hal::{pac, prelude::*, rcc::RccExt};

    use crate::config::*;
    use crate::tasks::activation_log_reader::*;
    use crate::tasks::external_event_server::*;
    use crate::tasks::on_call_producer::*;
    use crate::tasks::regular_producer::*;

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
        actv_log: u32,
        event_queue: Queue<Event, 10>,
    }

    // local resources
    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        // Three different clock sources can be used to drive the system clock (SYSCLK):
        //  • HSI oscillator clock
        //  • HSE oscillator clock
        //  • Main PLL (PLL) clock

        // The HSI clock signal is generated from an internal 16 MHz RC oscillator and can be used
        // directly as a system clock, or used as PLL input.
        // The HSI RC oscillator has the advantage of providing a clock source at low cost (no external
        // components). It also has a faster startup time than the HSE crystal oscillator however, even
        // with calibration the frequency is less accurate than an external crystal oscillator or ceramic
        // resonator.

        // The high speed external clock signal (HSE) can be generated from two possible clock sources:
        //  • HSE external crystal/ceramic resonator
        //  • HSE external user clock
        // The resonator and the load capacitors have to be placed as close as possible to the
        // oscillator pins in order to minimize output distortion and startup stabilization time. The
        // loading capacitance values must be adjusted according to the selected oscillator.

        // The STM32F4xx devices feature three PLLs:
        //   • A main PLL (PLL) clocked by the HSE or HSI oscillator and featuring two different
        //     output clocks:
        //       – The first output is used to generate the high speed system clock (up to 180 MHz)
        //       – The second output is used to generate the clock for the USB OTG FS (48 MHz),
        //         the random analog generator (≤48 MHz) and the SDIO (≤48 MHz).
        //   • Two dedicated PLLs (PLLI2S and PLLSAI) used to generate an accurate clock to
        //      achieve high-quality audio performance on the I2S and SAI1 interfaces. PLLSAI is also
        //      used for the LCD-TFT clock.

        let dp: pac::Peripherals = cx.device;
        let rcc = dp.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz()) // HSE clock: 8 MHz
            .sysclk(180.MHz()) // System clock: 180 MHz
            .require_pll48clk() // USB/SDIO requires 48 MHz PLL clock
            .freeze();
        
        // RUN SETTINGS
        hprintln!("RP;{};{};{};{}", REGULAR_PRODUCER_WORKLOAD, REGULAR_PRODUCER_DEADLINE, REGULAR_PRODUCER_PERIOD, 7);
        hprintln!("OCP;{};{};{};{}", ON_CALL_PRODUCER_WORKLOAD, ON_CALL_PRODUCER_DEADLINE, ON_CALL_PRODUCER_MIAP, 5);
        hprintln!("ALR;{};{};{};{}", ACTIVATION_LOG_READER_WORKLOAD, ACTIVATION_LOG_READER_DEADLINE, ACTIVATION_LOG_READER_MIAP, 3);
        hprintln!("EES;;{};{};{}", EXTERNAL_EVENT_SERVER_DEADLINE, EXTERNAL_EVENT_SERVER_MIAP, 11);

        let (on_call_prod_sender, on_call_prod_recv) = make_channel!(u32, 5);
        let (actv_log_reader_sender, actv_log_reader_recv) = make_channel!(u32, 1);

        regular_producer::spawn(on_call_prod_sender, actv_log_reader_sender).ok();
        on_call_producer::spawn(on_call_prod_recv).ok();
        external_event_server::spawn().ok();
        activation_log_reader::spawn(actv_log_reader_recv).ok();

        Mono::start(cx.core.SYST, clocks.sysclk().to_Hz());
        (
            Shared {
                actv_log: 0,
                event_queue: Queue::new()
            },
            Local {},
        )
    }

    extern "Rust" {

        #[task(priority = 7)]
        async fn regular_producer(
            cx: regular_producer::Context,
            mut send1: Sender<'static, u32, 5>,
            mut send2: Sender<'static, u32, 1>,
        );

        #[task(priority = 5)]
        async fn on_call_producer(
            cx: on_call_producer::Context,
            mut recv: Receiver<'static, u32, 5>,
        );

        // this task is a sporadic task that serve an aperiodic (hardware) interrupt
        #[task(priority = 11, shared = [actv_log, event_queue])]
        async fn external_event_server(mut cx: external_event_server::Context);

        #[task(priority = 3, shared = [actv_log])]
        async fn activation_log_reader(
            mut cx: activation_log_reader::Context,
            mut recv1: Receiver<'static, u32, 1>,
        );
    }
}

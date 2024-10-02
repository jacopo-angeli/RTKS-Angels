#![no_std]
#![no_main]

#![allow(unused)]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal as f4;

use bern_kernel::exec::process::Process;
use bern_kernel::exec::runnable::Priority;
use bern_kernel::exec::thread::Thread;
use bern_kernel::stack::Stack;
use bern_kernel::sleep;
use bern_kernel::units::frequency::ExtMilliHertz;

use rtt_target::{rtt_init_print, rprintln};

static PROC: &Process = bern_kernel::new_process!(my_process1, 8192);


mod regular_produces;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = f4::pac::Peripherals::take().expect("Peripherals cannot be accessed");

    let gpio_g = dp.GPIOG.split();

    let mut green_led = gpio_g.pg13.into_push_pull_output();
    let mut blue_led = gpio_g.pg14.into_push_pull_output();

    bern_kernel::kernel::init();
    bern_kernel::time::set_tick_frequency(2.kHz(), 180.MHz());

    PROC.init(move |c| {
        // Regular producer
        regular_produces::spawn_green_led_thread(blue_led, c);
        
    }).unwrap();

    bern_kernel::start()
}

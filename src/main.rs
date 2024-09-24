#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use bern_kernel::exec::process::Process;
use bern_kernel::exec::runnable::Priority;
use bern_kernel::exec::thread::Thread;
use bern_kernel::stack::Stack;
use bern_kernel::sleep;
use bern_kernel::units::frequency::ExtMilliHertz;

use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::prelude::_stm32f4xx_hal_rcc_RccExt;

// Allocate a process.
static PROC: &Process = bern_kernel::new_process!(my_process, 8192);

#[entry]
fn main() -> ! {
    //take control (ownership) of all the peripherals
    let p = Peripherals::take()
        .expect("cannot take stm32 peripherals");

    let rcc = p.RCC.constrain();
    let _clock = rcc.cfgr.sysclk(stm32f4xx_hal::prelude::_fugit_RateExtU32::MHz(48)).freeze();

    // Set-up GPIOs
    let gpiog = p.GPIOG.split();
    let mut led = gpiog.pg13.into_push_pull_output().erase();

    let mut led2 = gpiog.pg14.into_push_pull_output().erase();
    // Another led maybe at gipio pg14 

    bern_kernel::time::set_tick_frequency(
        1.kHz(),
        48.MHz()
    );

    // Create a new process.
    // All threads within a process share the same protected memory region.
    PROC.init(move |c| {

        Thread::new(c)
            .priority(Priority::new(0))
            .stack(Stack::try_new_in(c, 128).unwrap())
            .spawn(move || {
                loop {
                    led.set_high();
                    sleep(200);
                    led.set_low();
                    sleep(800);
                }
            });

    }).unwrap();
    //LE TASK INIZIANO QUI PORCODDIO
    //TODO LEGGERE BERN DOCUMENTATION
    bern_kernel::start();
}

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    use defmt_rtt as _;

    // Resources shared between tasks
    #[shared]
    struct Shared {}

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        (
            
            // Initialization of shared resources
            Shared {},
            // Initialization of task local resources
            Local {},
        )
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(mut ctx: idle::Context) -> ! {
        loop {}
    }
    
}

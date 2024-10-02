use bern_kernel::embedded_time::duration::Duration;
// led_green.rs
use bern_kernel::exec::process::Context;
use bern_kernel::exec::thread::Thread;
use bern_kernel::sleep;
use bern_kernel::stack::Stack;
use rtt_target::rprintln;
use stm32f4xx_hal::gpio::gpiog::PG14;
use stm32f4xx_hal::gpio::{Output, PushPull};

pub fn spawn_green_led_thread(mut green_led: PG14<Output<PushPull>>, c: &Context) {
    Thread::new(c)
        .stack(Stack::try_new_in(c, 1024).unwrap())
        .spawn(move || {
            let mut next_time = Time::now();
            let period = Duration::from_millis(REGULAR_PRODUCER_PERIOD);

            loop {
                next_time = next_time + period;

                // Non-suspending operation (perform workload)
                regular_producer_operation();

                // Delay until next time to maintain periodicity
                Timer::delay_until(next_time).unwrap_or_else(|_| {
                    rprintln!("Something went wrong with the timer.");
                });
            }
        });
}

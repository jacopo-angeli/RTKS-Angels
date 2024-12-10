use rtic_monotonics::fugit::Instant;

pub type TimeStamp = Instant<u32, 1, 100000>;

// target remote :3333
// monitor arm semihosting enable

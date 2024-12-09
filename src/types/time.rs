use rtic_monotonics::fugit::Instant;

pub type TimeStamp = Instant<u32, 1, 1000000>;

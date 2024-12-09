use rtic_monotonics::fugit::MicrosDurationU32;
// WORKLOADS
pub const REGULAR_PRODUCER_WORKLOAD: u32 = 756;
pub const ON_CALL_PRODUCER_WORKLOAD: u32 = 278;
pub const ACTIVATION_LOG_READER_WORKLOAD: u32 = 139;

// DEADLINES (in milliseconds)
pub const REGULAR_PRODUCER_DEADLINE: MicrosDurationU32 = MicrosDurationU32::micros(500000);
pub const ON_CALL_PRODUCER_DEADLINE: MicrosDurationU32 = MicrosDurationU32::micros(800000);
pub const ACTIVATION_LOG_READER_DEADLINE: MicrosDurationU32 = MicrosDurationU32::micros(1000000);
pub const EXTERNAL_EVENT_SERVER_DEADLINE: MicrosDurationU32 = MicrosDurationU32::micros(100000);

// PERIODS (in milliseconds)
pub const REGULAR_PRODUCER_PERIOD: MicrosDurationU32 = MicrosDurationU32::micros(1000000);
pub const ON_CALL_PRODUCER_MIAP: MicrosDurationU32 = MicrosDurationU32::micros(3000000);
pub const ACTIVATION_LOG_READER_MIAP: MicrosDurationU32 = MicrosDurationU32::micros(3000000);
pub const EXTERNAL_EVENT_SERVER_MIAP: MicrosDurationU32 = MicrosDurationU32::micros(5000000);

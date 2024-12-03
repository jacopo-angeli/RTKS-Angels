use rtic_monotonics::fugit::MillisDurationU32;
// WORKLOADS
pub const REGULAR_PRODUCER_WORKLOAD: u32 = 7350;
pub const ON_CALL_PRODUCER_WORKLOAD: u32 = 278;
pub const ACTIVATION_LOG_READER_WORKLOAD: u32 = 139;

// DEADLINES (in milliseconds)
pub const REGULAR_PRODUCER_DEADLINE: MillisDurationU32 = MillisDurationU32::millis(500);
pub const ON_CALL_PRODUCER_DEADLINE: MillisDurationU32 = MillisDurationU32::millis(800);
pub const ACTIVATION_LOG_READER_DEADLINE: MillisDurationU32 = MillisDurationU32::millis(1000);
pub const EXTERNAL_EVENT_SERVER_DEADLINE: MillisDurationU32 = MillisDurationU32::millis(100);

// PERIODS (in milliseconds)
pub const REGULAR_PRODUCER_PERIOD: MillisDurationU32 = MillisDurationU32::millis(1000);
pub const ON_CALL_PRODUCER_MIAP: MillisDurationU32 = MillisDurationU32::millis(3000);
pub const ACTIVATION_LOG_READER_MIAP: MillisDurationU32 = MillisDurationU32::millis(3000);
pub const EXTERNAL_EVENT_SERVER_MIAP: MillisDurationU32 = MillisDurationU32::millis(5000);

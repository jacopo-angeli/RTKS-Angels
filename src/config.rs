use rtic_monotonics::fugit::Duration;
// WORKLOADS
pub const REGULAR_PRODUCER_WORKLOAD: u32 = 756 + (756 * 288 / 100);
pub const ON_CALL_PRODUCER_WORKLOAD: u32 = 278 + (278 * 288 / 100);
pub const ACTIVATION_LOG_READER_WORKLOAD: u32 = 139 + (139 * 288 / 100);

// DEADLINES (in milliseconds)
pub const REGULAR_PRODUCER_DEADLINE: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(50000);
pub const ON_CALL_PRODUCER_DEADLINE: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(80000);
pub const ACTIVATION_LOG_READER_DEADLINE: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(10000);
pub const EXTERNAL_EVENT_SERVER_DEADLINE: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(10000);

// PERIODS (in milliseconds)
pub const REGULAR_PRODUCER_PERIOD: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(100000);
pub const ON_CALL_PRODUCER_MIAP: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(300000);
pub const ACTIVATION_LOG_READER_MIAP: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(300000);
pub const EXTERNAL_EVENT_SERVER_MIAP: Duration<u32, 1, 100000> = Duration::<u32, 1, 100000>::from_ticks(500000);

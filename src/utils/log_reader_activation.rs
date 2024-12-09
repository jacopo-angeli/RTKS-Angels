#[allow(dead_code)]
pub mod log_activation_condition {
    use core::sync::atomic::AtomicI32;
    use core::sync::atomic::Ordering;

    const LOG_READER_MOD: i32 = 1000;
    const LOG_READER_ACTV_RATIO: i32 = 3;
    static LOG_READER_ACTV_REQUEST: AtomicI32 = AtomicI32::new(0);

    
    pub fn activation_log_reader_condition() -> bool {
        LOG_READER_ACTV_REQUEST.fetch_add(1, Ordering::Relaxed);
        LOG_READER_ACTV_REQUEST.load(Ordering::Relaxed)
            % LOG_READER_MOD
            % LOG_READER_ACTV_RATIO
            == 0
    }
}
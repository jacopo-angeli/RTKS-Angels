#[allow(dead_code)]
pub mod prod_activation_condition {
    use core::sync::atomic::AtomicI32;
    use core::sync::atomic::Ordering;

    const ON_CALL_PROD_MOD: i32 = 5;
    static ON_CALL_PROD_ACTV_REQUEST: AtomicI32 = AtomicI32::new(0);

    // true dopo 0 chiamate e in seguito ogni 6 chiamate
    // #call  -> 0 1 2 3 4 5 6 7
    // result -> 0 0 1 0 0 0 0 1
    pub fn on_call_prod_activation_condition() -> bool {
        ON_CALL_PROD_ACTV_REQUEST.fetch_add(1, Ordering::Relaxed);
        ON_CALL_PROD_ACTV_REQUEST.load(Ordering::Relaxed) % ON_CALL_PROD_MOD == 2
    }
}
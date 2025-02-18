#[macro_export]
macro_rules! rate_limit {
    ($duration:expr) => {
        RateLimitLayer::<RealIp>::builder()
            .with_default_quota(Quota::simple(std::time::Duration::from_millis($duration)))
            .default_handle_error()
    };
}

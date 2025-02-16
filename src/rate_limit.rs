#[macro_export]
macro_rules! rate_limit {
    ($per_second:expr, $burst_size:expr) => {
        Arc::new(
            GovernorConfigBuilder::default()
                .per_second($per_second)
                .burst_size($burst_size)
                .finish()
                .unwrap(),
        )
    };
}

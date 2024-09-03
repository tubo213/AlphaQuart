// ref: https://zenn.dev/tipstar0125/articles/245bceec86e40a#time-keeper
#[derive(Debug, Clone)]
pub struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    pub fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    #[inline]
    pub fn is_time_over(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        elapsed_time >= self.time_threshold
    }
}

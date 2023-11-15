use std::time::Instant;

pub(crate) struct Timer {
    start_time: Instant,
}

impl Timer {
    pub(crate) fn start_timer() -> Self {
        Timer {
            start_time: Instant::now(),
        }
    }

    pub(crate) fn get_time_seconds(&self) -> f64 {
        let elapsed = self.start_time.elapsed();
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1e9
    }
}
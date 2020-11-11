use super::Limiter;

/// Nothing special here, just using thread::sleep.
pub struct LimiterImpl {}

impl Default for LimiterImpl {
    fn default() -> Self {
        LimiterImpl {

        }
    }
}

impl LimiterImpl {
    pub fn new() -> Self {
        LimiterImpl::default()
    }
}

impl Limiter for LimiterImpl {
    fn wait(&self, time: std::time::Duration) {
        std::thread::sleep(time)
    }
}

pub fn create() -> Box<dyn Limiter> {
    Box::new(LimiterImpl::new())
}

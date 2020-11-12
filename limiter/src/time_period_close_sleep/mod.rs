use super::Limiter;

pub struct LimiterImpl {
    close_enough: std::time::Duration,

    #[cfg(windows)]
    min_period: u32,
}

impl Default for LimiterImpl {
    fn default() -> Self {
        let mut result = LimiterImpl {
            #[cfg(windows)]
            min_period: LimiterImpl::get_min_period(),
            close_enough: std::time::Duration::from_micros(500)
        };
        result.init();
        result
    }
}

impl Drop for LimiterImpl {
    fn drop(&mut self) {
        self.shutdown()
    }
}

impl LimiterImpl {
    pub fn new() -> Self {
        LimiterImpl::default()
    }
}

impl Limiter for LimiterImpl {
    fn wait(&self, time: std::time::Duration) {
        // Current time.
        let mut now = std::time::Instant::now();

        // Only bother if we are within tolerance.
        if time > self.close_enough
        {
            // So long as we have waited this amount of time, we are good.
            let close_enough = time - self.close_enough;

            // We'll go by halves till we get into range.
            // NOTE: If close_enough to too small, this ends up just doing a busy wait.
            let mut current = time / 2;

            // Total we have waited so far.
            let mut total = std::time::Duration::from_millis(0);

            while total < close_enough &&
                current > self.close_enough
            {
                std::thread::sleep(current);

                // Get the new time and increase total.
                let last_now = now;
                now = std::time::Instant::now();

                let delta = now - last_now;
                total += delta;

                // How long to wait during the next loop.
                if time > total {
                    current = (time - total) / 2;
                } else {
                    return
                }
            }
        }
    }
}

#[cfg(windows)]
impl LimiterImpl {
    fn get_min_period() -> u32 {
        use std::mem;
        use winapi::um::{mmsystem::*, timeapi::timeGetDevCaps};

        let mut time_caps = TIMECAPS {
            wPeriodMin: 0,
            wPeriodMax: 0,
        };

        unsafe {
            let time_caps_size = mem::size_of::<TIMECAPS>() as u32;
            if timeGetDevCaps(&mut time_caps as *mut TIMECAPS, time_caps_size) == TIMERR_NOERROR {
                time_caps.wPeriodMin
            } else {
                1
            }
        }
    }

    fn init(&mut self) {
        use winapi::um::timeapi::{timeBeginPeriod};
        println!("Setting min period to: {}", self.min_period);
        unsafe { timeBeginPeriod(self.min_period); }
    }

    fn shutdown(&mut self) {
        use winapi::um::timeapi::{timeEndPeriod};
        println!("Ending time period.");
        unsafe { timeEndPeriod(self.min_period); }
    }
}

#[cfg(not(windows))]
impl LimiterImpl {
    fn init(&mut self) {
    }

    fn shutdown(&mut self) {
    }
}

pub fn create() -> Box<dyn Limiter> {
    Box::new(LimiterImpl::new())
}

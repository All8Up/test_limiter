use super::Limiter;

pub struct LimiterImpl {
    min_period: u32
}

impl Default for LimiterImpl {
    fn default() -> Self {
        let mut result = LimiterImpl {
            min_period: LimiterImpl::get_min_period()
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
        std::thread::sleep(time)
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
mod os_specific {

}

pub fn create() -> Box<dyn Limiter> {
    Box::new(LimiterImpl::new())
}

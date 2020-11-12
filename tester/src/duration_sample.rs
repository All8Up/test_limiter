use std::time::Duration;

pub struct DurationSample {
    accumulator: Duration,
    min: Duration,
    max: Duration,
    count: u32
}

impl Default for DurationSample {
    fn default() -> Self {
        DurationSample {
            accumulator: Duration::default(),
            min: Duration::from_secs(100000),
            max: Duration::default(),
            count: 0
        }
    }
}

impl DurationSample {
    pub fn min(&self) -> Duration {
        self.min
    }

    pub fn max(&self) -> Duration {
        self.max
    }

    pub fn duration(&self) -> Duration {
        self.accumulator
    }

    pub fn average(&self) -> Duration {
        self.accumulator / self.count
    }

    pub fn add(&mut self, value: Duration) {
        self.count += 1;
        self.accumulator += value;
        self.min = if self.min <= value { self.min } else { value };
        self.max = if self.max >= value { self.max } else { value };
    }

    pub fn capture<F>(&mut self, f: F) where F: FnOnce() {
        let start = std::time::Instant::now();
        {
            f();
        }
        let end = std::time::Instant::now();
        self.add(end - start);
    }
}

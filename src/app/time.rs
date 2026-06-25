use std::time::{Duration, Instant};

pub struct Clock {
    last_instant: Instant,
    last_delta: Duration,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            last_delta: Duration::new(0,0),
        }
    }

    pub fn delta(&self) -> Duration {
        self.last_delta
    }

    pub fn delta_seconds(&self) -> f32 {
        self.last_delta.as_secs_f32()
    }

    pub fn fps(&self) -> f32  {
        let delta: f32 = self.last_delta.as_secs_f32();
        if delta > 0.0 {
            1.0 / delta
        } else {
            0.0
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_instant);
        self.last_instant = now;
        self.last_delta = delta;
    }
    

}
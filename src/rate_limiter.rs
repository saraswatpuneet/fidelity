use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub struct RateLimiter {
    pub max_requests: usize,
    pub timestamps: VecDeque<Instant>,
    pub time_window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, time_window: Duration) -> Self {
        RateLimiter {
            max_requests,
            timestamps: VecDeque::new(),
            time_window,
        }
    }

    pub fn is_allowed(&mut self) -> bool {
        let now = Instant::now();
        // First remove all the timestamps older than now
        while let Some(&oldest) = self.timestamps.front() {
            if now.duration_since(oldest) > self.time_window {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
        if self.timestamps.len() < self.max_requests {
            self.timestamps.push_back(now);
            true
        } else {
            false
        }
    }
}

pub fn run_rate_limiter() {
    let mut limiter = RateLimiter::new(3, Duration::from_secs(10));
    for i in 0..10 {
        let is_allowed = limiter.is_allowed();
        println!("Request {:?} Is Allowed: {:?}", i, is_allowed);
        std::thread::sleep(Duration::from_secs(2));
    }
}

use dashmap::DashMap;
use std::time::{Instant, Duration};

pub struct Scheduler {
    map: DashMap<String, Instant>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { map: DashMap::new() }
    }

    pub fn allow(&self, domain: &str) -> bool {
        let now = Instant::now();

        match self.map.get(domain) {
            Some(last) if now.duration_since(*last) < Duration::from_secs(2) => false,
            _ => {
                self.map.insert(domain.to_string(), now);
                true
            }
        }
    }
}

use vclock::VClock;

use crate::types::{DataEntry, DataMap};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Device {
    pub id: String,
    pub clock: VClock<String>,
    pub data: DataMap,
}

impl Device {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            clock: VClock::new(id.to_string()),
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.clock.incr(&self.id);
        self.data.insert(
            key.to_string(),
            DataEntry {
                value: value.to_string(),
                timestamp: self.clock.clone(),
            },
        );
    }

    pub fn merge(&mut self, other: &DataMap) {
        for (key, entry) in other {
            self.data
                .entry(key.clone())
                .and_modify(|existing| {
                    if entry.timestamp > existing.timestamp {
                        *existing = entry.clone();
                    }
                })
                .or_insert_with(|| entry.clone());
        }
    }
}

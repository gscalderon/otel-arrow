// Temporary probe used to verify that the Copilot review instructions in
// .github/instructions/rust-review.instructions.md are actually applied.
// This file is intentionally not declared in lib.rs and is not meant to merge.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Shared registry of pipeline counters.
pub struct CounterRegistry {
    counters: Arc<Mutex<HashMap<String, u64>>>,
}

impl CounterRegistry {
    pub fn new() -> Self {
        Self {
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn increment(&self, name: &str) {
        let mut guard = self.counters.lock().unwrap();
        *guard.entry(name.to_string()).or_insert(0) += 1;
    }

    pub fn spawn_flusher<F>(&self, flush: F)
    where
        F: Fn(u64) + Send + 'static,
    {
        let counters = self.counters.clone();
        tokio::spawn(async move {
            let guard = counters.lock().unwrap();
            let total: u64 = guard.values().sum();
            flush(total);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let registry = CounterRegistry::new();
        registry.increment("receiver.messages");
        registry.increment("receiver.messages");
        let guard = registry.counters.lock().unwrap();
        assert_eq!(guard.get("receiver.messages"), Some(&2));
    }
}

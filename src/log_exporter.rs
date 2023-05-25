use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc, Mutex},
};

use metrics::{atomics::AtomicU64, Counter, CounterFn, Key, Recorder};

pub struct LogCounter {
    key: Key,
    value: AtomicU64,
}

impl CounterFn for LogCounter {
    fn increment(&self, value: u64) {
        self.value.fetch_add(value, Ordering::SeqCst);

        println!(
            "Incremented `{}` counter to {}",
            self.key.name(),
            self.value.load(Ordering::SeqCst)
        );
    }

    fn absolute(&self, value: u64) {
        self.value.store(value, Ordering::SeqCst);
    }
}

pub struct LogRecorder {
    state: Mutex<HashMap<String, Arc<LogCounter>>>,
}

impl LogRecorder {
    pub fn init() {
        let this = Self::new();

        metrics::set_boxed_recorder(Box::new(this)).unwrap();
    }

    fn new() -> Self {
        Self {
            state: Mutex::new(HashMap::new()),
        }
    }
}

impl Recorder for LogRecorder {
    fn describe_counter(
        &self,
        _key: metrics::KeyName,
        _unit: Option<metrics::Unit>,
        _description: metrics::SharedString,
    ) {
        todo!()
    }

    fn describe_gauge(
        &self,
        _key: metrics::KeyName,
        _unit: Option<metrics::Unit>,
        _description: metrics::SharedString,
    ) {
        todo!()
    }

    fn describe_histogram(
        &self,
        _key: metrics::KeyName,
        _unit: Option<metrics::Unit>,
        _description: metrics::SharedString,
    ) {
        todo!()
    }

    fn register_counter(&self, key: &metrics::Key) -> metrics::Counter {
        let mut guard = self.state.lock().unwrap();
        let counter = guard
            .entry(key.name().to_owned())
            .or_insert_with(|| {
                Arc::new(LogCounter {
                    key: key.clone(),
                    value: AtomicU64::new(0),
                })
            })
            .clone();

        Counter::from_arc(counter)
    }

    fn register_gauge(&self, _key: &metrics::Key) -> metrics::Gauge {
        todo!()
    }

    fn register_histogram(&self, _key: &metrics::Key) -> metrics::Histogram {
        todo!()
    }
}

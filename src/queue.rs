use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

pub(crate) struct QueueJob {
    pub path: String,
    pub req_time: Instant,
    pub event_type: String,
    pub json_str: String,
}

impl QueueJob {
    pub fn new(path: String, req_time: Instant, event_type: String, json_str: String) -> Self {
        QueueJob {
            path,
            req_time,
            event_type,
            json_str,
        }
    }
}

pub(crate) struct Queue {
    q: HashMap<String, VecDeque<QueueJob>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue { q: HashMap::new() }
    }

    /// For the per-repo program at `path`, push a new request.
    pub fn push_back(&mut self, qj: QueueJob) {
        self.q
            .entry(qj.path.clone())
            .or_insert_with(VecDeque::new)
            .push_back(qj);
    }

    /// For the per-repo program at `path`, push an old request that has had to be requeued due to
    /// a (hopefully) temporary error.
    pub fn push_front(&mut self, qj: QueueJob) {
        self.q
            .entry(qj.path.clone())
            .or_insert_with(VecDeque::new)
            .push_front(qj);
    }

    /// If the queue has one or more entries, pop one and return it, or `None` otherwise.
    pub fn pop(&mut self) -> Option<QueueJob> {
        // We find the oldest element in the queue and pop that.
        let mut earliest_time = None;
        let mut earliest_key = None;
        for (k, v) in self.q.iter() {
            if let Some(qj) = v.get(0) {
                if let Some(et) = earliest_time {
                    if et > qj.req_time {
                        continue;
                    }
                }
                earliest_time = Some(qj.req_time);
                earliest_key = Some(k.clone());
            }
        }
        if let Some(k) = earliest_key {
            Some(self.q.get_mut(&k).unwrap().pop_front().unwrap())
        } else {
            None
        }
    }
}

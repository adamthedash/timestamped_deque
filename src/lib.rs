use datetime::Instant;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TimewiseDeque<T> {
    queue: VecDeque<(Instant, T)>,
}

impl<T> TimewiseDeque<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Adds a new item to the queue. Must be newer than all previously added items.
    pub fn add_item(&mut self, timestamp: Instant, item: T) {
        if !self.queue.is_empty() {
            assert!(
                self.queue.back().unwrap().0 <= timestamp,
                "Items must be in chronological order"
            )
        }

        self.queue.push_back((timestamp, item));
    }

    /// Deletes all items before the specified time.
    pub fn prune_before(&mut self, timestamp: &Instant) {
        while self.queue.front().is_some_and(|(t, _)| t < timestamp) {
            self.queue.pop_front();
        }
    }

    /// Returns an iterator over items since the given timestamp
    pub fn fetch_items_since(&self, timestamp: Instant) -> impl Iterator<Item = &(Instant, T)> {
        let (s1, s2) = self.queue.as_slices();
        s1.iter().chain(s2).skip_while(move |(t, _)| t < &timestamp)
    }
}

impl<T> Default for TimewiseDeque<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use datetime::{Duration, Instant};

    use crate::TimewiseDeque;

    #[test]
    fn it_works() {
        #[derive(Debug)]
        struct Event {
            val: usize,
        }

        let mut queue = TimewiseDeque::<Event>::new();

        queue.add_item(Instant::now(), Event { val: 10 });
        println!("{:?}", queue);
        let now = Instant::now();
        queue.add_item(now, Event { val: 10 });
        println!("{:?}", queue);

        queue.prune_before(&now);
        println!("{:?}", queue);
        queue.prune_before(&Instant::now());
        println!("{:?}", queue);

        queue.add_item(Instant::now(), Event { val: 10 });
        queue.add_item(Instant::now(), Event { val: 10 });
        queue.add_item(Instant::now(), Event { val: 10 });

        for item in queue.fetch_items_since(now) {
            println!("{:?}", item)
        }

        let sum = queue
            .fetch_items_since(now - Duration::of_ms(10, 0))
            .fold(0, |acc, (t, event)| acc + event.val);
        println!("{sum}")
    }
}

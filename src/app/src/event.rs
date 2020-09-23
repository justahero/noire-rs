
#[derive(Debug)]
pub enum WindowEvent {
    /// Send to update the application
    Update,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event<T> {
    pub event: T,
}

/// The Events list contains a number of events within a given frame
#[derive(Debug)]
pub struct Events<T> {
    /// The list of all events
    events: Vec<Event<T>>,
    /// The current counf to iterate over
    count: usize,
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            count: 0,
        }
    }
}

impl<T> Events<T> {
    pub fn add(&mut self, event: T) -> &mut Self {
        self.events.push(Event { event });
        self
    }

    /// Returns the number of events
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Creates a new iterator, drains all events
    pub fn drain<'a>(&mut self) -> Vec<T> {
        self.events
            .drain(..)
            .map(|event_type| event_type.event)
            .collect()
    }
}

impl<T: Copy> Iterator for Events<T> {
    type Item = Event<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.events.len() > self.count {
            self.count += 1;
            Some(self.events[self.count - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Events;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct TestEvent {}

    #[test]
    fn test_iterator() {
        let mut events = Events::<TestEvent>::default();
        events.add(TestEvent{});

        assert_eq!(1, events.len());
        assert_eq!(Some(Event::<TestEvent>{ event: TestEvent{}}), events.next());
        assert_eq!(None, events.next());
    }

    #[test]
    fn test_drain_events() {
        let mut events = Events::<TestEvent>::default();
        events.add(TestEvent{});
        events.add(TestEvent{});

        assert_eq!(2, events.len());
        assert_eq!(vec![TestEvent{}, TestEvent{}], events.drain());
        assert_eq!(0, events.len());
    }
}

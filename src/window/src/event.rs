use crate::WindowId;

#[derive(Debug, Clone, Copy)]
pub struct EventType<T> {
    pub event: T,
}

/// The Events list contains a number of events within a given frame
#[derive(Debug)]
pub struct Events<T> {
    /// The list of all events
    events: Vec<EventType<T>>,
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
        let event_type = EventType {
            event,
        };

        self.events.push(event_type);
        self
    }
}

impl<T: Copy> Iterator for Events<T> {
    type Item = EventType<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.events.len() < self.count {
            self.count += 1;
            Some(self.events[self.count - 1])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloseWindow {
    pub id: WindowId,
}

#[cfg(test)]
mod tests {
    use crate::Events;

    #[derive(Debug, PartialEq)]
    struct TestEvent {}

    #[test]
    fn test_iterator() {
        let mut events = Events::<TestEvent>::default();
        events.add(TestEvent{});

        assert_eq!(Some(TestEvent{}), events.next());
        assert_eq!(None, events.next());
    }
}

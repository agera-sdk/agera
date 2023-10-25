use std::sync::{Arc, RwLock};
use crate::common::*;

type EventListenerList<T> = Arc<RwLock<Vec<EventListener<T>>>>;

/// An event emitter.
///
/// `EventEmitter` consists of a sequence of listeners whose function is invoked
/// when an event is emitted with a single data value.
/// 
/// Event listeners to an event emitter are created through the `listener` method.
pub struct EventEmitter<T: Clone> {
    listener_list: EventListenerList<T>,
}

impl<T: Clone> EventEmitter<T> {
    pub fn new() -> Self {
        Self {
            listener_list: Arc::new(RwLock::new(vec![])),
        }
    }

    /// Adds a listener to an event emitter.
    pub fn listener<F>(&self, function: F) -> EventListener<T>
        where F: Fn(T) + Send + Sync + 'static
    {
        let listener = EventListener::new(Arc::clone(&self.listener_list), function);
        listener.add();
        listener
    }

    /// Emits a single data value.
    pub fn emit(&self, data: T) {
        let mut list_2 = vec![];
        for listener in &*self.listener_list.read().unwrap() {
            list_2.push(listener.clone());
        }
        for listener in list_2 {
            (listener.inner.function)(data.clone());
        }
    }
}

/*
/// Creates an event listener to an event emitter, returning `EventListener<T>`.
/// 
/// # Syntax
/// 
/// ```ignore
/// use agera::common::*;
///
/// let listener = event_listener!(event_emitter, |data| {
///     // Action
/// });
/// ```
pub macro event_listener {
    ($emitter:expr, $function:expr) => {
        let emitter: EventEmitter<_> = $emitter;
        emitter.add_listener(Box::new($function))
    },
}
*/

pub struct EventListener<T: Clone> {
    inner: Arc<EventListenerInner<T>>,
}

impl<T: Clone> PartialEq for EventListener<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T: Clone> Eq for EventListener<T> {}

impl<T: Clone> Clone for EventListener<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Clone> EventListener<T> {
    pub fn new<F: Fn(T) + Send + Sync + 'static>(listener_list: EventListenerList<T>, function: F) -> Self {
        Self {
            inner: Arc::new(EventListenerInner {
                listener_list,
                function: Box::new(function),
            }),
        }
    }

    /// Adds the event listener to the end of the sequence of listeners if it was previously
    /// removed by the `remove` method. If the event listener is already attached
    /// to the sequence, it is moved to the end of the sequence.
    pub fn add(&self) {
        self.remove();
        let list = &self.inner.listener_list;
        list.write().unwrap().push(self.clone());
    }

    /// Indicates whether the event listener is attached to the sequence of listeners,
    /// that is, whether it was not removed from the sequence.
    pub fn is_active(&self) -> bool {
        let list = &self.inner.listener_list;
        list.read().unwrap().contains(self)
    }

    /// Removes the event listener from the sequence of listeners.
    pub fn remove(&self) {
        let list = &self.inner.listener_list;
        list.write().unwrap().remove_equals(self);
    }
}

struct EventListenerInner<T: Clone> {
    listener_list: EventListenerList<T>,
    function: Box<dyn Fn(T) + Send + Sync + 'static>,
}
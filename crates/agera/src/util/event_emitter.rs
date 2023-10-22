use std::sync::{Arc, RwLock};

pub type EventListenerList<T> = Arc<RwLock<Vec<Arc<EventListener<T>>>>>;
pub type EventListenerFunction<T> = Box<dyn Fn(T) + Send + Sync + 'static>;

/// An event emitter.
///
/// `EventEmitter` consists of a sequence of listeners whose function is invoked
/// when an event is emitted with a single data value.
/// 
/// Event listeners to an event emitter are created through the [`event_listener!`] macro.
pub struct EventEmitter<T: Clone> {
    listener_list: EventListenerList<T>,
}

impl<T: Clone> EventEmitter<T> {
    pub fn new() -> Self {
        Self {
            listener_list: Arc::new(RwLock::new(vec![])),
        }
    }

    /// The sequence of listeners attached to this event emitter.
    pub fn listener_seq(&self) -> EventListenerList<T> {
        Arc::clone(&self.listener_list)
    }

    #[doc(hidden)]
    pub fn add_listener(&self, function: EventListenerFunction<T>) -> Arc<EventListener<T>> {
        let listener = EventListener::new(self.listener_seq(), function);
        listener.add();
        listener
    }

    /// Emits a single data value.
    pub fn emit(&self, data: T) {
        let mut list_2 = vec![];
        for listener in &*self.listener_list.read().unwrap() {
            list_2.push(Arc::clone(listener));
        }
        for listener in list_2 {
            (listener.function)(data.clone());
        }
    }
}

/// Creates an event listener to an event emitter, returning `Arc<EventListener<T>>`.
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

pub struct EventListener<T: Clone> {
    listener_list: EventListenerList<T>,
    function: EventListenerFunction<T>,
}

impl<T: Clone> EventListener<T> {
    pub fn new(listener_list: EventListenerList<T>, function: EventListenerFunction<T>) -> Arc<Self> {
        Arc::new(Self {
            listener_list,
            function,
        })
    }

    /// Adds the event listener to the sequence of listeners.
    /// This method is called implicitly by the [`event_listener!`] macro.
    pub fn add(self: &Arc<Self>) {
        self.remove();
        let list = &self.listener_list;
        list.write().unwrap().push(Arc::clone(self));
    }

    /// Removes the event listener from the sequence of listeners.
    pub fn remove(self: &Arc<Self>) {
        let list = &self.listener_list;
        let list_len = list.read().unwrap().len();
        for i in 0..list_len {
            if Arc::ptr_eq(&list.read().unwrap()[i], self) {
                list.write().unwrap().remove(i);
                break;
            }
        }
    }
}
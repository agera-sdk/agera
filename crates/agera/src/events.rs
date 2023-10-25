/*!
Work with events.

Event conditions are represented by `EventEmitter` objects, to which
listeners can be attached through their `.listener` method, which returns
an `EventListener` object that can be removed from and added back to the set
of listeners.

```
button.on_click().listener(|e| {
    // Action
});
```
*/

mod event_emitter;
pub use self::event_emitter::*;
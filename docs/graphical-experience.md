# Graphical experience

## Custom controls

Creating a custom control requires implementing `AgeraControlDelegate`, thus inheriting various common control methods:

```rust
use agera::controls::*;

#[derive(Clone, Eq, PartialEq, Hash)]
struct CustomButton(Button);

impl CustomButton {
    fn new() -> Self {
        Self(Button::new())
    }
}

impl AgeraControlDelegate for CustomButton {
    fn delegate(&self) -> Control {
        self.0.into()
    }
}
```
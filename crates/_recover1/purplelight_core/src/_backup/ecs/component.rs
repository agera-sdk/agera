pub trait Component {}

#[derive(Clone)]
pub struct Name(pub String);

impl Name {
    pub fn new(name: &str) -> Self {
        Name(name.into())
    }
}
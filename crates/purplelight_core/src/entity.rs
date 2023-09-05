/// Lightweight identifier of an entity.
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity(pub(crate) u64);

impl Entity {
    /// An entity identifier with a placeholder value.
    pub const PLACEHOLDER: Entity = Entity(0);

    pub const fn from_int(int: u64) -> Entity {
        Self(int)
    }

    pub const fn to_int(self) -> u64 {
        self.0
    }
}
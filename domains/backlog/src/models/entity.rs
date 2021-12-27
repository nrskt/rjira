use uuid::Uuid;

/// It has id that represented identifier.
pub trait Entity {
    /// return identifier.
    fn id(&self) -> Uuid;
}

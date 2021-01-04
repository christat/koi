/// Manual variant of Drop trait. Use only when it is crucial to explicitly handle drops in a very specific order.
pub trait Destroy {
    fn destroy(&mut self);
}
//----------------------------------------------------------------------------------------------------------------------

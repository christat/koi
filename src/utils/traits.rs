/// Manual variant of Drop trait. Required when explicit drops need to be handled from nested structures
/// and drop order is crucial.
pub trait Cleanup {
    fn cleanup(&mut self);
}
//----------------------------------------------------------------------------------------------------------------------

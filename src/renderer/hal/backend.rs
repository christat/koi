pub trait RendererBackend {
    fn draw(&mut self);
    fn await_device_idle(&mut self);
}
//----------------------------------------------------------------------------------------------------------------------

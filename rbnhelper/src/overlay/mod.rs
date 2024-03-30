pub mod copyright;

pub trait Overlay {
    fn init(&mut self, width: i16, height: i16);

    fn draw_ui(&mut self);
}
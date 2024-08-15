use crate::map::viewport::Viewport;

pub trait Renderer<Target> {
    fn render(&self, target: &mut Target, viewport: &Viewport);
}

use crate::map::rendering::Renderer;
use crate::map::Viewport;

pub struct MapRenderer {}

type SharedPixelBuffer = slint::SharedPixelBuffer<slint::Rgba8Pixel>;

impl Renderer<SharedPixelBuffer> for MapRenderer {
    fn render(&self, target: &mut SharedPixelBuffer, viewport: &Viewport) {}
}

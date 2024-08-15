use crate::map::rendering::Renderer;
use crate::map::Map;
use slint::SharedPixelBuffer;

use super::renderer::map_renderer::MapRenderer;

pub struct MapControl {
    map: Map,
    renderer: MapRenderer,
}

impl MapControl {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            renderer: MapRenderer {},
        }
    }

    pub fn render(&self) -> slint::Image {
        let mut pixel_buffer = SharedPixelBuffer::new(640, 480);
        self.renderer
            .render(&mut pixel_buffer, &self.map.navigator.viewport);

        slint::Image::from_rgba8(pixel_buffer)
    }
}

impl crate::map::MapControl for MapControl {
    fn common_draw_control(&self) {}
}

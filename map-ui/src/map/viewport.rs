pub struct Viewport {
    pub center_x: f64,
    pub center_y: f64,
    pub resolution: f64,
    pub rotation: f64,
    pub width: f64,
    pub height: f64,
}

impl Viewport {
    pub fn new(
        center_x: f64,
        center_y: f64,
        resolution: f64,
        rotation: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            center_x,
            center_y,
            resolution,
            rotation,
            width,
            height,
        }
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
    }
}

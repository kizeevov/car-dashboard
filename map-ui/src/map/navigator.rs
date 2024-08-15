use crate::map::viewport::Viewport;

pub struct Navigator {
    pub viewport: Viewport,
    pub pan_lock: bool,
}

impl Navigator {
    pub fn new() -> Self {
        Self {
            viewport: Viewport::default(),
            pan_lock: false,
        }
    }

    pub fn set_pan_lock(&mut self, pan_lock: bool) {
        self.pan_lock = pan_lock
    }
}

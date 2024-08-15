use crate::map::layers::{Layer, LayerBox};
use crate::map::navigator::Navigator;

pub struct Map {
    pub navigator: Navigator,
    pub layers: Vec<LayerBox>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            navigator: Navigator::new(),
            layers: vec![],
        }
    }
}

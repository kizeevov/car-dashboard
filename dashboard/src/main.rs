#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod player;
mod ui;

use crate::ui::MainWindow;
use slint::ComponentHandle;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let window = MainWindow::new().unwrap();
    window.run().unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
mod map;
mod ui;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!( $( $t )* );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    #[cfg(target_arch = "wasm32")]
    web::wasm_main();

    #[cfg(not(target_arch = "wasm32"))]
    main_with_tokio();
}

#[cfg(not(target_arch = "wasm32"))]
fn main_with_tokio() {
    use crate::ui::MainWindow;
    use slint::ComponentHandle;

    let rt = tokio::runtime::Runtime::new().unwrap();
    let _tokio = rt.enter();

    let window = MainWindow::new().unwrap();

    let _map = map::setup(&window);

    // let _timer = ui::setup_timer(&window);
    // let ui_join = ui::setup(&window);

    window.run().unwrap();
    // ui_join.join().unwrap();
}

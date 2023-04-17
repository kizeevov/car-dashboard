#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
// #[cfg(target_arch = "wasm32")]
// use web_sys::Event;

mod player;
mod ui;

use crate::ui::{MainWindow, MainWindowAdapter};
use slint::{ComponentHandle, Weak};

#[cfg(not(target_arch = "wasm32"))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!( $( $t )* );
    }
}

#[cfg(target_arch = "wasm32")]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let window = MainWindow::new().unwrap();

    #[cfg(target_arch = "wasm32")]
    resize(window.as_weak());

    window.run().unwrap();
}

#[cfg(target_arch = "wasm32")]
fn resize(window_weak: Weak<MainWindow>) {
    let window = web_sys::window().expect("no global `window` exists");

    // let window_weak_1 = window_weak.clone();
    // let cb = Closure::wrap(Box::new(move |_e: Event| {
    //     // let input = e
    //     //     .current_target()
    //     //     .unwrap()
    //     //     .dyn_into::<web_sys::HtmlTextAreaElement>()
    //     //     .unwrap();
    //     let window = web_sys::window().expect("no global `window` exists");

    //     let height = window.inner_height().unwrap().as_f64().unwrap();
    //     let width = window.inner_width().unwrap().as_f64().unwrap();

    //     log!("{width} {height}");

    //     window_weak_1
    //         .upgrade_in_event_loop(move |window| {
    //             window
    //                 .global::<MainWindowAdapter>()
    //                 .set_height(height as f32);
    //             window.global::<MainWindowAdapter>().set_width(width as f32)
    //         })
    //         .unwrap();

    //     log!("resize");
    // }) as Box<dyn FnMut(_)>);

    // window
    //     .add_event_listener_with_callback("resize", &cb.as_ref().unchecked_ref())
    //     .unwrap();

    // cb.forget();

    let height = window.inner_height().unwrap().as_f64().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap();

    log!("{width} {height}");

    window_weak
        .upgrade_in_event_loop(move |window| {
            window
                .global::<MainWindowAdapter>()
                .set_height(height as f32);
            window.global::<MainWindowAdapter>().set_width(width as f32)
        })
        .unwrap();
}

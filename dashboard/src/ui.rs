slint::include_modules!();

use std::thread;
use slint::{Timer, Weak};
use crate::player;

pub fn setup_timer(window: &MainWindow) -> Timer {
    let update_timer = Timer::default();
    update_timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(300), {
        let weak_window = window.as_weak();

        move || {
            update(&weak_window);
        }
    });

    update_timer
}

fn update(window_weak: &Weak<MainWindow>) {
    // println!("player_worker_loop");
}

pub fn setup(window: &MainWindow) -> thread::JoinHandle<()> {
    player::setup(window)
}

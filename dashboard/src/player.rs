use crate::ui::MainWindow;
use slint::{ComponentHandle, Weak};
use std::thread;
use std::time::Duration;

pub fn setup(window: &MainWindow) -> thread::JoinHandle<()> {
    let window_weak = window.as_weak();

    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(player_worker_loop(window_weak))
    })
}

async fn player_worker_loop(_window_weak: Weak<MainWindow>) {
    // println!("player_worker_loop");
    tokio::time::sleep(Duration::from_secs(5)).await;
}

pub trait Player {}

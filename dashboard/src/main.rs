mod player;
mod ui;

use crate::ui::MainWindow;
use slint::ComponentHandle;

fn main() {
    let window = MainWindow::new().unwrap();
    window.run().unwrap();
}

use crate::main_window::MainWindow;

mod gui;
mod main_window;

fn main() {
    let main_window = MainWindow::new();
    main_window.run();
}

use crate::main_window::MainWindow;

mod gui;
mod main_window;

fn main() -> Result<(), String> {
    let mut main_window = MainWindow::new()?;
    main_window.run()?;
    Ok(())
}

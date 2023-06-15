// Imports ui.slint via the build script(?)
slint::include_modules!();

fn main() {
    // Draw the main window
    MainWindow::new().unwrap().run().unwrap();
}

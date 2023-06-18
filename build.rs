use slint_build::CompilerConfiguration;

fn main() {
    let config : CompilerConfiguration =
        slint_build::CompilerConfiguration::new()
        // Change this to "material" to switch to Material You buttons
        .with_style("fluent".to_string());
    // Compiles the project including src/ui.slint
    slint_build::compile_with_config("ui/MainWindow.slint", config).unwrap();
}
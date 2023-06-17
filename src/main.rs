// use slint::ComponentHandle;

// Imports MainWindow.slint via the build script(?)
slint::include_modules!();
// Stupid, but necessary, hack to do multi-window Slint
// https://github.com/slint-ui/slint/issues/784
mod settings;

// You can have 2, but not 3, Slint windows. Credits will be shown as a message box.

// Define behavior for main buttons
macro_rules! main_button_callbacks {($main_win:expr, $sett_win:expr)=>{
    $main_win.on_run_game(|| {
        // Launch Bullet Heck
        std::process::Command::new("Bullet Heck/BulletHeck.exe")
            // TODO enable these for release
            // .stdout(process::Stdio::null())
            // .stderr(process::Stdio::null())
            .current_dir("Bullet Heck")
            .spawn()
            .expect("Bullet Heck didn't spawn, probably because it ain't there");
        // Skedaddle
        // TODO: probably unsafe, might break settings file?
        std::process::exit(0);
    });

    $main_win.on_open_settings(move|| {
        // Show settings window
        $sett_win.show().unwrap();
    });

    $main_win.on_open_credits(|| {
        // Show a credits message box
        let content = "Sound effects obtained from www.zapsplat.com\n\
            Explosion graphics created at the now-defunct www.explosiongenerator.com\n\
            Font is \"Uno Estado\" by Dan Zadorozny (Iconian Fonts)\n\n\
            \"Space Fighter Loop\"\n\
            Kevin MacLeod (incompetech.com)\n\
            Licensed under Creative Commons: By Attribution 3.0 License\n\
            http://creativecommons.org/licenses/by/3.0/\n\n\
            All other assets were created by me, Austin Slaughter.";
        msgbox::create("Credits", content, msgbox::IconType::Info).unwrap();
    });
}}

macro_rules! sett_button_callbacks {($sett_win:expr) => {
    $sett_win.on_res_changed(|value| {
        println!("Resolution selected: {}", value);
    });

    $sett_win.on_checkbox_changed(|name, value| {
        println!("{} changed to {}", name, value);
    });

    let handle = $sett_win.as_weak();

    $sett_win.on_save(|| {
        println!("Save !");
        // TODO need a creative way to call hide()
    });

    $sett_win.on_reset(|| {
        println!("Reset !!");
    });
    
    $sett_win.on_cancel(|| {
        println!("Cancel !!!");
    });
};}

fn main() -> Result<(), slint::PlatformError> {
    // Set up MainWindow according to official template
    let main_win: MainWindow = MainWindow::new()?;
    // let _mw_handle = mw.as_weak();

    // Loads the settings window's Slint code, then sets it up
    settings::gen_settings_window!();
    let sett_win = SettingsWindow::new()?;

    // Called first because sett_win gets moved later
    sett_button_callbacks!(sett_win);

    main_button_callbacks!(main_win, sett_win);

    // Spawns the main window
    main_win.run()
}

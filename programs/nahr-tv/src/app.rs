use crate::models::MainWindow;
use slint::ComponentHandle;
use std::sync::{Arc, Mutex};

pub fn run() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // 1. Exit & Back to Desktop controller
    main_window.on_back_clicked(move || {
        println!("Back button clicked. Returning to Desktop...");
        std::process::exit(0);
    });

    // 2. Bluetooth click controller (Delegates to modular simulator thread)
    let weak_bt = main_window.as_weak();
    main_window.on_bluetooth_device_clicked(move |dev| {
        crate::bluetooth::handle_bluetooth_click(weak_bt.clone(), dev.to_string());
    });

    // 3. Audio navigation key-click sound player
    main_window.on_play_navigation_sound(move || {
        crate::audio::play_navigation_sound();
    });

    // 4. Central app launcher matching process spawner
    main_window.on_app_clicked(move |app_id| {
        crate::launcher::launch_app(&app_id);
    });

    // 5. Unification of volume and brightness popup hide timers
    let weak_vol = main_window.as_weak();
    let vol_counter = Arc::new(Mutex::new(0u32));
    main_window.on_trigger_volume_timer(move || {
        crate::timers::start_hide_timer(
            weak_vol.clone(),
            vol_counter.clone(),
            MainWindow::set_show_volume_bar,
        );
    });

    let weak_bright = main_window.as_weak();
    let bright_counter = Arc::new(Mutex::new(0u32));
    main_window.on_trigger_brightness_timer(move || {
        crate::timers::start_hide_timer(
            weak_bright.clone(),
            bright_counter.clone(),
            MainWindow::set_show_brightness_bar,
        );
    });

    // 6. Schedules clock synchronization every 15 seconds
    let weak_clock = main_window.as_weak();
    let _time_timer = crate::clock::start_clock_sync(weak_clock);

    main_window.run()
}

use std::sync::{Arc, Mutex};
use slint::{ComponentHandle, Model};
use crate::models::{AppWindow, AppInfo};
use crate::store::refresh_apps_list;
use crate::handlers::*;

pub fn run() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    
    // Store the thread-safe master copy of all applications
    let all_apps = Arc::new(Mutex::new(
        app.get_apps().iter().collect::<Vec<AppInfo>>()
    ));
    
    // Initialize view immediately to show only Sumer Apps on Tab 0
    refresh_apps_list(&app, &all_apps.lock().unwrap());
    println!("[+] Zaqura App Store initialized successfully with {} total applications.", all_apps.lock().unwrap().len());
    
    // 0. TAB CHANGE CALLBACK
    let app_weak = app.as_weak();
    let all_apps_clone = all_apps.clone();
    app.on_tab_changed(move |idx| {
        handle_tab_changed(app_weak.clone(), all_apps_clone.clone(), idx);
    });
    
    // 0.5. SEARCH CHANGE CALLBACK
    let app_weak = app.as_weak();
    let all_apps_clone = all_apps.clone();
    app.on_search_changed(move |query| {
        handle_search_changed(app_weak.clone(), all_apps_clone.clone(), query);
    });
    
    // 1. LAUNCH APP CALLBACK
    let app_weak = app.as_weak();
    app.on_launch_app(move |app_id| {
        handle_launch_app(app_weak.clone(), app_id);
    });
    
    // 2. INSTALL APP CALLBACK
    let app_weak = app.as_weak();
    let all_apps_clone = all_apps.clone();
    app.on_install_app(move |idx| {
        handle_install_app(app_weak.clone(), all_apps_clone.clone(), idx);
    });
    
    // 3. UPDATE APP CALLBACK
    let app_weak = app.as_weak();
    let all_apps_clone = all_apps.clone();
    app.on_update_app(move |idx| {
        handle_update_app(app_weak.clone(), all_apps_clone.clone(), idx);
    });
    
    // 4. UPDATE ALL APPS CALLBACK
    let app_weak = app.as_weak();
    let all_apps_clone = all_apps.clone();
    app.on_update_all_apps(move || {
        handle_update_all_apps(app_weak.clone(), all_apps_clone.clone());
    });
    
    // 5. TOGGLE ADDON CALLBACK
    let app_weak = app.as_weak();
    app.on_toggle_addon(move |idx| {
        handle_toggle_addon(app_weak.clone(), idx);
    });
    
    app.run()
}

use std::rc::Rc;
use crate::models::{AppWindow, AppInfo};

// Dynamically refresh apps model shown in Slint based on active tab and master list
pub fn refresh_apps_list(app: &AppWindow, all_apps: &[AppInfo]) {
    let active_tab = app.get_active_tab();
    let search_query = app.get_search_query().to_string().to_lowercase();
    
    let filtered: Vec<AppInfo> = all_apps
        .iter()
        .filter(|a| {
            let tab_match = if active_tab == 0 {
                a.is_sumer // Sumer programs
            } else if active_tab == 1 {
                !a.is_sumer // General programs
            } else {
                true // Keep all apps for Updates view or details
            };
            
            let search_match = if search_query.is_empty() {
                true
            } else {
                a.name.to_string().to_lowercase().contains(&search_query) ||
                a.description.to_string().to_lowercase().contains(&search_query)
            };
            
            tab_match && search_match
        })
        .cloned()
        .collect();
    
    let model = Rc::new(slint::VecModel::from(filtered));
    app.set_apps(model.into());
}

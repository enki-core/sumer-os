slint::include_modules!();

use slint::{VecModel, SharedString};
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    title: String,
    description: String,
    priority: String,   // "عالية", "متوسطة", "منخفضة"
    category: String,   // "العمل", "شخصية", "الدراسة", "التسوق", "الصحة", "أخرى"
    due_date: String,
    completed: bool,
    created_at: String,
}

impl From<&Task> for SlintTask {
    fn from(t: &Task) -> Self {
        Self {
            id: SharedString::from(&t.id),
            title: SharedString::from(&t.title),
            description: SharedString::from(&t.description),
            priority: SharedString::from(&t.priority),
            category: SharedString::from(&t.category),
            due_date: SharedString::from(&t.due_date),
            completed: t.completed,
            created_at: SharedString::from(&t.created_at),
        }
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    file_path: String,
}

impl TaskManager {
    fn new() -> Self {
        let file_path = "tasks.json".to_string();
        let mut manager = Self {
            tasks: Vec::new(),
            file_path,
        };
        manager.load();
        manager
    }

    fn load(&mut self) {
        if Path::new(&self.file_path).exists() {
            if let Ok(mut file) = File::open(&self.file_path) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&content) {
                        self.tasks = tasks;
                        return;
                    }
                }
            }
        }
        self.tasks = Vec::new();
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn export_to_txt(&self) -> Result<String, std::io::Error> {
        // Export to both local and Sumer Desktop for the user's immediate convenience
        let desktop_path = "/home/debian/Desktop/tasks_report.txt";
        let local_path = "tasks_report.txt";
        
        let mut content = String::new();
        content.push_str("==================================================\n");
        content.push_str("          تقرير المهام اليومي - إنانا للمهام       \n");
        content.push_str(&format!("          تاريخ التصدير: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        content.push_str("==================================================\n\n");
        
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.completed).count();
        let pending = total - completed;
        let rate = if total > 0 { (completed as f32 / total as f32) * 100.0 } else { 0.0 };
        
        content.push_str("إحصائيات المهام الكلية:\n");
        content.push_str(&format!("  - إجمالي المهام: {}\n", total));
        content.push_str(&format!("  - المهام المكتملة: {}\n", completed));
        content.push_str(&format!("  - المهام المعلقة: {}\n", pending));
        content.push_str(&format!("  - نسبة الإنجاز: {:.1}%\n\n", rate));
        
        content.push_str("--------------------------------------------------\n");
        content.push_str("قائمة المهام:\n");
        content.push_str("--------------------------------------------------\n");
        
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "[✓] مكتملة" } else { "[ ] معلقة" };
            content.push_str(&format!("{}. {} (الأولوية: {} | التصنيف: {})\n", i + 1, task.title, task.priority, task.category));
            if !task.description.is_empty() {
                content.push_str(&format!("   التفاصيل: {}\n", task.description));
            }
            if !task.due_date.is_empty() {
                content.push_str(&format!("   تاريخ الاستحقاق: {}\n", task.due_date));
            }
            content.push_str(&format!("   الحالة: {}\n", status));
            content.push_str("--------------------------------------------------\n");
        }
        
        // Try writing to Desktop first
        let write_result = File::create(desktop_path).and_then(|mut f| f.write_all(content.as_bytes()));
        
        if write_result.is_ok() {
            // Also write a copy locally
            if let Ok(mut f_local) = File::create(local_path) {
                let _ = f_local.write_all(content.as_bytes());
            }
            Ok(desktop_path.to_string())
        } else {
            // If desktop is read-only or doesn't exist, write locally and return that
            let mut f_local = File::create(local_path)?;
            f_local.write_all(content.as_bytes())?;
            Ok(local_path.to_string())
        }
    }
}

fn get_filtered_tasks(
    tasks: &[Task],
    category: &str,
    status: &str,
    query: &str,
    sort_by: &str,
) -> Vec<Task> {
    let mut filtered: Vec<Task> = tasks
        .iter()
        .filter(|t| {
            // Category filter
            if category != "الكل" && t.category != category {
                return false;
            }
            // Status filter
            if status != "الكل" {
                let is_completed = status == "المكتملة";
                if t.completed != is_completed {
                    return false;
                }
            }
            // Search query filter
            if !query.is_empty() {
                let q = query.to_lowercase();
                let title_matches = t.title.to_lowercase().contains(&q);
                let desc_matches = t.description.to_lowercase().contains(&q);
                if !title_matches && !desc_matches {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // Sort tasks
    if sort_by == "الأولوية" {
        filtered.sort_by(|a, b| {
            let priority_val = |p: &str| match p {
                "عالية" => 3,
                "متوسطة" => 2,
                "منخفضة" => 1,
                _ => 0,
            };
            priority_val(&b.priority).cmp(&priority_val(&a.priority))
        });
    } else {
        // Sort by due date (empty due dates last)
        filtered.sort_by(|a, b| {
            if a.due_date.is_empty() && b.due_date.is_empty() {
                std::cmp::Ordering::Equal
            } else if a.due_date.is_empty() {
                std::cmp::Ordering::Greater
            } else if b.due_date.is_empty() {
                std::cmp::Ordering::Less
            } else {
                a.due_date.cmp(&b.due_date)
            }
        });
    }

    filtered
}

fn update_slint_state(ui: &AppWindow, manager: &TaskManager) {
    let total = manager.tasks.len() as i32;
    let completed = manager.tasks.iter().filter(|t| t.completed).count() as i32;
    let pending = total - completed;
    let rate = if total > 0 { completed as f32 / total as f32 } else { 0.0 };

    ui.set_stat_total(total);
    ui.set_stat_completed(completed);
    ui.set_stat_pending(pending);
    ui.set_completion_rate(rate);

    // Get current filters from Slint UI
    let category = ui.get_filter_category();
    let status = ui.get_filter_status();
    let query = ui.get_search_query();
    let sort_by = ui.get_sort_by();

    let filtered = get_filtered_tasks(&manager.tasks, &category, &status, &query, &sort_by);
    let slint_tasks: Vec<SlintTask> = filtered.iter().map(SlintTask::from).collect();
    let model = Rc::new(VecModel::from(slint_tasks));
    ui.set_tasks(model.into());
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let ui_handle = app.as_weak();
    
    // Core state manager wrapped in RefCell for shared mutable borrowing in callbacks
    let manager_rc = Rc::new(RefCell::new(TaskManager::new()));

    // 1. Trigger initial dashboard and list state
    if let Some(ui) = ui_handle.upgrade() {
        update_slint_state(&ui, &manager_rc.borrow());
    }

    // 2. Add or Update Task Callback
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_add_or_update_task(move |id, title, desc, prio_idx, cat_idx, due_date| {
        let ui = ui_weak.upgrade().unwrap();
        let mut m = manager.borrow_mut();

        let id_str = id.as_str().to_string();
        let title_str = title.as_str().trim().to_string();
        let desc_str = desc.as_str().trim().to_string();
        let due_str = due_date.as_str().trim().to_string();

        let prio_str = match prio_idx {
            0 => "منخفضة".to_string(),
            1 => "متوسطة".to_string(),
            2 => "عالية".to_string(),
            _ => "متوسطة".to_string(),
        };

        let cat_str = match cat_idx {
            0 => "العمل".to_string(),
            1 => "شخصية".to_string(),
            2 => "الدراسة".to_string(),
            3 => "التسوق".to_string(),
            4 => "الصحة".to_string(),
            5 => "أخرى".to_string(),
            _ => "العمل".to_string(),
        };

        if id_str.is_empty() {
            // Mode: ADD Task
            let new_id = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string();
            
            let new_task = Task {
                id: new_id,
                title: title_str,
                description: desc_str,
                priority: prio_str,
                category: cat_str,
                due_date: due_str,
                completed: false,
                created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            };
            m.tasks.push(new_task);
            ui.set_status_message("تم إضافة المهمة الجديدة بنجاح!".into());
        } else {
            // Mode: UPDATE Existing Task
            if let Some(task) = m.tasks.iter_mut().find(|t| t.id == id_str) {
                task.title = title_str;
                task.description = desc_str;
                task.priority = prio_str;
                task.category = cat_str;
                task.due_date = due_str;
                ui.set_status_message("تم تحديث بيانات المهمة بنجاح!".into());
            }
        }

        let _ = m.save();
        update_slint_state(&ui, &m);
    });

    // 3. Delete Task Callback
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_delete_task(move |id| {
        let ui = ui_weak.upgrade().unwrap();
        let mut m = manager.borrow_mut();
        let id_str = id.as_str().to_string();

        m.tasks.retain(|t| t.id != id_str);
        
        // If we were editing this deleted task, reset the form state
        if ui.get_editing_id().as_str() == id_str {
            ui.set_editing_id("".into());
            ui.set_current_title("".into());
            ui.set_current_desc("".into());
            ui.set_current_due_date("".into());
            ui.set_selected_priority_index(1);
        }

        let _ = m.save();
        ui.set_status_message("تم حذف المهمة نهائياً.".into());
        update_slint_state(&ui, &m);
    });

    // 4. Toggle Task Completion Status Callback
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_toggle_complete(move |id| {
        let ui = ui_weak.upgrade().unwrap();
        let mut m = manager.borrow_mut();
        let id_str = id.as_str().to_string();

        if let Some(task) = m.tasks.iter_mut().find(|t| t.id == id_str) {
            task.completed = !task.completed;
            if task.completed {
                ui.set_status_message("رائع! تم إنجاز المهمة بنجاح 🎉".into());
            } else {
                ui.set_status_message("تمت إعادة المهمة إلى قائمة المعلقة.".into());
            }
        }

        let _ = m.save();
        update_slint_state(&ui, &m);
    });

    // 5. Request Edit (Load task data into sidebar fields)
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_request_edit(move |id| {
        let ui = ui_weak.upgrade().unwrap();
        let m = manager.borrow();
        let id_str = id.as_str().to_string();

        if let Some(task) = m.tasks.iter().find(|t| t.id == id_str) {
            ui.set_editing_id(task.id.clone().into());
            ui.set_current_title(task.title.clone().into());
            ui.set_current_desc(task.description.clone().into());
            ui.set_current_due_date(task.due_date.clone().into());
            
            // Map priority to index
            let prio_idx = match task.priority.as_str() {
                "منخفضة" => 0,
                "متوسطة" => 1,
                "عالية" => 2,
                _ => 1,
            };
            ui.set_selected_priority_index(prio_idx);

            // Map category to index
            let cat_idx = match task.category.as_str() {
                "العمل" => 0,
                "شخصية" => 1,
                "الدراسة" => 2,
                "التسوق" => 3,
                "الصحة" => 4,
                "أخرى" => 5,
                _ => 0,
            };
            ui.set_selected_category_index(cat_idx);

            ui.set_status_message("تم جلب بيانات المهمة للتعديل.".into());
        }
    });

    // 6. Clear All Tasks Callback
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_clear_all_tasks(move || {
        let ui = ui_weak.upgrade().unwrap();
        let mut m = manager.borrow_mut();
        
        m.tasks.clear();
        ui.set_editing_id("".into());
        ui.set_current_title("".into());
        ui.set_current_desc("".into());
        ui.set_current_due_date("".into());

        let _ = m.save();
        ui.set_status_message("تم مسح كافة المهام بالكامل!".into());
        update_slint_state(&ui, &m);
    });

    // 7. Apply Filters Callback (Search, status, category, sorting)
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_apply_filters(move |cat, status, search, sort| {
        let ui = ui_weak.upgrade().unwrap();
        let m = manager.borrow();

        let filtered = get_filtered_tasks(
            &m.tasks,
            cat.as_str(),
            status.as_str(),
            search.as_str(),
            sort.as_str()
        );
        let slint_tasks: Vec<SlintTask> = filtered.iter().map(SlintTask::from).collect();
        let model = Rc::new(VecModel::from(slint_tasks));
        ui.set_tasks(model.into());
    });

    // 8. Export to Text Report Callback
    let ui_weak = ui_handle.clone();
    let manager = manager_rc.clone();
    app.on_export_to_txt(move || {
        let ui = ui_weak.upgrade().unwrap();
        let m = manager.borrow();

        match m.export_to_txt() {
            Ok(path) => {
                let msg = format!("تم تصدير التقرير لملف نصي بنجاح: {}", path);
                ui.set_status_message(msg.into());
                ui.set_is_status_error(false);
            }
            Err(e) => {
                let msg = format!("خطأ أثناء التصدير: {}", e);
                ui.set_status_message(msg.into());
                ui.set_is_status_error(true);
            }
        }
    });

    app.run()
}

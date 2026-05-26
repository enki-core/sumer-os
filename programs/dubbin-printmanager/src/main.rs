slint::include_modules!();

use slint::{Color, ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    // Initial printers list
    let initial_printers = vec![
        Printer {
            id: "p1".into(),
            name: "Sumer Jet Office 5000".into(),
            port: "USB001".into(),
            driver: "Sumer Universal InkJet".into(),
            status: "جاهز".into(),
            status_color: Color::from_rgb_u8(16, 185, 129),
            is_default: true,
            ink_c: 0.85,
            ink_m: 0.65,
            ink_y: 0.40,
            ink_k: 0.90,
            jobs_count: 0
        },
        Printer {
            id: "p2".into(),
            name: "Basra Laser Pro 80X".into(),
            port: "192.168.1.120:9100".into(),
            driver: "Basra Laser Mono PCL6".into(),
            status: "مشغول".into(),
            status_color: Color::from_rgb_u8(245, 158, 11),
            is_default: false,
            ink_c: 0.0,
            ink_m: 0.0,
            ink_y: 0.0,
            ink_k: 0.15,
            jobs_count: 2
        },
        Printer {
            id: "p3".into(),
            name: "Lagash Color Multi".into(),
            port: "192.168.1.155:9100".into(),
            driver: "Lagash PostScript Color".into(),
            status: "غير متصل".into(),
            status_color: Color::from_rgb_u8(239, 68, 68),
            is_default: false,
            ink_c: 0.45,
            ink_m: 0.12,
            ink_y: 0.80,
            ink_k: 0.35,
            jobs_count: 0
        }
    ];

    // Initial print queue
    let initial_jobs = vec![
        PrintJob {
            id: 101,
            printer_name: "Basra Laser Pro 80X".into(),
            doc_name: "تقرير الميزانية السنوية_٢٠٢٦.pdf".into(),
            owner: "debian".into(),
            pages: 24,
            size_formatted: "4.2 MB".into(),
            status: "يتم الطبع الآن".into(),
            status_color: Color::from_rgb_u8(59, 130, 246),
            progress: 0.45,
            submitted_time: "16:20".into(),
            is_pdf: true,
        },
        PrintJob {
            id: 102,
            printer_name: "Basra Laser Pro 80X".into(),
            doc_name: "مذكرة_عمليات_السيرفر.txt".into(),
            owner: "system".into(),
            pages: 2,
            size_formatted: "128 KB".into(),
            status: "قيد الانتظار".into(),
            status_color: Color::from_rgb_u8(245, 158, 11),
            progress: 0.0,
            submitted_time: "16:31".into(),
            is_pdf: false,
        }
    ];

    // Initial logs
    let initial_logs = vec![
        "[16:01] تم ربط الطابعة Sumer Jet Office 5000 بنجاح عبر منفذ USB001.".into(),
        "[16:15] تم الكشف عن طابعة الشبكة Basra Laser Pro 80X وتعيين تعريف PCL6.".into(),
        "[16:20] بدأ المستخدم 'debian' مهمة الطباعة رقم 101 لحجم 4.2 ميجابايت.".into(),
        "[16:31] أضاف النظام مهمة الطباعة 'مذكرة_عمليات_السيرفر' في صف الانتظار.".into()
    ];

    let printers_model = Rc::new(VecModel::from(initial_printers));
    let jobs_model = Rc::new(VecModel::from(initial_jobs));
    let logs_model = Rc::new(VecModel::from(initial_logs));

    app.set_printers(ModelRc::from(printers_model.clone()));
    app.set_active_jobs(ModelRc::from(jobs_model.clone()));
    app.set_system_logs(ModelRc::from(logs_model.clone()));
    app.set_default_printer_name("Sumer Jet Office 5000".into());

    // Set default selected printer
    if let Some(first) = printers_model.row_data(0) {
        app.set_selected_printer(first);
        app.set_has_selected_printer(true);
    }

    // 1. Callback: select_printer
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    app.on_select_printer(move |id| {
        let app = app_weak.unwrap();
        for i in 0..printers_model_clone.row_count() {
            if let Some(p) = printers_model_clone.row_data(i) {
                if p.id == id {
                    app.set_selected_printer(p);
                    app.set_has_selected_printer(true);
                    break;
                }
            }
        }
    });

    // 2. Callback: set_default_printer
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_set_default_printer(move |id| {
        let app = app_weak.unwrap();
        let mut target_name = String::new();
        for i in 0..printers_model_clone.row_count() {
            if let Some(mut p) = printers_model_clone.row_data(i) {
                p.is_default = p.id == id;
                printers_model_clone.set_row_data(i, p.clone());
                if p.id == id {
                    target_name = p.name.to_string();
                    app.set_selected_printer(p);
                    app.set_default_printer_name(target_name.clone().into());
                }
            }
        }
        app.set_notification_msg(format!("تم تعيين '{}' كطابعة افتراضية", target_name).into());
        app.set_notification_color(Color::from_rgb_u8(16, 185, 129));
        
        let log_msg = format!("[نظام] تم تعيين '{}' كطابعة افتراضية بنجاح.", target_name);
        logs_model_clone.push(log_msg.into());
    });

    // 3. Callback: print_test_page
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let jobs_model_clone = jobs_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_print_test_page(move |id| {
        let app = app_weak.unwrap();
        let mut printer_name = String::new();
        
        // Find and update printer
        for i in 0..printers_model_clone.row_count() {
            if let Some(mut p) = printers_model_clone.row_data(i) {
                if p.id == id {
                    if p.status == "غير متصل" {
                        app.set_notification_msg("خطأ: لا يمكن الطباعة على جهاز غير متصل!".into());
                        app.set_notification_color(Color::from_rgb_u8(239, 68, 68));
                        logs_model_clone.push(format!("[خطأ] فشلت محاولة إرسال صفحة اختبار إلى الطابعة غير المتصلة '{}'.", p.name).into());
                        return;
                    }
                    
                    p.jobs_count += 1;
                    p.status = "مشغول".into();
                    p.status_color = Color::from_rgb_u8(245, 158, 11);
                    printer_name = p.name.to_string();
                    printers_model_clone.set_row_data(i, p.clone());
                    app.set_selected_printer(p);
                    break;
                }
            }
        }
        
        if printer_name.is_empty() {
            return;
        }

        // Create the print job
        let new_job_id = 100 + jobs_model_clone.row_count() as i32 + 1;
        let doc_name = format!("صفحة اختبار الطباعة - {}.pdf", new_job_id);
        let new_job = PrintJob {
            id: new_job_id,
            printer_name: printer_name.clone().into(),
            doc_name: doc_name.clone().into(),
            owner: "debian".into(),
            pages: 1,
            size_formatted: "512 KB".into(),
            status: "يتم الطبع الآن".into(),
            status_color: Color::from_rgb_u8(59, 130, 246),
            progress: 0.1,
            submitted_time: "الآن".into(),
            is_pdf: true,
        };
        
        jobs_model_clone.push(new_job);
        
        app.set_notification_msg(format!("بدء طباعة صفحة الاختبار على '{}'", printer_name).into());
        app.set_notification_color(Color::from_rgb_u8(59, 130, 246));
        
        let log_msg = format!("[طابعة] تم إرسال صفحة اختبار إلى '{}' (رقم المهمة: {}).", printer_name, new_job_id);
        logs_model_clone.push(log_msg.into());
    });

    // 4. Callback: pause_printer
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_pause_printer(move |id| {
        let app = app_weak.unwrap();
        for i in 0..printers_model_clone.row_count() {
            if let Some(mut p) = printers_model_clone.row_data(i) {
                if p.id == id {
                    if p.status == "إيقاف مؤقت" {
                        p.status = "جاهز".into();
                        p.status_color = Color::from_rgb_u8(16, 185, 129);
                        app.set_notification_msg(format!("استئناف عمل الطابعة '{}'", p.name).into());
                        app.set_notification_color(Color::from_rgb_u8(16, 185, 129));
                        logs_model_clone.push(format!("[طابعة] تم استئناف طابعة '{}'.", p.name).into());
                    } else {
                        p.status = "إيقاف مؤقت".into();
                        p.status_color = Color::from_rgb_u8(245, 158, 11);
                        app.set_notification_msg(format!("تم إيقاف الطابعة '{}' مؤقتاً", p.name).into());
                        app.set_notification_color(Color::from_rgb_u8(245, 158, 11));
                        logs_model_clone.push(format!("[طابعة] تم إيقاف طابعة '{}' مؤقتاً.", p.name).into());
                    }
                    printers_model_clone.set_row_data(i, p.clone());
                    app.set_selected_printer(p);
                    break;
                }
            }
        }
    });

    // 5. Callback: clear_printer_queue
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let jobs_model_clone = jobs_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_clear_printer_queue(move |id| {
        let app = app_weak.unwrap();
        let mut printer_name = String::new();
        
        for i in 0..printers_model_clone.row_count() {
            if let Some(mut p) = printers_model_clone.row_data(i) {
                if p.id == id {
                    p.jobs_count = 0;
                    p.status = "جاهز".into();
                    p.status_color = Color::from_rgb_u8(16, 185, 129);
                    printer_name = p.name.to_string();
                    printers_model_clone.set_row_data(i, p.clone());
                    app.set_selected_printer(p);
                    break;
                }
            }
        }
        
        if printer_name.is_empty() {
            return;
        }

        // Cancel jobs for this printer
        let mut j = 0;
        while j < jobs_model_clone.row_count() {
            if let Some(job) = jobs_model_clone.row_data(j) {
                if job.printer_name == printer_name {
                    jobs_model_clone.remove(j);
                } else {
                    j += 1;
                }
            } else {
                break;
            }
        }
        
        app.set_notification_msg(format!("تم تفريغ قائمة الانتظار لـ '{}'", printer_name).into());
        app.set_notification_color(Color::from_rgb_u8(16, 185, 129));
        logs_model_clone.push(format!("[طابعة] تم مسح صف الانتظار بالكامل لـ '{}'.", printer_name).into());
    });

    // 6. Callback: delete_printer
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_delete_printer(move |id| {
        let app = app_weak.unwrap();
        let mut target_name = String::new();
        let mut found_idx = None;
        
        for i in 0..printers_model_clone.row_count() {
            if let Some(p) = printers_model_clone.row_data(i) {
                if p.id == id {
                    target_name = p.name.to_string();
                    found_idx = Some(i);
                    break;
                }
            }
        }
        
        if let Some(idx) = found_idx {
            printers_model_clone.remove(idx);
            app.set_notification_msg(format!("تمت إزالة الطابعة '{}' بنجاح", target_name).into());
            app.set_notification_color(Color::from_rgb_u8(239, 68, 68));
            logs_model_clone.push(format!("[نظام] تمت إزالة الطابعة '{}' من النظام.", target_name).into());
            
            // Update selected printer
            if printers_model_clone.row_count() > 0 {
                let first = printers_model_clone.row_data(0).unwrap();
                app.set_selected_printer(first);
                app.set_has_selected_printer(true);
            } else {
                app.set_has_selected_printer(false);
            }
        }
    });

    // 7. Callback: cancel_job
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let jobs_model_clone = jobs_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_cancel_job(move |job_id| {
        let app = app_weak.unwrap();
        let mut found_idx = None;
        let mut doc_name = String::new();
        let mut printer_name = String::new();
        
        for i in 0..jobs_model_clone.row_count() {
            if let Some(job) = jobs_model_clone.row_data(i) {
                if job.id == job_id {
                    found_idx = Some(i);
                    doc_name = job.doc_name.to_string();
                    printer_name = job.printer_name.to_string();
                    break;
                }
            }
        }
        
        if let Some(idx) = found_idx {
            jobs_model_clone.remove(idx);
            
            // Decrement jobs on printer
            for j in 0..printers_model_clone.row_count() {
                if let Some(mut p) = printers_model_clone.row_data(j) {
                    if p.name == printer_name {
                        if p.jobs_count > 0 {
                            p.jobs_count -= 1;
                        }
                        if p.jobs_count == 0 {
                            p.status = "جاهز".into();
                            p.status_color = Color::from_rgb_u8(16, 185, 129);
                        }
                        printers_model_clone.set_row_data(j, p.clone());
                        let sel = app.get_selected_printer();
                        if sel.id == p.id {
                            app.set_selected_printer(p);
                        }
                        break;
                    }
                }
            }
            
            app.set_notification_msg(format!("إلغاء مهمة الطباعة: '{}'", doc_name).into());
            app.set_notification_color(Color::from_rgb_u8(239, 68, 68));
            logs_model_clone.push(format!("[نظام] تم إلغاء مهمة الطباعة '{}' (رقم: {}).", doc_name, job_id).into());
        }
    });

    // 8. Callback: add_new_printer
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_add_new_printer(move |name, port, driver| {
        let app = app_weak.unwrap();
        let new_id = format!("p{}", printers_model_clone.row_count() + 1);
        let new_printer = Printer {
            id: new_id.clone().into(),
            name: name.clone(),
            port: port.clone(),
            driver: driver.clone(),
            status: "جاهز".into(),
            status_color: Color::from_rgb_u8(16, 185, 129),
            is_default: false,
            ink_c: 1.0,
            ink_m: 1.0,
            ink_y: 1.0,
            ink_k: 1.0,
            jobs_count: 0
        };
        
        printers_model_clone.push(new_printer.clone());
        
        app.set_selected_printer(new_printer);
        app.set_has_selected_printer(true);
        
        app.set_notification_msg(format!("تمت إضافة الطابعة '{}' بنجاح", name).into());
        app.set_notification_color(Color::from_rgb_u8(16, 185, 129));
        logs_model_clone.push(format!("[نظام] تمت إضافة طابعة جديدة '{}' على منفذ '{}'.", name, port).into());
    });

    // 9. Callback: refresh_all
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let logs_model_clone = logs_model.clone();
    app.on_refresh_all(move || {
        let app = app_weak.unwrap();
        
        for i in 0..printers_model_clone.row_count() {
            if let Some(mut p) = printers_model_clone.row_data(i) {
                if p.status == "جاهز" || p.status == "مشغول" {
                    // deteministically slightly adjust ink levels for a dynamic feel
                    if p.ink_k > 0.05 { p.ink_k -= 0.01; }
                    if p.ink_c > 0.05 { p.ink_c -= 0.01; }
                    if p.ink_m > 0.05 { p.ink_m -= 0.02; }
                    if p.ink_y > 0.05 { p.ink_y -= 0.015; }
                    
                    printers_model_clone.set_row_data(i, p.clone());
                    
                    let sel = app.get_selected_printer();
                    if sel.id == p.id {
                        app.set_selected_printer(p);
                    }
                }
            }
        }
        
        app.set_notification_msg("تم تحديث كافة الأجهزة ومستويات الحبر".into());
        app.set_notification_color(Color::from_rgb_u8(16, 185, 129));
        logs_model_clone.push("[طريقة] تم استعلام الأجهزة ومستويات الحبر بنجاح.".into());
    });

    // ⌛ ACTIVE TIMER: Simulates real-time printing progress & queue processing
    let timer = slint::Timer::default();
    let app_weak = app.as_weak();
    let printers_model_clone = printers_model.clone();
    let jobs_model_clone = jobs_model.clone();
    let logs_model_clone = logs_model.clone();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1500), move || {
        let app = match app_weak.upgrade() {
            Some(a) => a,
            None => return,
        };
        
        let mut completed_jobs = Vec::new();

        for i in 0..jobs_model_clone.row_count() {
            if let Some(mut job) = jobs_model_clone.row_data(i) {
                if job.status == "يتم الطبع الآن" {
                    job.progress += 0.15;
                    
                    // Consume inks on the active printer
                    let p_name = job.printer_name.to_string();
                    for j in 0..printers_model_clone.row_count() {
                        if let Some(mut p) = printers_model_clone.row_data(j) {
                            if p.name == p_name {
                                if p.ink_k > 0.01 { p.ink_k -= 0.005; }
                                if job.is_pdf {
                                    if p.ink_c > 0.01 { p.ink_c -= 0.003; }
                                    if p.ink_m > 0.01 { p.ink_m -= 0.003; }
                                    if p.ink_y > 0.01 { p.ink_y -= 0.003; }
                                }
                                printers_model_clone.set_row_data(j, p.clone());
                                let sel = app.get_selected_printer();
                                if sel.id == p.id {
                                    app.set_selected_printer(p);
                                }
                                break;
                            }
                        }
                    }

                    if job.progress >= 1.0 {
                        job.progress = 1.0;
                        job.status = "مكتمل".into();
                        job.status_color = Color::from_rgb_u8(16, 185, 129);
                        completed_jobs.push((job.id, job.printer_name.to_string(), job.doc_name.to_string()));
                    }
                    jobs_model_clone.set_row_data(i, job);
                    break; // Only process one job in progress at a time
                }
            }
        }
        
        // Start next job if there is a pending one and no active jobs are printing
        let mut is_any_printing = false;
        for i in 0..jobs_model_clone.row_count() {
            if let Some(job) = jobs_model_clone.row_data(i) {
                if job.status == "يتم الطبع الآن" {
                    is_any_printing = true;
                    break;
                }
            }
        }

        if !is_any_printing {
            for i in 0..jobs_model_clone.row_count() {
                if let Some(mut job) = jobs_model_clone.row_data(i) {
                    if job.status == "قيد الانتظار" {
                        job.status = "يتم الطبع الآن".into();
                        job.status_color = Color::from_rgb_u8(59, 130, 246);
                        job.progress = 0.05;
                        jobs_model_clone.set_row_data(i, job.clone());
                        
                        let log_msg = format!("[نظام] بدأت طباعة '{}' في قائمة الانتظار.", job.doc_name);
                        logs_model_clone.push(log_msg.into());
                        break;
                    }
                }
            }
        }

        // Process completed jobs: update printer states
        for (_job_id, p_name, doc_name) in completed_jobs {
            for j in 0..printers_model_clone.row_count() {
                if let Some(mut p) = printers_model_clone.row_data(j) {
                    if p.name == p_name {
                        if p.jobs_count > 0 {
                            p.jobs_count -= 1;
                        }
                        if p.jobs_count == 0 {
                            p.status = "جاهز".into();
                            p.status_color = Color::from_rgb_u8(16, 185, 129);
                        }
                        printers_model_clone.set_row_data(j, p.clone());
                        
                        let sel = app.get_selected_printer();
                        if sel.id == p.id {
                            app.set_selected_printer(p);
                        }
                        break;
                    }
                }
            }
            
            let log_msg = format!("[نظام] اكتملت طباعة '{}' على الطابعة '{}' بنجاح.", doc_name, p_name);
            logs_model_clone.push(log_msg.into());
            
            app.set_notification_msg(format!("اكتملت طباعة وثيقة: {}", doc_name).into());
            app.set_notification_color(Color::from_rgb_u8(16, 185, 129));
        }

        // Prune completed jobs from the list automatically
        let mut idx = 0;
        while idx < jobs_model_clone.row_count() {
            if let Some(job) = jobs_model_clone.row_data(idx) {
                if job.status == "مكتمل" {
                    jobs_model_clone.remove(idx);
                } else {
                    idx += 1;
                }
            } else {
                break;
            }
        }
    });

    app.run()
}

pub fn format_system_time(time: std::time::SystemTime) -> String {
    if let Ok(duration) = time.duration_since(std::time::SystemTime::UNIX_EPOCH) {
        let secs = duration.as_secs();
        
        // GMT+3 Offset for Sumer OS user local time
        let local_secs = secs + 3 * 3600;
        
        let days_since_epoch = local_secs / 86400;
        let mut year = 1970;
        let mut days_left = days_since_epoch;
        
        loop {
            let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            let days_in_year = if is_leap { 366 } else { 365 };
            if days_left < days_in_year {
                break;
            }
            days_left -= days_in_year;
            year += 1;
        }
        
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let month_days = if is_leap {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        
        let mut month = 1;
        for &days in month_days.iter() {
            if days_left < days {
                break;
            }
            days_left -= days;
            month += 1;
        }
        let day = days_left + 1;
        
        let hour = (local_secs % 86400) / 3600;
        let min = (local_secs % 3600) / 60;
        
        format!("{:04}-{:02}-{:02} {:02}:{:02}", year, month, day, hour, min)
    } else {
        "غير معروف".to_string()
    }
}

use std::fs;
use crate::FileItem;

pub fn sort_file_items(items: &mut [FileItem], sort_mode: &str) {
    items.sort_by(|a, b| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            match sort_mode {
                "z-a" => b.name.to_lowercase().cmp(&a.name.to_lowercase()),
                "latest" | "oldest" => {
                    let time_a = fs::metadata(a.path.as_str()).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    let time_b = fs::metadata(b.path.as_str()).and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                    if sort_mode == "latest" {
                        time_b.cmp(&time_a)
                    } else {
                        time_a.cmp(&time_b)
                    }
                }
                "size" => {
                    let len_a = fs::metadata(a.path.as_str()).map(|m| m.len()).unwrap_or(0);
                    let len_b = fs::metadata(b.path.as_str()).map(|m| m.len()).unwrap_or(0);
                    len_b.cmp(&len_a)
                }
                "type" => a.file_type.to_lowercase().cmp(&b.file_type.to_lowercase()),
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()), // "a-z"
            }
        }
    });
}

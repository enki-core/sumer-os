use std::fs;
use std::path::Path;

pub fn create_item(dir: &Path, name: &str, is_folder: bool) -> std::io::Result<()> {
    let target = dir.join(name);
    if is_folder {
        fs::create_dir_all(&target)?;
    } else {
        fs::File::create(&target)?;
    }
    Ok(())
}

pub fn rename_item(old_path_str: &str, new_name: &str) -> std::io::Result<()> {
    let old_path = Path::new(old_path_str);
    if let Some(parent) = old_path.parent() {
        let new_path = parent.join(new_name);
        fs::rename(old_path, new_path)?;
    }
    Ok(())
}

pub fn delete_item(path_str: &str) -> std::io::Result<()> {
    let path = Path::new(path_str);
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

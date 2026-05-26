use std::fs;
use std::path::Path;

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn paste_item(src_str: &str, target_dir_str: &str, is_cut: bool) -> std::io::Result<()> {
    let src = Path::new(src_str);
    let target_dir = Path::new(target_dir_str);
    if !src.exists() || !target_dir.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "مسار المصدر أو الهدف غير موجود"));
    }

    let file_name = src.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "اسم ملف غير صالح")
    })?;
    let dst = target_dir.join(file_name);

    if is_cut {
        fs::rename(src, &dst)?;
    } else if src.is_dir() {
        copy_dir_all(src, &dst)?;
    } else {
        fs::copy(src, &dst)?;
    }
    Ok(())
}

use std::path::{Path, PathBuf};
use slint::SharedString;
use crate::PathSegment;

pub fn generate_path_segments(path: &Path) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    
    // Add Root segment
    segments.push(PathSegment {
        name: SharedString::from("الجذر 💾"),
        path: SharedString::from("/"),
    });

    let mut current = PathBuf::from("/");
    
    // Iterate over components of the path
    for component in path.components() {
        if let std::path::Component::Normal(name) = component {
            let name_str = name.to_string_lossy().to_string();
            current.push(&name_str);
            segments.push(PathSegment {
                name: SharedString::from(name_str),
                path: SharedString::from(current.to_string_lossy().to_string()),
            });
        }
    }
    
    segments
}

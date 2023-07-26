use std::fs::metadata;
use std::path::Path;

pub fn get_file_size(path: &Path) -> Option<u64> {
    if let Ok(file) = metadata(path) {
        return Some(file.len());
    }
    None
}

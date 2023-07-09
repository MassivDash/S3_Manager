// Import the std::path::Path type
use std::path::Path;

// Define a function to get a file name from a full path
pub fn get_file_name(full_path: &str) -> &str {
    // Check if the full path is empty
    if full_path.is_empty() {
        // Return an empty string
        return "";
    }

    // Parse the full path as a Path type
    let path = Path::new(full_path);

    // Get the file name as an Option<&OsStr>
    let file_name = path.file_name();

    // Convert the file name to a string slice or return an empty string
    match file_name {
        Some(name) => name.to_str().unwrap_or(""),
        None => "",
    }
}

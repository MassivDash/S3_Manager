#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::lib::s3::get::file::save_files;
use crate::lib::s3::get::files::{get_files, get_cached_files};
use crate::lib::s3::get::images::get_all_images;
use crate::lib::s3::get::image::get_image;

use crate::lib::s3::put::files::put_files;


mod lib;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Bucket already exists")]
    ListBucketsError(#[from] aws_sdk_s3::Error),
}

// we must also implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![get_files, get_cached_files, get_all_images, get_image, put_files, save_files])
        .run(context)
        .expect("error while running tauri application");
}



// Get object using presigned request.
// snippet-start:[s3.rust.get-object-presigned]


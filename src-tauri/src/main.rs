#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::lib::s3::get::buckets::get_buckets;
use crate::lib::s3::get::file::save_files;
use crate::lib::s3::get::files::get_files;
use crate::lib::s3::get::image::get_image;
use crate::lib::s3::get::images::{get_all_images, get_cached_images};
use crate::lib::s3::get::movies::{get_all_movies, get_cached_movies};

use crate::lib::s3::delete::files::delete_files;
use crate::lib::s3::delete::folder::delete_folders;
use crate::lib::s3::get::tags::get_all_tags;
use crate::lib::s3::put::files::put_files;
use crate::lib::s3::put::folder::put_folder;
use crate::lib::s3::put::tags::set_all_tags;

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
        .invoke_handler(tauri::generate_handler![
            get_files,
            get_buckets,
            get_cached_images,
            get_all_images,
            get_all_movies,
            get_cached_movies,
            get_image,
            put_files,
            put_folder,
            save_files,
            delete_files,
            delete_folders,
            get_all_tags,
            set_all_tags,
        ])
        .run(context)
        .expect("error while running tauri application");
}

// Get object using presigned request.
// snippet-start:[s3.rust.get-object-presigned]

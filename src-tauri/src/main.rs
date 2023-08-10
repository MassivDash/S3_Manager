#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::libs::s3::get::buckets::get_buckets;
use crate::libs::s3::get::file::save_files;
use crate::libs::s3::get::files::get_files;
use crate::libs::s3::get::image::get_image;
use crate::libs::s3::get::images::{get_all_images, get_cached_images};
use crate::libs::s3::get::movies::{get_all_movies, get_cached_movies};

use crate::libs::s3::delete::files::delete_files;
use crate::libs::s3::delete::folder::delete_folders;
use crate::libs::s3::get::tags::get_all_tags;
use crate::libs::s3::put::files::put_files;
use crate::libs::s3::put::folder::put_folder;
use crate::libs::s3::put::tags::set_all_tags;

use crate::libs::tauri::operations::close_splashscreen::close_splashscreen;
use crate::libs::tauri::operations::show_folder::show_folder;

use crate::libs::tauri::menu::menu::get_menu;
use crate::libs::tauri::menu::on_menu_event::on_menu_event_handler;

mod libs;

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
        .menu(get_menu(&context))
        .invoke_handler(tauri::generate_handler![
            get_files,
            show_folder,
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
            close_splashscreen
        ])
        .on_menu_event(on_menu_event_handler)
        .run(context)
        .expect("error while running tauri application");
}

// Get object using presigned request.
// snippet-start:[s3.rust.get-object-presigned]

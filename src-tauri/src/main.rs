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

use tauri::api::dialog::message;
use tauri::{CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu};

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

    // let menu = Menu::os_default(&context.package_info().name)
    // .add_submenu(Submenu::new(
    //     "About",
    //     Menu::with_items([
    //       CustomMenuItem::new("AboutS3", "About S3").into(),
    //     ])
    //   ))

    tauri::Builder::default()
        .menu(
            Menu::os_default(&context.package_info().name)
                .add_submenu(Submenu::new(
                    "Operations",
                    Menu::with_items([
                        CustomMenuItem::new("resync", "Resync").into(),
                        CustomMenuItem::new("upload", "Upload files").into(),
                    ]),
                ))
                .add_submenu(Submenu::new(
                    "Help",
                    Menu::with_items([
                        CustomMenuItem::new("docs", "Documentation").into(),
                        MenuEntry::NativeItem(MenuItem::Separator),
                        CustomMenuItem::new("AboutS3", "About S3").into(),
                        CustomMenuItem::new("spaceout", "About spaceout").into(),
                    ]),
                )), // add resync to the file menu
        )
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
        .on_menu_event(|event| match event.menu_item_id() {
            "resync" => {
                event
                    .window()
                    .emit("event-resync", "resync menu action")
                    .unwrap();
            }
            "upload" => {
                event
                    .window()
                    .emit("event-upload", "upload menu action")
                    .unwrap();
            }
            "AboutS3" => message(
                Some(event.window()),
                "About S3",
                "So you want to know more about s3",
            ),
            "spaceout" => {
                event.window().emit("event-menu", "spaceout").unwrap();
            }
            _ => {}
        })
        .run(context)
        .expect("error while running tauri application");
}

// Get object using presigned request.
// snippet-start:[s3.rust.get-object-presigned]

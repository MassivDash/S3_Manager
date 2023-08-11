use tauri::api::dialog::message;
use tauri::WindowMenuEvent;

use crate::libs::tauri::operations::open_url::open_url;

pub fn on_menu_event_handler(event: WindowMenuEvent) -> () {
    match event.menu_item_id() {
        "resync" => {
            event
                .window()
                .emit("event-resync", "resync menu action")
                .unwrap();
        }
        "upload-files" => {
            event
                .window()
                .emit("event-upload-menu-files", "upload menu action")
                .unwrap();
        }
        "upload-folders" => {
            event
                .window()
                .emit("event-upload-menu-folders", "upload menu action")
                .unwrap();
        }
        "AboutS3" => match open_url("https://s3manager.spaceout.pl") {
            Ok(_) => {}
            Err(err) => {
                message(
                    Some(event.window()),
                    "Error",
                    format!("Error opening docs: {}", err),
                );
            }
        },
        "spaceout" => match open_url("https://spaceout.pl") {
            Ok(_) => {}
            Err(err) => {
                message(
                    Some(event.window()),
                    "Error",
                    format!("Error opening docs: {}", err),
                );
            }
        },
        "docs" => match open_url("https://github.com/MassivDash/S3_Manager") {
            Ok(_) => {}
            Err(err) => {
                message(
                    Some(event.window()),
                    "Error",
                    format!("Error opening docs: {}", err),
                );
            }
        },
        _ => {}
    }
}

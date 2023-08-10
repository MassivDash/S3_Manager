use tauri::api::dialog::message;
use tauri::WindowMenuEvent;

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
        "AboutS3" => message(
            Some(event.window()),
            "About S3",
            "So you want to know more about s3",
        ),
        "spaceout" => {
            event.window().emit("event-menu", "spaceout").unwrap();
        }
        _ => {}
    }
}

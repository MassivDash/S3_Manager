use tauri::{
    utils::assets::EmbeddedAssets, Context, CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu,
};

pub fn get_menu(context: &Context<EmbeddedAssets>) -> Menu {
    Menu::os_default(&context.package_info().name)
        .add_submenu(Submenu::new(
            "Operations",
            Menu::with_items([
                CustomMenuItem::new("resync", "Resync").into(),
                MenuEntry::NativeItem(MenuItem::Separator),
                CustomMenuItem::new("upload-files", "Upload files").into(),
                CustomMenuItem::new("upload-folders", "Upload folders").into(),
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
        ))
}

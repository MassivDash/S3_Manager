#[cfg(target_os = "linux")]
use fork::{daemon, Fork};
use std::process::Command;
#[cfg(target_os = "linux")]
use std::{fs::metadata, path::PathBuf};

use crate::lib::s3::utils::response_error::{create_error, ResponseError};

#[tauri::command]
pub async fn show_folder(path: String) -> Result<(), ResponseError> {
    #[cfg(target_os = "windows")]
    {
        match Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
        {
            Ok(_) => (),
            Err(err) => return Err(create_error("folder open error".into(), err.to_string())),
        }
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            match Command::new("xdg-open").arg(&new_path).spawn() {
                Ok(_) => (),
                Err(err) => return Err(create_error("folder open errror".into(), err.to_string())),
            };
        } else {
            if let Ok(Fork::Child) = daemon(false, false) {
                match Command::new("dbus-send")
                    .args([
                        "--session",
                        "--dest=org.freedesktop.FileManager1",
                        "--type=method_call",
                        "/org/freedesktop/FileManager1",
                        "org.freedesktop.FileManager1.ShowItems",
                        format!("array:string:\"file://{path}\"").as_str(),
                        "string:\"\"",
                    ])
                    .spawn()
                {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        return Err(create_error("folder open error".into(), err.to_string()))
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        match Command::new("open").args(["-R", &path]).spawn() {
            Ok(_) => Ok(()),
            Err(err) => return Err(create_error("folder open error".into(), err.to_string())),
        }
    }
}

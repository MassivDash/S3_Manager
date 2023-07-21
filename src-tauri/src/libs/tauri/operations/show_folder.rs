#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use std::{fs::metadata, path::PathBuf};

use crate::libs::s3::utils::response_error::{create_error, ResponseError};

#[tauri::command]
pub async fn show_folder(path: String) -> Result<(), ResponseError> {
    #[cfg(target_os = "windows")]
    {
        return match Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(err) => return Err(create_error("folder open error".into(), err.to_string())),
        };
    }

    #[cfg(target_os = "linux")]
    {
        let image_extents = path.contains(".jpg")
            || path.contains(".jpeg")
            || path.contains(".png")
            || path.contains(".gif")
            || path.contains(".bmp");
        let video_extents = path.contains(".mp4")
            || path.contains(".avi")
            || path.contains(".mov")
            || path.contains(".mkv")
            || path.contains(".wmv")
            || path.contains(".flv")
            || path.contains(".webm")
            || path.contains(".m4v")
            || path.contains(".mpg")
            || path.contains(".mpeg")
            || path.contains(".3gp")
            || path.contains(".3g2");
        if image_extents || video_extents || path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            return match Command::new("xdg-open").arg(&new_path).spawn() {
                Ok(_) => Ok(()),
                Err(err) => {
                    return Err(create_error(
                        "xdg-open folder open error".into(),
                        err.to_string(),
                    ))
                }
            };
        } else {
            return match Command::new("xdg-open").arg(&path).spawn() {
                Ok(_) => Ok(()),
                Err(err) => {
                    return Err(create_error(
                        "xdg-open folder open error".into(),
                        err.to_string(),
                    ))
                }
            };
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

//  This test works, but it opens a folder on the local machine, not in the container.

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_show_folder() {
//         let path = "/path/to/folder".to_string();
//         let result = show_folder(path).await;
//         assert!(result.is_ok());
//     }
// }

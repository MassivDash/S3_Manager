#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client};
use serde::{Deserialize, Serialize};

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
        .invoke_handler(tauri::generate_handler![buckets])
        .run(context)
        .expect("error while running tauri application");
}




#[derive(Serialize, Deserialize, Clone)]
pub struct MyBucket {
    pub name: String,
    pub files: Vec<String>
}

#[tauri::command]
async fn buckets() -> String {
    // snippet-start:[s3.rust.client-client]
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    // snippet-end:[s3.rust.client-client]

    let resp = client.list_buckets().send().await;
    match resp {
        Ok(resp) => {
            let buckets = resp.buckets().unwrap();
            let mut my_buckets = Vec::new();
            for bucket in buckets {
                my_buckets.push(MyBucket {
                    name: bucket.name().unwrap_or_default().to_string(),
                    files: show_objects(&client, bucket.name().unwrap_or_default()).await
                });
                
            }
            return serde_json::to_string(&my_buckets).unwrap()
        }
        Err(_) => "Error".to_string(),
    }
}

async fn show_objects(client: &Client, bucket: &str) -> Vec<String> {
  let resp = client.list_objects_v2().bucket(bucket).send().await;
  let mut files: Vec<String> = Vec::new();  
  match resp {
    Ok(resp) => {
    for object in resp.contents().unwrap_or_default() {
      files.push(object.key().unwrap_or_default().to_string());
    }
    return files
  }
    Err(_) => files,
  }
}
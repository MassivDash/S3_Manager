use crate::libs::s3::utils::get_file_name::get_file_name;
use crate::libs::s3::utils::response_error::create_error;
use crate::libs::s3::{client::client::create_client, utils::response_error::ResponseError};
use crate::libs::tauri::operations::get_file_size::get_file_size;

use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::Client;
use aws_smithy_http::byte_stream::{ByteStream, Length};
use std::{error::Error, fs, path::Path};
use tauri::Window;
use walkdir::WalkDir;

// AWS recommends using multipart uploads for objects larger than 100 MB.
const MULTIPART_THRESHOLD: u64 = 1024 * 1024 * 100;

//Divide the file into 5MB chunks

//AWS recommends using part sizes between 5 MB and 15 MB for most scenarios.
//This is because smaller part sizes can result in a larger number of parts, which can increase the likelihood of errors and slow down the upload process.
//Larger part sizes can also increase the likelihood of errors and can result in slower upload times if the network speed is slow or if the client machine does not have enough memory to handle the larger parts.

const CHUNK_SIZE: u64 = 1024 * 1024 * 10;

//Maximum number of chunks allowed
//Prevent rust stack overflow on lower end systems
const MAX_CHUNKS: u64 = 10000;

pub async fn put_file(
    window: &Window,
    client: &Client,
    bucket_name: String,
    file_name: String,
    key: String,
) -> Result<(), Box<dyn Error>> {
    // get actual file name from path
    let actual_file_name = get_file_name(&file_name);
    // get file size
    let file_size = get_file_size(Path::new(&file_name)).unwrap_or_default();

    if file_size > MULTIPART_THRESHOLD {
        // Emit the window event to update the progress bar
        window.emit("event-upload-file", &actual_file_name).unwrap();

        //Inform the user that we are starting a multipart upload
        window
            .emit(
                "event-multipart-upload-file",
                format!("{} ... currently at 0%", actual_file_name),
            )
            .unwrap();

        let multipart_upload = client
            .create_multipart_upload()
            .bucket(bucket_name.clone())
            .key(key.clone())
            .send()
            .await?;

        let upload_id = multipart_upload.upload_id().unwrap();
        let mut parts = Vec::new();

        let mut chunk_count = (file_size / CHUNK_SIZE) + 1;
        let mut size_of_last_chunk = file_size % CHUNK_SIZE;
        if size_of_last_chunk == 0 {
            size_of_last_chunk = CHUNK_SIZE;
            chunk_count -= 1;
        }

        if chunk_count > MAX_CHUNKS {
            panic!("Too many chunks! Try increasing your chunk size.")
        }

        for chunk_index in 0..chunk_count {
            let this_chunk = if chunk_count - 1 == chunk_index {
                size_of_last_chunk
            } else {
                CHUNK_SIZE
            };
            let stream = ByteStream::read_from()
                .path(&file_name)
                .offset(chunk_index * CHUNK_SIZE)
                .length(Length::Exact(this_chunk))
                .build()
                .await
                .unwrap();
            //Chunk index needs to start at 0, but part numbers start at 1.
            let part_number = (chunk_index as i32) + 1;

            let upload_part_res = client
                .upload_part()
                .key(&key)
                .bucket(&bucket_name)
                .upload_id(upload_id)
                .body(stream)
                .part_number(part_number)
                .send()
                .await?;

            // Calculate the percentage of the upload
            // We need to add 1 to the chunk index because it starts at 0

            let percentage = ((chunk_index + 1) as f64 / chunk_count as f64) * 100.0;
            let round = percentage.round();

            // Emit the window event to update the progress bar
            window
                .emit(
                    "event-multipart-upload-file",
                    format!("{} ... currently at {}%", actual_file_name, round),
                )
                .unwrap();

            parts.push(
                CompletedPart::builder()
                    .e_tag(upload_part_res.e_tag.unwrap_or_default())
                    .part_number(part_number)
                    .build(),
            );
        }

        let completed_multipart_upload: CompletedMultipartUpload =
            CompletedMultipartUpload::builder()
                .set_parts(Some(parts))
                .build();

        client
            .complete_multipart_upload()
            .bucket(bucket_name.clone())
            .key(key.clone())
            .upload_id(multipart_upload.upload_id.clone().unwrap())
            .multipart_upload(completed_multipart_upload)
            .send()
            .await?;

        window.emit("event-multipart-upload-file", "").unwrap();

        Ok(())
    } else {
        // Emit the window event to update the progress bar
        window.emit("event-upload-file", &actual_file_name).unwrap();

        let body = ByteStream::from_path(Path::new(&file_name)).await;
        client
            .put_object()
            .bucket(bucket_name)
            .key(key)
            .body(body.unwrap())
            .send()
            .await?;

        Ok(())
    }
}

#[tauri::command]
pub async fn put_files(
    window: Window,
    bucket_name: String,
    folder_name: String,
    files: Vec<String>,
) -> Result<bool, ResponseError> {
    let client_call = create_client().await;

    let client = match client_call {
        Ok(instance) => instance,
        Err(err) => {
            return Err(create_error(
                "AWS Client Config error".into(),
                err.to_string(),
            ))
        }
    };

    for file in files {
        let path = fs::metadata(&file).unwrap();
        if path.is_dir() {
            for fil in WalkDir::new(&file).into_iter().filter_map(|fil| fil.ok()) {
                if fil.metadata().unwrap().is_file() {
                    let filename = get_file_name(&fil.path().to_str().unwrap_or_default());
                    let key = folder_name.clone() + "/" + filename;
                    println!("{} {}", filename, !filename.starts_with("."));

                    if !filename.starts_with(".") {
                        match put_file(
                            &window,
                            &client,
                            bucket_name.to_string(),
                            fil.path().display().to_string(),
                            key,
                        )
                        .await
                        {
                            Ok(_) => {
                                println!("uploaded {}", filename);
                            }
                            Err(err) => {
                                return Err(create_error(
                                    "Error uploading file".into(),
                                    err.to_string(),
                                ));
                            }
                        }
                    }
                }
            }
        } else {
            let filename = get_file_name(&file);

            let key = folder_name.to_string() + "/" + filename;
            match put_file(
                &window,
                &client,
                bucket_name.to_string(),
                file.clone(),
                key.to_string(),
            )
            .await
            {
                Ok(_) => {
                    println!("uploaded {}", filename);
                }
                Err(err) => {
                    return Err(create_error("Error uploading file".into(), err.to_string()));
                }
            }
        }
    }
    window.emit("event-resync", "upload successful").unwrap();
    return Ok(true);
}

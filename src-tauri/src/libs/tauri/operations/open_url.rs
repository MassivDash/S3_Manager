// Open url in default browser

use crate::libs::s3::utils::response_error::{create_error, ResponseError};

pub fn open_url(url: &str) -> Result<(), ResponseError> {
    match open::that(&url) {
        Ok(_) => Ok(()),
        Err(err) => Err(create_error("docs open error".into(), err.to_string())),
    }
}

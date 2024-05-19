use std::result::Result as StdResult;

use regex::Regex;

use crate::controllers::api::{add::AddError, edit::EditError};

pub enum UrlValidError {
    InvalidUrl(String),
}

impl From<UrlValidError> for AddError {
    fn from(e: UrlValidError) -> Self {
        match e {
            UrlValidError::InvalidUrl(e) => AddError::InvalidUrl(e),
        }
    }
}

impl From<UrlValidError> for EditError {
    fn from(e: UrlValidError) -> Self {
        match e {
            UrlValidError::InvalidUrl(e) => EditError::InvalidUrl(e),
        }
    }
}

const URL_REGEX: &str = r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&/=]*)";

pub fn check_url(url: &str) -> StdResult<(), UrlValidError> {
    if url.is_empty() {
        return Err(UrlValidError::InvalidUrl("URL cannot be empty".to_string()));
    }

    let re = Regex::new(URL_REGEX).unwrap();

    if !re.is_match(url) {
        return Err(UrlValidError::InvalidUrl(
            "Please insert a valid URL".to_string(),
        ));
    }

    Ok(())
}

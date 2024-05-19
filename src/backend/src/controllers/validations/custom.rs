use std::result::Result as StdResult;

use rustrict::CensorStr;

use crate::controllers::api::{add::AddError, edit::EditError};

pub enum CustomValidError {
    InvalidCustom(String),
}

pub fn check_custom(custom: &str, max_length: usize) -> StdResult<(), CustomValidError> {
    if custom.chars().any(|c| c.is_whitespace()) {
        return Err(CustomValidError::InvalidCustom(
            "Custom shortened link contains whitespace".to_string(),
        ));
    }

    if custom.chars().any(|c| !c.is_alphanumeric()) {
        return Err(CustomValidError::InvalidCustom(
            "Custom shortened link contains invalid characters".to_string(),
        ));
    }

    if custom.len() > max_length {
        return Err(CustomValidError::InvalidCustom(
            "Custom shortened link is too long".to_string(),
        ));
    }

    if custom.is_inappropriate() {
        return Err(CustomValidError::InvalidCustom(
            "Custom shortened link is inappropriate".to_string(),
        ));
    }

    Ok(())
}

impl From<CustomValidError> for AddError {
    fn from(e: CustomValidError) -> Self {
        match e {
            CustomValidError::InvalidCustom(e) => AddError::InvalidCustom(e),
        }
    }
}

impl From<CustomValidError> for EditError {
    fn from(e: CustomValidError) -> Self {
        match e {
            CustomValidError::InvalidCustom(e) => EditError::InvalidCustom(e),
        }
    }
}

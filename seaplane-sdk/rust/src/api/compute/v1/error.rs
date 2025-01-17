use reqwest::blocking::Response;

use crate::{api::error::ApiError, error::Result};

pub fn map_api_error(resp: Response) -> Result<Response> {
    if let Err(source) = resp.error_for_status_ref() {
        let kind = source.status().into();
        return Err(ApiError { message: resp.text()?, source, kind }.into());
    }
    Ok(resp)
}

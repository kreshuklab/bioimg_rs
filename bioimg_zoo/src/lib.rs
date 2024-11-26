pub mod auth;
pub mod client;
pub mod collection;

use std::error::Error;

use serde::de::DeserializeOwned;

const APPLICATION_JSON: http::header::HeaderValue = http::header::HeaderValue::from_static("application/json");
const TEXT_PLAIN: &'static str = "text/plain";
const OCTET_STREAM: &'static str = "application/octet-stream";

#[derive(thiserror::Error, Debug)]
pub enum BadResponse{
    #[error("Response returned with code {status}")]
    FaiedResponse{status: http::StatusCode },
    #[error("Response is missing Content-Type header")]
    ResponseMissingContentType,
    #[error("Response Content-Type header is not {APPLICATION_JSON:?} nor {TEXT_PLAIN}")]
    ResponseNotApplicationJson,
    #[error("Could not deserialize response: {inner}")]
    GarbledPayload{inner: serde_json::Error},
    #[error("Could not parse response as a {expected_type_name}")]
    ParsingError{expected_type_name: &'static str, source: Box<dyn Error>}
}

fn read_json_response<T: DeserializeOwned>(response: &http::Response<Vec<u8>>) -> Result<T, BadResponse>{
    if !response.status().is_success(){
        return Err(BadResponse::FaiedResponse { status: response.status() })
    }
    let Some(content_type) = response.headers().get(http::header::CONTENT_TYPE) else {
        return Err(BadResponse::ResponseMissingContentType)
    };
    if content_type != APPLICATION_JSON {
        let raw_content_type = content_type.to_str().map_err(|_| {
            BadResponse::ResponseNotApplicationJson
        })?;
        if !raw_content_type.starts_with(TEXT_PLAIN) && raw_content_type != OCTET_STREAM{
            return Err(BadResponse::ResponseNotApplicationJson)
        }
    }
    serde_json::from_slice::<T>(response.body())
        .map_err(|err| BadResponse::GarbledPayload { inner: err })
}

use crate::postcode_api::api_client::PostcodeApiErrors;
use tide::StatusCode;

#[derive(Debug, thiserror::Error)]
#[error("{status_code}")]
pub struct ErrorResponse {
    status_code: StatusCode,
}

impl ErrorResponse {
    fn new(status_code: StatusCode) -> Self {
        ErrorResponse { status_code }
    }
}

impl From<ErrorResponse> for tide::Response {
    fn from(error: ErrorResponse) -> Self {
        tide::Response::new(error.status_code)
    }
}

impl From<PostcodeApiErrors> for ErrorResponse {
    fn from(postcode_api_errors: PostcodeApiErrors) -> Self {
        match postcode_api_errors {
            PostcodeApiErrors::BadResponse => ErrorResponse::new(StatusCode::BadRequest),
            PostcodeApiErrors::HttpClientError => ErrorResponse::new(StatusCode::BadRequest),
            PostcodeApiErrors::MalformedUrl(_) => ErrorResponse::new(StatusCode::BadRequest),
            PostcodeApiErrors::OtherError => ErrorResponse::new(StatusCode::BadRequest),
        }
    }
}

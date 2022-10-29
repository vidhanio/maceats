use serde::Serialize;
use warp::{hyper::StatusCode, reject::Reject, Reply};

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "error")]
    pub message: String,
    #[serde(skip)]
    pub code: StatusCode,
}

impl ErrorResponse {
    pub const fn new(message: String) -> Self {
        Self {
            message,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Reject for ErrorResponse {}

impl Reply for ErrorResponse {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(warp::reply::json(&self), self.code).into_response()
    }
}

macro_rules! convert_error {
    ($err:ty) => {
        impl From<$err> for ErrorResponse {
            fn from(error: $err) -> Self {
                Self {
                    message: error.to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
        }
    };
}

convert_error!(maceats::Error);

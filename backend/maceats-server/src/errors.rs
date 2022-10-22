use std::fmt::{self, Display, Formatter};

use serde::Serialize;
use warp::{hyper::StatusCode, reject::Reject, Reply};

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    pub message: String,
    #[serde(serialize_with = "serialize_status_code")]
    pub code: StatusCode,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl Reject for Error {}

impl Reply for Error {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(warp::reply::json(&self), self.code).into_response()
    }
}

macro_rules! convert_error {
    ($err:ty) => {
        impl From<$err> for Error {
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

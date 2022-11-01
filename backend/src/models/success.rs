use serde::Serialize;
use warp::{hyper::StatusCode, Reply};

#[derive(Debug, Clone, Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
    #[serde(skip)]
    pub code: StatusCode,
}

impl<T> SuccessResponse<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data,
            code: StatusCode::OK,
        }
    }
}

impl<T> Reply for SuccessResponse<T>
where
    T: Serialize + Send,
{
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(warp::reply::json(&self), self.code).into_response()
    }
}

impl<T> From<T> for SuccessResponse<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

//! An API for [MacEats].
//!
//! [MacEats]: https://maceats.mcmaster.ca

// Clippy warnings
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// Other warnings
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
// Clippy allows
#![allow(clippy::doc_markdown)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::use_self)]

mod handlers;
mod models;
mod routes;

use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use tokio_cron_scheduler::{Job, JobScheduler};
use warp::{
    http::StatusCode,
    hyper::Method,
    reject::{
        InvalidHeader, LengthRequired, MethodNotAllowed, MissingCookie, MissingHeader,
        PayloadTooLarge, UnsupportedMediaType,
    },
    Filter, Rejection, Reply,
};

use models::error::ErrorResponse;
use models::success::SuccessResponse;

use crate::handlers::CACHE;

#[derive(Parser)]
struct Arguments {
    /// The port for the server to listen on.
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::unused_async)]
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found.".to_owned())
    } else if let Some(e) = err.find::<InvalidHeader>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else if let Some(e) = err.find::<MethodNotAllowed>() {
        (StatusCode::METHOD_NOT_ALLOWED, e.to_string())
    } else if let Some(e) = err.find::<LengthRequired>() {
        (StatusCode::LENGTH_REQUIRED, e.to_string())
    } else if let Some(e) = err.find::<MissingCookie>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else if let Some(e) = err.find::<MissingHeader>() {
        (StatusCode::BAD_REQUEST, e.to_string())
    } else if let Some(e) = err.find::<PayloadTooLarge>() {
        (StatusCode::PAYLOAD_TOO_LARGE, e.to_string())
    } else if let Some(e) = err.find::<UnsupportedMediaType>() {
        (StatusCode::UNSUPPORTED_MEDIA_TYPE, e.to_string())
    } else if let Some(&ErrorResponse { code, ref message }) = err.find::<ErrorResponse>() {
        (code, message.clone())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error.".to_owned(),
        )
    };

    Ok(ErrorResponse { message, code }.into_response())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    pretty_env_logger::init();

    let sched = JobScheduler::new().await?;
    sched
        .add(Job::new_async("0 0 0 * * * *", |_, _| {
            Box::pin(async {
                CACHE.lock().await.invalidate();
            })
        })?)
        .await?;
    sched.start().await?;

    let args = Arguments::parse();
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let cors = warp::cors().allow_any_origin().allow_method(Method::GET);
    let log = warp::log("maceats_server");

    let filter = routes::filter()
        .with(cors)
        .with(log)
        .recover(handle_rejection);

    println!("Listening on {addr}");
    warp::serve(filter).run(addr).await;

    Ok(())
}

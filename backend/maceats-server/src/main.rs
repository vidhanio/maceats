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

mod errors;
mod handlers;
mod routes;

use std::net::SocketAddr;

use clap::Parser;

use errors::Error;

#[derive(Parser)]
struct Arguments {
    /// The port for the server to listen on.
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Arguments::parse();

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    println!("Listening on {addr}");

    warp::serve(routes::filter()).run(addr).await;

    Ok(())
}

//! An API wrapper for [MacEats].
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

mod error;
mod food_type;
mod location;
mod times;

mod restaurant;
#[cfg(test)]
mod tests;

use once_cell::sync::Lazy;
use reqwest::Client;

pub use error::{Error, Result};
pub use food_type::{CoffeeBrand, FoodType};
pub use location::Location;
pub use restaurant::Restaurant;
pub use times::{Open, Times};

macro_rules! selector {
    ($selector:literal $(,)?) => {{
        static SELECTOR: ::once_cell::sync::OnceCell<::scraper::Selector> =
            ::once_cell::sync::OnceCell::new();
        SELECTOR.get_or_init(|| {
            ::scraper::Selector::parse($selector).expect("static selector should be valid")
        })
    }};
}
pub(crate) use selector;

macro_rules! regex {
    ($regex:literal $(,)?) => {{
        static REGEX: ::once_cell::sync::OnceCell<::regex::Regex> =
            ::once_cell::sync::OnceCell::new();
        REGEX.get_or_init(|| ::regex::Regex::new($regex).expect("static regex should be valid"))
    }};
}
pub(crate) use regex;

pub(crate) static CLIENT: Lazy<Client> = Lazy::new(Client::new);

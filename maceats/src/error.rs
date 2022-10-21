use thiserror::Error;

/// An error type representing all possible errors that can occur when using
/// this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// A [`reqwest::Error`] occurred.
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),

    /// A [`url::ParseError`] occurred.
    #[error("url parse error")]
    ParseUrl(#[from] url::ParseError),

    /// A [`chrono::ParseError`] occurred.
    #[error("chrono parse error")]
    ParseChrono(#[from] chrono::ParseError),

    /// An element parse error occurred.
    #[error("element parse error: {0}")]
    ParseElement(&'static str),

    /// An error occurred while parsing a [`FoodType`].
    ///
    /// [`FoodType`]: crate::FoodType
    #[error("food type parse error: {0}")]
    ParseFoodType(String),

    /// An error occurred while parsing a [`Location`].
    ///
    /// [`Location`]: crate::Location
    #[error("coffee brand parse error: {0}")]
    ParseCoffeeBrand(String),
}

/// A convenience type representing a [`Result`] with the error type set to
/// [`enum@Error`].
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;

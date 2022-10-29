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

    /// No element matching the selector was found.
    #[error("no element matching selector error: {0}")]
    ElementNotFound(&'static str),

    /// No text was found in the element.
    #[error("element text not found error: {0}")]
    TextNotFound(&'static str),

    /// No attribute was found in the element matching the selector.
    #[error("no attribute matching selector error: {0}")]
    AttributeNotFound(&'static str),

    /// An error ocurred while splitting time on ` - `.
    #[error("error splitting time on ` - `")]
    SplitTime,

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

use std::{
    borrow::ToOwned,
    fmt::{self, Display, Formatter},
    path::Path,
    str::FromStr,
};

use heck::ToKebabCase;
use reqwest::Url;
use scraper::{ElementRef, Html};
use selectors::attr::CaseSensitivity;
use serde::{Deserialize, Serialize};

use crate::{selector, Error, Restaurant, Result, CLIENT};

/// A location where [`Restaurant`]s are located.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Location {
    /// The name of the location.
    pub name: String,

    /// The URL of the location.
    pub slug: String,
}

impl Location {
    /// Create a new [`Location`].
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            slug: name
                .split_whitespace()
                .filter(|s| !(s.eq_ignore_ascii_case("for") || s.eq_ignore_ascii_case("off")))
                .collect::<Vec<_>>()
                .join(" ")
                .to_kebab_case(),
        }
    }

    /// Get the maceats url of this location.
    #[must_use]
    pub fn url(&self) -> Url {
        "https://maceats.mcmaster.ca/locations/"
            .parse::<Url>()
            .expect("static url should be valid")
            .join(&self.slug)
            .expect("slug should be valid")
    }

    /// Get every location on MacEats.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails.
    pub async fn all() -> Result<Vec<Self>> {
        Self::from_location_list_url(
            "https://maceats.mcmaster.ca/locations"
                .parse()
                .expect("static url should be valid"),
        )
        .await
    }

    /// Parse a location list into a [`Vec<Location>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub async fn from_location_list_url(url: Url) -> Result<Vec<Self>> {
        let response = CLIENT.get(url).send().await?.error_for_status()?;
        let html = Html::parse_document(&response.text().await?);

        Self::from_location_list_html(&html)
    }

    /// Parse a location list [`Html`] document into a [`Vec<Location>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub fn from_location_list_html(html: &Html) -> Result<Vec<Self>> {
        html.select(selector!("div.unit.unit-location"))
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the [`Restaurant`]s at this [`Location`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        let response = CLIENT.get(self.url()).send().await?;
        let html = Html::parse_document(&response.text().await?);

        Restaurant::from_restaurant_list_html(&html)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Location {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(s))
    }
}

impl TryFrom<ElementRef<'_>> for Location {
    type Error = Error;

    fn try_from(mut element: ElementRef<'_>) -> Result<Self> {
        debug_assert_eq!(element.value().name(), "div");
        debug_assert!(element
            .value()
            .has_class("unit", CaseSensitivity::CaseSensitive));
        debug_assert!(element
            .value()
            .has_class("unit-location", CaseSensitivity::CaseSensitive));

        element = element
            .select(selector!("a"))
            .next()
            .ok_or_else(|| Error::ElementNotFound("location"))?;

        let name = element
            .text()
            .next()
            .ok_or_else(|| Error::TextNotFound("location"))?
            .trim()
            .to_owned();

        let slug = Path::new(
            &element
                .value()
                .attr("href")
                .ok_or_else(|| Error::AttributeNotFound("location"))?,
        )
        .file_name()
        .ok_or_else(|| Error::AttributeNotFound("location"))?
        .to_str()
        .ok_or_else(|| Error::AttributeNotFound("location"))?
        .to_owned();

        Ok(Self { name, slug })
    }
}

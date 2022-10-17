use std::borrow::ToOwned;

use heck::ToKebabCase;
use reqwest::Url;
use scraper::{ElementRef, Html};
use selectors::attr::CaseSensitivity;
use serde::{Deserialize, Serialize};

use crate::{selector, Error, Restaurant, Result, CLIENT};

/// A location where [`Restaurant`]s are located.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Location {
    name: String,
    url: Url,
}

impl Location {
    /// Create a new [`Location`].
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            url: "https://maceats.mcmaster.ca/locations/"
                .parse::<Url>()
                .and_then(|url| url.join(&name.to_kebab_case()))
                .expect("url should be valid"),
        }
    }

    /// Create a new [`Location`] with a custom url.
    #[must_use]
    pub fn new_with_url(name: &str, url: &Url) -> Self {
        Self {
            name: name.to_owned(),
            url: url.clone(),
        }
    }

    /// Get the url for this [`Location`].
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Get the name of the [`Location`].
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get every location on MacEats.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request fails.
    pub async fn all() -> Result<Vec<Self>> {
        let response = CLIENT
            .get("https://maceats.mcmaster.ca/locations")
            .send()
            .await?
            .error_for_status()?;
        let html = Html::parse_document(&response.text().await?);

        Self::from_html(&html)
    }

    /// Get every location from the given html.
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the response fails.
    pub fn from_html(html: &Html) -> Result<Vec<Self>> {
        html.select(selector!("div.unit.unit-location"))
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the [`Restaurant`]s at this [`Location`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the response fails.
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        let response = CLIENT.get(self.url.clone()).send().await?;
        let html = Html::parse_document(&response.text().await?);

        Restaurant::from_restaurant_page_html(&html)
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
            .ok_or(Error::ParseElement("location"))?;

        let name = element
            .text()
            .next()
            .ok_or(Error::ParseElement("location"))?
            .trim();

        let url = "https://maceats.mcmaster.ca".parse::<Url>()?.join(
            element
                .value()
                .attr("href")
                .ok_or(Error::ParseElement("location"))?,
        )?;

        Ok(Self::new_with_url(name, &url))
    }
}

use std::{
    borrow::ToOwned,
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use chrono::{Duration, Local, NaiveDate, NaiveTime};
use heck::ToKebabCase;
use reqwest::Url;
use scraper::{ElementRef, Html};
use selectors::attr::CaseSensitivity;
use serde::{Deserialize, Serialize};

use crate::{selector, Error, FoodType, Result, CLIENT};

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

        html.select(selector!("div.unit.unit-location"))
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the [`Restaurant`]s at the [`Location`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the response fails.
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        let response = CLIENT.get(self.url.clone()).send().await?;
        let html = Html::parse_document(&response.text().await?);

        html.select(selector!("div.unit"))
            .map(TryInto::try_into)
            .collect()
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
            .ok_or(Error::ParseElement("location"))?;

        let url = element
            .value()
            .attr("href")
            .ok_or(Error::ParseElement("location"))?;

        Ok(Self::new_with_url(name, &url.parse()?))
    }
}

/// A restaurant that serves food.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Restaurant {
    /// The restaurant's name.
    pub name: String,

    /// The restaurant's location.
    pub location: Location,

    /// The restaurant's location data.
    pub location_details: Option<String>,

    /// The location's phone number.
    pub location_phone: Option<String>,

    /// The restaurant's open status.
    pub schedule: Option<BTreeMap<NaiveDate, Times>>,

    /// The restaurant's food type tags.
    pub tags: BTreeSet<FoodType>,
}

impl TryFrom<ElementRef<'_>> for Restaurant {
    type Error = Error;

    fn try_from(element_ref: ElementRef<'_>) -> Result<Self> {
        macro_rules! select_text {
            ($selector:literal, $name:literal) => {
                element_ref
                    .select(selector!($selector))
                    .next()
                    .ok_or(Error::ParseElement($name))?
                    .text()
                    .next()
                    .ok_or(Error::ParseElement($name))?
            };
        }

        macro_rules! select_optional_text {
            ($selector:literal, $name:literal) => {
                element_ref
                    .select(selector!($selector))
                    .next()
                    .map(|element_ref| element_ref.text().next().ok_or(Error::ParseElement($name)))
                    .transpose()?
            };
        }

        let name = select_text!("h1.title", "name").to_owned();

        let location = Location::new(select_text!("h2.location", "location"));

        let location_details =
            select_optional_text!("div.location-data", "location details").map(ToOwned::to_owned);

        let location_phone =
            select_optional_text!("div.location-phone", "location phone").map(ToOwned::to_owned);

        let schedule = element_ref
            .select(selector!("div.schedule"))
            .next()
            .map(|schedule| {
                let today = Local::today().naive_local();

                let times = schedule.select(selector!("td.time")).map(TryInto::try_into);

                (0..7)
                    .zip(times)
                    .map(|(i, times)| times.map(|times| (today + Duration::days(i), times)))
                    .collect()
            })
            .transpose()?;

        let tags = element_ref
            .select(selector!("ul.tags"))
            .next()
            .map(|tags| {
                tags.select(selector!("li"))
                    .map(TryInto::try_into)
                    .collect()
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            name,
            location,
            location_details,
            location_phone,
            schedule,
            tags,
        })
    }
}

/// The times a [`Restaurant`] is open on a given day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Times {
    /// The restaurant is open from [`start`] to [`end`].
    ///
    /// [`start`]: Times::Open.start
    /// [`end`]: Times::Open.end
    Open {
        /// The time the restaurant opens.
        start: NaiveTime,

        /// The time the restaurant closes.
        end: NaiveTime,
    },

    /// The restaurant is closed.
    Closed,
}

impl FromStr for Times {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "Closed" {
            Ok(Self::Closed)
        } else {
            let (start, end) = s.split_once(" - ").ok_or(Error::ParseElement("time"))?;

            let start = NaiveTime::parse_from_str(start, "%l %P")
                .or_else(|_| NaiveTime::parse_from_str(start, "%l:%M %P"))?;
            let end = NaiveTime::parse_from_str(end, "%l %P")
                .or_else(|_| NaiveTime::parse_from_str(end, "%l:%M %P"))?;

            Ok(Self::Open { start, end })
        }
    }
}

impl TryFrom<ElementRef<'_>> for Times {
    type Error = Error;

    fn try_from(element: ElementRef<'_>) -> Result<Self> {
        debug_assert!(element
            .value()
            .has_class("time", CaseSensitivity::CaseSensitive));
        debug_assert_eq!(element.value().name(), "td");

        let text = element.text().next().ok_or(Error::ParseElement("time"))?;

        text.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn restaurants_parse() {
        let location = Location::new("Alumni Memorial Hall");

        location.restaurants().await.unwrap();
    }
}

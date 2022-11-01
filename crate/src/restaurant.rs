use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display, Formatter},
};

use chrono::{Duration, Local, NaiveDate};
use futures::{stream, StreamExt, TryStreamExt};
use reqwest::Url;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};

use crate::{selector, Error, FoodType, Location, Result, Times, CLIENT};

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

impl Restaurant {
    /// Get every restaurant on MacEats.
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub async fn all() -> Result<Vec<Self>> {
        stream::iter(Location::all().await?)
            .then(|location| async move {
                location
                    .restaurants()
                    .await
                    .map(|v| stream::iter(v).map(Ok))
            })
            .try_flatten()
            .try_collect()
            .await
    }

    /// Get all restaurants open now.
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub async fn open_now() -> Result<Vec<Self>> {
        Self::from_restaurant_list_url(
            "https://maceats.mcmaster.ca/open-now"
                .parse()
                .expect("static url is valid"),
        )
        .await
    }

    /// Parse a restaurant list into a [`Vec<Restaurant>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub async fn from_restaurant_list_url(url: Url) -> Result<Vec<Self>> {
        let response = CLIENT.get(url).send().await?.error_for_status()?;
        let html = Html::parse_document(&response.text().await?);

        Self::from_restaurant_list_html(&html)
    }

    /// Parse a restaurant list [`Html`] document into a [`Vec<Restaurant>`].
    ///
    /// # Errors
    ///
    /// This function will return an error if parsing the response fails.
    ///
    /// [`Html`]: scraper::Html
    pub fn from_restaurant_list_html(html: &Html) -> Result<Vec<Self>> {
        html.select(selector!("div.unit"))
            .map(TryInto::try_into)
            .collect()
    }
}

impl Display for Restaurant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TryFrom<ElementRef<'_>> for Restaurant {
    type Error = Error;

    fn try_from(element: ElementRef<'_>) -> Result<Self> {
        macro_rules! select_text {
            ($selector:literal, $name:literal) => {
                element
                    .select(selector!($selector))
                    .next()
                    .ok_or(Error::ElementNotFound($name))?
                    .text()
                    .next()
                    .ok_or(Error::TextNotFound($name))?
                    .trim()
            };
        }

        macro_rules! select_optional_text {
            ($selector:literal, $name:literal) => {
                element
                    .select(selector!($selector))
                    .next()
                    .map(|element| {
                        element
                            .text()
                            .next()
                            .ok_or(Error::TextNotFound($name))
                            .map(|s| s.trim())
                    })
                    .transpose()?
            };
        }

        let name = select_text!("h1.title", "name").to_owned();

        let location = Location::new(select_text!("h2.location", "location"));

        let location_details =
            select_optional_text!("div.location-data", "location details").map(ToOwned::to_owned);

        let location_phone =
            select_optional_text!("div.location-phone", "location phone").map(ToOwned::to_owned);

        let schedule = element
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

        let tags = element
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

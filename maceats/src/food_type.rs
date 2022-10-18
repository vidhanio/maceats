use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use futures::{stream, StreamExt, TryStreamExt};
use reqwest::Url;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};

use crate::{Error, Restaurant, Result, CLIENT};

/// The type of food served at a [`Restaurant`].
///
/// [`Restaurant`]: crate::Restaurant
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FoodType {
    /// Breakfast food.
    Breakfast,

    /// Coffee.
    Coffee(Option<CoffeeBrand>),

    /// Convenience food.
    Convenience,

    /// Dessert.
    Dessert,

    /// Gluten-free food.
    GlutenFree,

    /// Grill food.
    Grill,

    /// Halal food.
    Halal,

    /// Kosher food.
    Kosher,

    /// Noodles.
    Noodles,

    /// Pasta.
    Pasta,

    /// Pizza.
    Pizza,

    /// Sandwiches.
    Sandwiches,

    /// Snacks.
    Snacks,

    /// Soup.
    Soup,

    /// Sushi.
    Sushi,

    /// Vegetarian food.
    Vegetarian,
}

impl FoodType {
    /// Get the urls for this [`FoodType`].
    #[must_use]
    pub fn urls(&self) -> Vec<Url> {
        macro_rules! url {
            ($slug:literal) => {
                vec![concat!("https://maceats.mcmaster.ca/types/", $slug)
                    .parse::<Url>()
                    .expect("static url should be valid")]
            };
        }

        match self {
            Self::Breakfast => url!("breakfast"),
            Self::Coffee(Some(brand)) => vec![brand.url()],
            Self::Coffee(None) => vec![
                CoffeeBrand::Marley.url(),
                CoffeeBrand::Rejuvenate.url(),
                CoffeeBrand::Starbucks.url(),
                CoffeeBrand::TimHortons.url(),
                CoffeeBrand::Williams.url(),
            ],
            Self::Convenience => url!("convenience"),
            Self::Dessert => url!("dessert"),
            Self::GlutenFree => url!("gluten-free"),
            Self::Grill => url!("grill"),
            Self::Halal => url!("halal"),
            Self::Kosher => url!("kosher"),
            Self::Noodles => url!("noodles"),
            Self::Pasta => url!("pasta"),
            Self::Pizza => url!("pizza"),
            Self::Sandwiches => url!("sandwiches"),
            Self::Snacks => url!("snacks"),
            Self::Soup => url!("soup"),
            Self::Sushi => url!("sushi"),
            Self::Vegetarian => url!("vegetarian"),
        }
    }

    /// Get the [`Restaurant`]s that serve this food type.
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the response fails.a
    ///
    /// [`Restaurant`]: crate::Restaurant
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        stream::iter(self.urls())
            .then(|url| async move {
                Restaurant::from_restaurant_list_url(&url)
                    .await
                    .map(|v| stream::iter(v.into_iter().map(Ok)))
            })
            .try_flatten()
            .try_collect::<Vec<_>>()
            .await
    }
}

impl Display for FoodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Breakfast => write!(f, "Breakfast"),
            Self::Coffee(Some(brand)) => write!(f, "Coffee ({})", brand),
            Self::Coffee(None) => write!(f, "Coffee"),
            Self::Convenience => write!(f, "Convenience"),
            Self::Dessert => write!(f, "Dessert"),
            Self::GlutenFree => write!(f, "Gluten Free"),
            Self::Grill => write!(f, "Grill"),
            Self::Halal => write!(f, "Halal"),
            Self::Kosher => write!(f, "Kosher"),
            Self::Noodles => write!(f, "Noodles"),
            Self::Pasta => write!(f, "Pasta"),
            Self::Pizza => write!(f, "Pizza"),
            Self::Sandwiches => write!(f, "Sandwiches"),
            Self::Snacks => write!(f, "Snacks"),
            Self::Soup => write!(f, "Soup"),
            Self::Sushi => write!(f, "Sushi"),
            Self::Vegetarian => write!(f, "Vegetarian"),
        }
    }
}

impl FromStr for FoodType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Breakfast" => Ok(Self::Breakfast),
            "Coffee" => Ok(Self::Coffee(None)),
            "Convenience" => Ok(Self::Convenience),
            "Dessert" => Ok(Self::Dessert),
            "Gluten Free" => Ok(Self::GlutenFree),
            "Grill" => Ok(Self::Grill),
            "Halal" => Ok(Self::Halal),
            "Kosher" => Ok(Self::Kosher),
            "Noodles" => Ok(Self::Noodles),
            "Pasta" => Ok(Self::Pasta),
            "Pizza" => Ok(Self::Pizza),
            "Sandwiches" => Ok(Self::Sandwiches),
            "Snacks" => Ok(Self::Snacks),
            "Soup" => Ok(Self::Soup),
            "Sushi" => Ok(Self::Sushi),
            "Vegetarian" => Ok(Self::Vegetarian),
            s => Err(Error::ParseFoodType(s.into())),
        }
    }
}

impl TryFrom<ElementRef<'_>> for FoodType {
    type Error = Error;

    fn try_from(element: ElementRef<'_>) -> Result<Self> {
        debug_assert_eq!(element.value().name(), "li");

        let text = element
            .text()
            .next()
            .ok_or(Error::ParseElement("food type"))?;

        Self::from_str(text)
    }
}

/// A brand of coffee served at a [`Restaurant`].
///
/// [`Restaurant`]: crate::Restaurant
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CoffeeBrand {
    /// Marley.
    Marley,

    /// Rejuvenate.
    Rejuvenate,

    /// Starbucks.
    Starbucks,

    /// Tim Hortons.
    TimHortons,

    /// Williams.
    Williams,
}

impl CoffeeBrand {
    /// Get the url for this [`CoffeeBrand`].
    #[must_use]
    pub fn url(&self) -> Url {
        match self {
            Self::Marley => "https://maceats.mcmaster.ca/types/coffee/marley",
            Self::Rejuvenate => "https://maceats.mcmaster.ca/types/coffee/rejuvenate",
            Self::Starbucks => "https://maceats.mcmaster.ca/types/coffee/starbucks",
            Self::TimHortons => "https://maceats.mcmaster.ca/types/coffee/tim-hortons",
            Self::Williams => "https://maceats.mcmaster.ca/types/coffee/williams",
        }
        .parse()
        .expect("static url should be valid")
    }

    /// Get the [`Restaurant`]s that serve this coffee brand.
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the response fails.a
    ///
    /// [`Restaurant`]: crate::Restaurant
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        let response = CLIENT.get(self.url()).send().await?;
        let html = Html::parse_document(&response.text().await?);

        Restaurant::from_restaurant_list_html(&html)
    }
}

impl Display for CoffeeBrand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Marley => write!(f, "Marley"),
            Self::Rejuvenate => write!(f, "Rejuvenate"),
            Self::Starbucks => write!(f, "Starbucks"),
            Self::TimHortons => write!(f, "Tim Hortons"),
            Self::Williams => write!(f, "Williams"),
        }
    }
}

impl FromStr for CoffeeBrand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Marley" => Ok(Self::Marley),
            "Rejuvenate" => Ok(Self::Rejuvenate),
            "Starbucks" => Ok(Self::Starbucks),
            "Tim Hortons" => Ok(Self::TimHortons),
            "Williams" => Ok(Self::Williams),
            s => Err(Error::ParseCoffeeBrand(s.into())),
        }
    }
}

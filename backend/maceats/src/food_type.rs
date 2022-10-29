use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use reqwest::Url;
use scraper::ElementRef;
use serde::{Deserialize, Serialize};

use crate::{Error, Restaurant, Result};

/// The type of food served at a [`Restaurant`].
///
/// [`Restaurant`]: crate::Restaurant
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FoodType {
    /// Breakfast food.
    Breakfast,

    /// Coffee.
    Coffee,

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
    pub fn url(&self) -> Option<Url> {
        macro_rules! url {
            ($slug:literal) => {
                Some(
                    concat!("https://maceats.mcmaster.ca/types/", $slug)
                        .parse()
                        .expect("static url should be valid"),
                )
            };
        }

        match self {
            Self::Breakfast => url!("breakfast"),
            Self::Coffee => None,
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
    /// Note that this function will get, then filter all [`Restaurant`]s if
    /// `self` is [`FoodType::Coffee`], as MacEats provides no way to filter for
    /// all coffee. To get the [`Restaurant`]s that serve a specific brand of
    /// coffee, use [`CoffeeBrand::restaurants`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    ///
    /// [`Restaurant`]: crate::Restaurant
    /// [`CoffeeBrand::restaurants`]: crate::CoffeeBrand::restaurants
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        if let Some(url) = self.url() {
            Restaurant::from_restaurant_list_url(url).await
        } else {
            Ok(Restaurant::all()
                .await?
                .into_iter()
                .filter(|r| r.tags.contains(self))
                .collect())
        }
    }

    /// Get the [`Restaurant`]s that serve this food type, returning an empty
    /// [`Vec<Restaurant>`] if `self` is [`FoodType::Coffee`].
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    pub async fn restaurants_no_coffee(&self) -> Result<Vec<Restaurant>> {
        if let Some(url) = self.url() {
            Restaurant::from_restaurant_list_url(url).await
        } else {
            Ok(Vec::new())
        }
    }
}

impl Display for FoodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Breakfast => write!(f, "Breakfast"),
            Self::Coffee => write!(f, "Coffee"),
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
            "Breakfast" | "breakfast" => Ok(Self::Breakfast),
            "Coffee" | "coffee" => Ok(Self::Coffee),
            "Convenience" | "convenience" => Ok(Self::Convenience),
            "Dessert" | "dessert" => Ok(Self::Dessert),
            "Gluten Free" | "gluten-free" => Ok(Self::GlutenFree),
            "Grill" | "grill" => Ok(Self::Grill),
            "Halal" | "halal" => Ok(Self::Halal),
            "Kosher" | "kosher" => Ok(Self::Kosher),
            "Noodles" | "noodles" => Ok(Self::Noodles),
            "Pasta" | "pasta" => Ok(Self::Pasta),
            "Pizza" | "pizza" => Ok(Self::Pizza),
            "Sandwiches" | "sandwiches" => Ok(Self::Sandwiches),
            "Snacks" | "snacks" => Ok(Self::Snacks),
            "Soup" | "soup" => Ok(Self::Soup),
            "Sushi" | "sushi" => Ok(Self::Sushi),
            "Vegetarian" | "vegetarian" => Ok(Self::Vegetarian),
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
            .ok_or_else(|| Error::TextNotFound("food type"))?;

        text.parse()
    }
}

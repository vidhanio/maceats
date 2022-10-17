use std::str::FromStr;

use scraper::ElementRef;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// The type of food served at a [`Restaurant`].
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

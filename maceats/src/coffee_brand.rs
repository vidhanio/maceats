use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{Error, Restaurant, Result};

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
    /// This function will return an error if sending the request or parsing the response fails.
    ///
    /// [`Restaurant`]: crate::Restaurant
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        Restaurant::from_restaurant_list_url(&self.url()).await
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

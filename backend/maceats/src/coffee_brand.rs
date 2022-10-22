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
        macro_rules! url {
            ($slug:literal) => {
                concat!("https://maceats.mcmaster.ca/types/coffee/", $slug)
                    .parse()
                    .expect("static url should be valid")
            };
        }

        match self {
            Self::Marley => url!("marley"),
            Self::Rejuvenate => url!("rejuvenate"),
            Self::Starbucks => url!("starbucks"),
            Self::TimHortons => url!("tim-hortons"),
            Self::Williams => url!("williams"),
        }
    }

    /// Get the [`Restaurant`]s that serve this coffee brand.
    ///
    /// # Errors
    ///
    /// This function will return an error if sending the request or parsing the
    /// response fails.
    ///
    /// [`Restaurant`]: crate::Restaurant
    pub async fn restaurants(&self) -> Result<Vec<Restaurant>> {
        Restaurant::from_restaurant_list_url(self.url()).await
    }
}

impl Display for CoffeeBrand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Marley => "Marley",
                Self::Rejuvenate => "Rejuvenate",
                Self::Starbucks => "Starbucks",
                Self::TimHortons => "Tim Hortons",
                Self::Williams => "Williams",
            }
        )
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

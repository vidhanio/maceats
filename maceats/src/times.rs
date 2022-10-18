use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use chrono::NaiveTime;
use scraper::ElementRef;
use selectors::attr::CaseSensitivity;
use serde::{Deserialize, Serialize};

use crate::{regex, Error, Result};

/// The times a [`Restaurant`] is open on a given day.
///
/// [`Restaurant`]: crate::Restaurant
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Times {
    /// Time ranges the restaurant is open.
    Open(Vec<Open>),

    /// The restaurant is closed.
    Closed,
}

impl Display for Times {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open(times) => times.iter().enumerate().for_each(|(i, time)| {
                if i > 0 {
                    write!(f, ", ").unwrap();
                }

                write!(f, "{}", time).unwrap();
            }),
            Self::Closed => write!(f, "Closed")?,
        }

        Ok(())
    }
}

impl FromStr for Times {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "Closed" {
            Ok(Self::Closed)
        } else {
            Ok(Self::Open(
                s.split(", ").map(str::parse).collect::<Result<_>>()?,
            ))
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

        let text = element
            .text()
            .next()
            .ok_or(Error::ParseElement("time"))?
            .trim();

        text.parse()
    }
}

/// The times a [`Restaurant`] is open on a given day.
///
/// [`Restaurant`]: crate::Restaurant
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Open {
    /// The time the restaurant opens.
    pub from: NaiveTime,

    /// The time the restaurant closes.
    pub to: NaiveTime,
}

impl Display for Open {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}",
            self.from.format("%l:%M %P").to_string().trim(),
            self.to.format("%l:%M %P").to_string().trim(),
        )
    }
}

impl FromStr for Open {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (from_s, to_s) = s.split_once(" - ").ok_or(Error::ParseElement("time"))?;

        let re = regex!(r"^(?P<hour>\d{1,2}) (?P<am_pm>am|pm)$");

        let from_s = re.replace_all(from_s, "$hour:00 $am_pm");
        let to_s = re.replace_all(to_s, "$hour:00 $am_pm");

        let from = NaiveTime::parse_from_str(&from_s, "%l:%M %P")?;
        let to = NaiveTime::parse_from_str(&to_s, "%l:%M %P")?;

        Ok(Self { from, to })
    }
}

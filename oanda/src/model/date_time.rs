use chrono::{DateTime, TimeZone, Utc};
use deref_derive::Deref;
use error_stack::{report, ResultExt};
use parse_display::{Display, FromStr};
use serde::Serialize;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::Error;

/// DateTime Wrapper for easy io with Oanda's API
/// See <https://developer.oanda.com/rest-live-v20/primitives-df/#DateTime>
#[derive(Debug, Deref, Serialize)]
pub struct DateTimeWrapper(DateTime<Utc>);

impl Display for DateTimeWrapper {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(&self.0.to_rfc3339())
    }
}

impl FromStr for DateTimeWrapper {
    type Err = error_stack::Report<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(datetime) = DateTime::parse_from_rfc3339(s) {
            Ok(DateTimeWrapper(datetime.with_timezone(&Utc)))
        } else {
            let seconds = s.parse::<f64>().map_err(Error::from)?;
            let Some (datetime) = Utc
                .timestamp_opt(seconds.trunc() as i64, (seconds.fract() * 1e9) as u32).single() else {
                    return Err(report!(Error::Other)).attach_printable("We got a unixtimestamp that's out of range of real dates: {seconds}");
            };
            Ok(DateTimeWrapper(datetime))
        }
    }
}

#[derive(Display, FromStr, Debug, Clone, Copy, Serialize)]
#[display(style = "UPPERCASE")]
pub enum DateTimeFormat {
    /// DateTime fields will be specified or returned in the “12345678.000000123” format.
    Unix,
    /// DateTime will be specified or returned in “YYYY-MM-DDTHH:MM:SS.nnnnnnnnnZ” format.
    Rfc3339,
}

impl Default for DateTimeFormat {
    fn default() -> Self {
        DateTimeFormat::Rfc3339
    }
}

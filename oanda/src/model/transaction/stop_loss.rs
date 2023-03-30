use error_stack::{report, Report};

use crate::Error;

pub use self::rust::{SLTrigger, StopLoss, TimeInForce};

// Builder / rust side
mod rust {
    use crate::model::trade::ClientExtensions;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use typed_builder::TypedBuilder;

    #[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum TimeInForce {
        /// The Order is "Good until Cancelled".
        #[default]
        Gtc,
        /// The Order is "Good until Date" and will be cancelled at the provided time.
        Gtd(DateTime<Utc>),
        /// The Order is "Good For Day" and will be cancelled at 5pm New York time.
        Gfd,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub enum SLTrigger {
        /// The price that the Stop Loss Order will be triggered at. Only one of
        /// the price and distance fields may be specified.
        Price(f32),
        /// Specifies the distance (in price units) from the Trade’s open price to
        /// use as the Stop Loss Order price. Only one of the distance and price
        /// fields may be specified.
        Distance(f32),
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, TypedBuilder)]
    #[serde(
        rename_all = "camelCase",
        into = "super::oanda::StopLoss",
        try_from = "super::oanda::StopLoss"
    )]
    #[doc()]
    pub struct StopLoss {
        /// Either the price or distance-from-current for the stop loss to trigger
        pub trigger: SLTrigger,

        /// The time in force for the created Stop Loss Order. This may only be GTC,
        /// GTD or GFD.
        #[builder(default)]
        pub time_in_force: TimeInForce,

        /// The Client Extensions to add to the Stop Loss Order when created.
        #[builder(default, setter(strip_option))]
        pub client_extensions: Option<ClientExtensions>,
    }
}

impl From<rust::StopLoss> for oanda::StopLoss {
    fn from(stop_loss: rust::StopLoss) -> Self {
        let (price, distance) = match stop_loss.trigger {
            SLTrigger::Price(price) => (Some(price), None),
            SLTrigger::Distance(distance) => (None, Some(distance)),
        };
        let (time_in_force, gtd_time) = match stop_loss.time_in_force {
            TimeInForce::Gtc => (oanda::TimeInForce::Gtc, None),
            TimeInForce::Gtd(date) => (oanda::TimeInForce::Gtd, Some(date)),
            TimeInForce::Gfd => (oanda::TimeInForce::Gfd, None),
        };
        Self {
            price,
            distance,
            gtd_time,
            time_in_force,
            client_extensions: stop_loss.client_extensions,
        }
    }
}

impl TryFrom<oanda::StopLoss> for rust::StopLoss {
    type Error = Report<Error>;

    fn try_from(input: oanda::StopLoss) -> Result<Self, Report<Error>> {
        let trigger = match (input.price, input.distance) {
            (None, None) => {
                return Err(report!(Error::JsonConversion).attach_printable(format!(
                    "Incoming stop loss conversion. No price nor distance: {input:#?}"
                )))
            }
            (None, Some(distance)) => SLTrigger::Distance(distance),
            (Some(price), None) => SLTrigger::Price(price),
            (Some(_), Some(_)) => {
                return Err(report!(Error::JsonConversion).attach_printable(format!(
                    "Incoming stop loss conversion. Both price and distance: {input:#?}"
                )))
            }
        };
        let time_in_force = match (input.time_in_force, input.gtd_time) {
            (oanda::TimeInForce::Gtc, _) => rust::TimeInForce::Gtc,
            (oanda::TimeInForce::Gtd, Some(gtd_time)) => rust::TimeInForce::Gtd(gtd_time),
            (oanda::TimeInForce::Gtd, None) => return Err(report!(Error::JsonConversion)
                .attach_printable(format!("Incoming StopLoss had time in force as good til date, but didn't provide a date: {input:#?}"))),
            (oanda::TimeInForce::Gfd, _) => rust::TimeInForce::Gfd,
        };
        Ok(Self {
            trigger,
            time_in_force,
            client_extensions: input.client_extensions,
        })
    }
}

// Oanda / json side
mod oanda {
    use crate::model::trade::ClientExtensions;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, DisplayFromStr};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
    #[serde(rename_all = "UPPERCASE")]
    pub enum TimeInForce {
        /// The Order is "Good until Cancelled".
        Gtc,
        /// The Order is "Good until Date" and will be cancelled at the provided time.
        Gtd,
        /// The Order is "Good For Day" and will be cancelled at 5pm New York time.
        Gfd,
    }

    impl Default for TimeInForce {
        fn default() -> Self {
            Self::Gtc
        }
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StopLoss {
        /// The price that the Stop Loss Order will be triggered at. Only one of
        /// the price and distance fields may be specified.
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub price: Option<f32>,

        /// Specifies the distance (in price units) from the Trade’s open price to
        /// use as the Stop Loss Order price. Only one of the distance and price
        /// fields may be specified.
        #[serde_as(as = "Option<DisplayFromStr>")]
        pub distance: Option<f32>,

        //// The date when the Guaranteed Stop Loss Order will be cancelled on if
        //// timeInForce is GTD.
        pub gtd_time: Option<DateTime<Utc>>,

        /// The time in force for the created Stop Loss Order. This may only be GTC,
        /// GTD or GFD.
        pub time_in_force: TimeInForce,

        /// The Client Extensions to add to the Stop Loss Order when created.
        pub client_extensions: Option<ClientExtensions>,
    }

    impl StopLoss {
        /// The price that the Stop Loss Order will be triggered at. Only one of
        /// the price and distance fields may be specified.
        pub fn get_price(&self) -> Option<f32> {
            self.price
        }

        /// Specifies the distance (in price units) from the Trade’s open price to
        /// use as the Stop Loss Order price. Only one of the distance and price
        /// fields may be specified.
        pub fn get_distance(&self) -> Option<f32> {
            self.distance
        }

        //// The date when the Guaranteed Stop Loss Order will be cancelled on if timeInForce is GTD.
        pub fn get_gtd_time(&self) -> Option<DateTime<Utc>> {
            self.gtd_time
        }

        /// The time in force for the created Stop Loss Order. This may only be GTC, GTD or GFD.
        pub fn get_time_in_force(&self) -> TimeInForce {
            self.time_in_force
        }

        /// The Client Extensions to add to the Stop Loss Order when created.
        pub fn get_client_extensions(&self) -> Option<&ClientExtensions> {
            self.client_extensions.as_ref()
        }

        pub fn client_extensions(self, client_extensions: ClientExtensions) -> Self {
            Self {
                price: self.price,
                distance: self.distance,
                gtd_time: self.gtd_time,
                time_in_force: self.time_in_force,
                client_extensions: Some(client_extensions),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::{trade::ClientExtensions, transaction::stop_loss::rust::SLTrigger};
    use chrono::TimeZone;

    use super::{oanda, rust};
    use chrono::Utc;
    use pretty_assertions::assert_eq;

    #[test]
    fn stop_loss_builder() {
        let got = rust::StopLoss::builder()
            .trigger(SLTrigger::Price(1.4))
            .time_in_force(rust::TimeInForce::Gtc)
            .build();
        let got: oanda::StopLoss = got.into();
        let expected = oanda::StopLoss {
            price: Some(1.4),
            time_in_force: oanda::TimeInForce::Gtc,
            ..Default::default()
        };
        assert_eq!(got, expected);
    }

    #[test]
    fn stop_loss_builder_all_fields() {
        let gtd_time = Utc.with_ymd_and_hms(2025, 4, 1, 7, 53, 0).unwrap();
        let extensions = ClientExtensions::builder()
            .id("trader")
            .tag("Joe")
            .comment("open")
            .build();

        let got = rust::StopLoss::builder()
            .trigger(SLTrigger::Distance(99.9))
            .time_in_force(rust::TimeInForce::Gtd(gtd_time))
            .client_extensions(extensions.clone())
            .build();
        let got: oanda::StopLoss = got.into();
        let expected = oanda::StopLoss {
            price: None,
            time_in_force: oanda::TimeInForce::Gtd,
            distance: Some(99.9),
            gtd_time: Some(gtd_time),
            client_extensions: Some(extensions),
        };
        assert_eq!(got, expected);
    }

    #[test]
    fn stop_loss_deserialize() {
        let input = r#"{ "timeInForce": "GTC", "price": "1.7000" }"#;
        let got: rust::StopLoss = serde_json::from_str(&input).unwrap();
        let expected = rust::StopLoss {
            trigger: SLTrigger::Price(1.7),
            time_in_force: rust::TimeInForce::Gtc,
            client_extensions: None,
        };
        assert_eq!(expected, got);
    }
}

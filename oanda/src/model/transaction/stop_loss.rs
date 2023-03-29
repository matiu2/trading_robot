use crate::model::trade::ClientExtensions;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StopLossDetailsCommon {}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum StopLossTimeInForce {
    /// The Order is "Good until Cancelled".
    GTC,
    /// The Order is "Good until Date" and will be cancelled at the provided time.
    GTD,
    /// The Order is "Good For Day" and will be cancelled at 5pm New York time.
    GFD,
}

impl Default for StopLossTimeInForce {
    fn default() -> Self {
        Self::GTC
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopLoss {
    /// The price that the Stop Loss Order will be triggered at. Only one of
    /// the price and distance fields may be specified.
    #[serde_as(as = "Option<DisplayFromStr>")]
    price: Option<f32>,

    /// Specifies the distance (in price units) from the Trade’s open price to
    /// use as the Stop Loss Order price. Only one of the distance and price
    /// fields may be specified.
    #[serde_as(as = "Option<DisplayFromStr>")]
    distance: Option<f32>,

    //// The date when the Guaranteed Stop Loss Order will be cancelled on if
    //// timeInForce is GTD.
    gtd_time: Option<DateTime<Utc>>,

    /// The time in force for the created Stop Loss Order. This may only be GTC,
    /// GTD or GFD.
    time_in_force: StopLossTimeInForce,

    /// The Client Extensions to add to the Stop Loss Order when created.
    client_extensions: Option<ClientExtensions>,
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
    pub fn get_time_in_force(&self) -> StopLossTimeInForce {
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

    pub fn builder() -> StopLossBuilderStart {
        StopLossBuilderStart {}
    }
}

pub struct StopLossBuilderStart {}

impl StopLossBuilderStart {
    pub fn price(self, price: f32) -> StopLossBuilder2 {
        StopLossBuilder2 {
            price: Some(price),
            distance: None,
        }
    }
    pub fn distance(self, distance: f32) -> StopLossBuilder2 {
        StopLossBuilder2 {
            price: None,
            distance: Some(distance),
        }
    }
}

pub struct StopLossBuilder2 {
    price: Option<f32>,
    distance: Option<f32>,
}

impl StopLossBuilder2 {
    /// Good until a certain date
    fn gtd(self, gtd_time: DateTime<Utc>) -> StopLoss {
        let price = self.price;
        let distance = self.distance;
        StopLoss {
            gtd_time: Some(gtd_time),
            price,
            distance,
            time_in_force: StopLossTimeInForce::GTD,
            ..Default::default()
        }
    }
    /// Good until cancelled
    fn gtc(self) -> StopLoss {
        let price = self.price;
        let distance = self.distance;
        StopLoss {
            price,
            distance,
            time_in_force: StopLossTimeInForce::GTC,
            ..Default::default()
        }
    }
    /// Good until 5 pm NY time
    fn gfd(self) -> StopLoss {
        let price = self.price;
        let distance = self.distance;
        StopLoss {
            price,
            distance,
            time_in_force: StopLossTimeInForce::GFD,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NoDateTimeInForce {
    /// The Order is "Good until Cancelled".
    GTC,
    /// The Order is "Good For Day" and will be cancelled at 5pm New York time.
    GFD,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateTimeInForce {
    /// The Order is "Good until Date" and will be cancelled at the provided time.
    GTD,
}

#[cfg(test)]
mod test {
    use crate::model::trade::ClientExtensions;
    use chrono::TimeZone;

    use super::StopLossTimeInForce;
    use chrono::Utc;
    use pretty_assertions::assert_eq;

    #[test]
    fn stop_loss_builder() {
        let got = super::StopLoss::builder().price(1.4).gtc();
        let expected = super::StopLoss {
            price: Some(1.4),
            time_in_force: super::StopLossTimeInForce::GTC,
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

        let got = super::StopLoss::builder()
            .distance(99.9)
            .gtd(gtd_time)
            .client_extensions(extensions.clone());
        let expected = super::StopLoss {
            price: None,
            time_in_force: super::StopLossTimeInForce::GTD,
            distance: Some(99.9),
            gtd_time: Some(gtd_time),
            client_extensions: Some(extensions),
        };
        assert_eq!(got, expected);
    }

    #[test]
    fn stop_loss_deserialize() {
        let input = r#"{ "timeInForce": "GTC", "price": "1.7000" }"#;
        let got = serde_json::from_str(&input).unwrap();
        let expected = super::StopLoss {
            price: Some(1.7),
            time_in_force: StopLossTimeInForce::GTC,
            ..Default::default()
        };
        assert_eq!(expected, got);
    }
}

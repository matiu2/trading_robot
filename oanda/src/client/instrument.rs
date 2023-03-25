pub use crate::model;
use crate::{builder_methods, client::Client, error::Error};
use chrono::{DateTime, Utc};
use error_stack::{Result, ResultExt};
use serde::Serialize;
use std::fmt;
use tracing::debug;

use self::model::{
    candle::CandlestickGranularity,
    date_time::DateTimeFormat,
    instrument::{DayOfWeek, PricingComponent},
};

pub struct Instrument<'a> {
    pub(crate) client: &'a Client,
    /// The instrument name that we'll be dealing with
    pub instrument: String,
}

impl<'a> Instrument<'a> {
    /// See <https://developer.oanda.com/rest-live-v20/instrument-ep/>
    pub fn candles(&self) -> CandleStickRequest {
        CandleStickRequest::new(self)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandleStickRequest<'a> {
    #[serde(skip)]
    instruments: &'a Instrument<'a>,
    accept_datetime_format: Option<DateTimeFormat>,
    price: Option<PricingComponent>,
    granularity: Option<CandlestickGranularity>,
    count: Option<u32>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    smooth: Option<bool>,
    include_first: Option<bool>,
    daily_alignment: Option<u8>,
    alignment_timezone: Option<&'a str>,
    weekly_alignment: Option<DayOfWeek>,
}

impl<'a> CandleStickRequest<'a> {
    builder_methods!([
        accept_datetime_format: DateTimeFormat,
        "Format of DateTime fields in the request and response.",
        price: PricingComponent,
        "The Price component(s) to get candlestick data for. [default=M] \
        A string of any combination of M, B and A \
        * M = Mid - The midpoint between bid and ask \
        * B = Bid - The price we can buy/long at \
        * A = Ask - The price we can sell/short at \
        ",
        granularity: CandlestickGranularity,
        "The granularity of the candlesticks to fetch [default=S5]",
        count: u32,
        "The number of candlesticks to return in the response. Count should not \
             be specified if both the start and end parameters are provided, as the \
             time range combined with the granularity will determine the number of \
             candlesticks to return. [default=500, maximum=5000]",
        from: DateTime<Utc>,
        "The start of the time range to fetch candlesticks for.",
        to: DateTime<Utc>,
        "The end of the time range to fetch candlesticks for.",
        smooth: bool,
        "A flag that controls whether the candlestick is “smoothed” or not. A \
             smoothed candlestick uses the previous candle’s close price as its open \
             price, while an un-smoothed candlestick uses the first price from its \
             time range as its open price. [default=False]",
        include_first: bool,
        "A flag that controls whether the candlestick that is covered by the \
             from time should be included in the results. This flag enables clients \
             to use the timestamp of the last completed candlestick received to poll \
             for future candlesticks but avoid receiving the previous candlestick \
             repeatedly. [default=True]",
        daily_alignment: u8,
        "The hour of the day (in the specified timezone) to use for granularities \
             that have daily alignments. [default=17, minimum=0, maximum=23]",
        alignment_timezone: &'a str,
        "The timezone to use for the dailyAlignment parameter. Candlesticks with \
             daily alignment will be aligned to the dailyAlignment hour within the \
             alignmentTimezone. Note that the returned times will still be \
             represented in UTC. [default=America/New_York]",
        weekly_alignment: DayOfWeek,
        "The day of the week used for granularities that have weekly \
             alignment. [default=Friday]",
    ]);

    pub async fn send(&self) -> Result<model::candle::CandleResponse, Error> {
        let path = format!("/v3/instruments/{}/candles", self.instruments.instrument);
        let url = self.instruments.client.url(&path);
        let request = self.instruments.client.start_get(&url).query(self);
        debug!("Get candles request: {request:#?}");
        self.instruments
            .client
            .get(request)
            .await
            .attach_printable_lazy(|| format!("With these params: {:?}", self))
    }

    fn new(instruments: &'a Instrument) -> CandleStickRequest<'a> {
        CandleStickRequest {
            instruments,
            accept_datetime_format: None,
            price: None,
            granularity: None,
            count: None,
            from: None,
            to: None,
            smooth: None,
            include_first: None,
            daily_alignment: None,
            alignment_timezone: None,
            weekly_alignment: None,
        }
    }
}

impl<'a> fmt::Debug for CandleStickRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CandleStickRequest")
            .field("accept_datetime_format", &self.accept_datetime_format)
            .field("price", &self.price)
            .field("granularity", &self.granularity)
            .field("count", &self.count)
            .field("from", &self.from)
            .field("to", &self.to)
            .field("smooth", &self.smooth)
            .field("include_first", &self.include_first)
            .field("daily_alignment", &self.daily_alignment)
            .field("alignment_timezone", &self.alignment_timezone)
            .field("weekly_alignment", &self.weekly_alignment)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};
    use std::env::var;

    use crate::{client::Client, model::candle::CandlestickGranularity};

    #[tokio::test]
    async fn candles() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        let eur_usd = client.instrument("EUR_USD");
        let request = eur_usd.candles();
        let candles = request.send().await.unwrap();
        dbg!(candles);
    }

    #[tokio::test]
    async fn candles_count() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        let eur_usd = client.instrument("EUR_USD");
        let request = eur_usd
            .candles()
            .count(5)
            .granularity(CandlestickGranularity::H1);
        let candles = request.send().await.unwrap();
        assert_eq!(candles.candles.len(), 5);
        assert_eq!(candles.granularity, CandlestickGranularity::H1);
    }

    #[tokio::test]
    async fn candles_date_range() {
        let api_key =
            var("OANDA_TOKEN").expect("expected OANDA_TOKEN environment variable to be set");
        let client = Client::new(api_key, crate::host::Host::Dev);
        let eur_usd = client.instrument("EUR_USD");
        let start_date = Utc.with_ymd_and_hms(2022, 2, 14, 0, 0, 0).single().unwrap();
        let end_date = Utc.with_ymd_and_hms(2022, 2, 19, 0, 0, 0).single().unwrap();
        let request = eur_usd
            .candles()
            .granularity(CandlestickGranularity::D)
            .from(start_date)
            .alignment_timezone("UTC")
            .daily_alignment(01)
            .include_first(false)
            .to(end_date);
        let candles = request.send().await.unwrap();
        dbg!(&candles);
        assert_eq!(candles.candles.len(), 5);
        assert_eq!(candles.granularity, CandlestickGranularity::D);
        for candle in &candles.candles {
            assert!(
                candle.time > start_date,
                "Candle: {:?} - {start_date}",
                candle
            );
            assert!(candle.time <= end_date, "Candle: {:?} - {end_date}", candle);
        }
        dbg!(candles);
    }
}

pub use crate::model;
use crate::{builder_methods, client::Client, error::Error};
use chrono::{DateTime, Utc};
use error_stack::{Result, ResultExt};
use serde::Serialize;
use std::fmt;

use self::model::{
    candle::CandlestickGranularity, date_time::DateTimeFormat, instrument::DayOfWeek,
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
    /// Name of the Instrument [required]
    #[serde(skip)]
    instruments: &'a Instrument<'a>,

    /// Format of DateTime fields in the request and response.
    accept_datetime_format: Option<DateTimeFormat>,

    /// The Price component(s) to get candlestick data for. [default=M]
    price: Option<f32>,

    /// The granularity of the candlesticks to fetch [default=S5]
    granularity: Option<CandlestickGranularity>,

    /// The number of candlesticks to return in the response.
    /// Count should not be specified if both the start and end parameters are provided,
    /// as the time range combined with the granularity will determine the number of candlesticks to return. [default=500, maximum=5000]
    count: Option<u32>,

    /// The start of the time range to fetch candlesticks for.
    from: Option<DateTime<Utc>>,

    /// The end of the time range to fetch candlesticks for.
    to: Option<DateTime<Utc>>,

    /// A flag that controls whether the candlestick is “smoothed” or not.
    /// A smoothed candlestick uses the previous candle’s close price as its open price,
    /// while an un-smoothed candlestick uses the first price from its time range as its open price. [default=False]
    smooth: Option<bool>,

    /// A flag that controls whether the candlestick that is covered by the from time should be included in the results.
    /// This flag enables clients to use the timestamp of the last completed candlestick received to poll for future candlesticks but avoid receiving the previous candlestick repeatedly. [default=True]
    include_first: Option<bool>,

    /// The hour of the day (in the specified timezone) to use for granularities that have daily alignments. [default=17, minimum=0, maximum=23]
    daily_alignment: Option<u8>,

    /// The timezone to use for the dailyAlignment parameter. Candlesticks with daily alignment will be aligned to the dailyAlignment hour within the alignmentTimezone.
    /// Note that the returned times will still be represented in UTC. [default=America/New_York]
    alignment_timezone: Option<&'a str>,

    /// The day of the week used for granularities that have weekly alignment. [default=Friday]
    weekly_alignment: Option<DayOfWeek>,
}

impl<'a> CandleStickRequest<'a> {
    builder_methods!([
        accept_datetime_format: DateTimeFormat,
        price: f32,
        granularity: CandlestickGranularity,
        count: u32,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        smooth: bool,
        include_first: bool,
        daily_alignment: u8,
        alignment_timezone: &'a str,
        weekly_alignment: DayOfWeek,
    ]);

    pub async fn send(&self) -> Result<model::candle::CandleResponse, Error> {
        let path = format!("/v3/instruments/{}/candles", self.instruments.instrument);
        let url = self.instruments.client.url(&path);
        let request = self.instruments.client.start_get(&url).query(self);
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

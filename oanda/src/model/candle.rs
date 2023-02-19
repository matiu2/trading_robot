use chrono::{DateTime, Utc};
use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Display, Debug)]
pub enum CandleType {
    #[display("M")]
    Midpoint,
    #[display("B")]
    Bid,
    #[display("A")]
    Ask,
    #[display("MB")]
    MidpointAndBid,
    #[display("MA")]
    MidpointAndAsk,
    #[display("BA")]
    BidAndAsk,
    #[display("MBA")]
    All,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    /// The start time of the candlestick
    pub time: DateTime<Utc>,

    /// The candlestick data based on bids. Only provided if bid-based candles
    /// were requested.
    pub bid: Option<CandlestickData>,

    /// The candlestick data based on asks. Only provided if ask-based candles
    /// were requested.
    pub ask: Option<CandlestickData>,

    /// The candlestick data based on midpoints. Only provided if midpoint-based
    /// candles were requested.
    pub mid: Option<CandlestickData>,

    /// The number of prices created during the time-range represented by the
    /// candlestick.
    pub volume: i32,

    /// A flag indicating if the candlestick is complete. A complete candlestick
    /// is one whose ending time is not in the future.
    pub complete: bool,
}

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlestickData {
    #[serde_as(as = "DisplayFromStr")]
    /// The first (open) price in the time-range represented by the candlestick.
    pub o: f32,

    #[serde_as(as = "DisplayFromStr")]
    /// The highest price in the time-range represented by the candlestick.
    pub h: f32,

    #[serde_as(as = "DisplayFromStr")]
    /// The lowest price in the time-range represented by the candlestick.
    pub l: f32,

    #[serde_as(as = "DisplayFromStr")]
    /// The last (closing) price in the time-range represented by the
    /// candlestick.
    pub c: f32,
}

#[derive(Display, FromStr, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[display(style = "UPPERCASE")]
pub enum CandlestickGranularity {
    /// 5 second candlesticks, minute alignment
    S5,
    /// 10 second candlesticks, minute alignment
    S10,
    /// 15 second candlesticks, minute alignment
    S15,
    /// 30 second candlesticks, minute alignment
    S30,
    /// 1 minute candlesticks, minute alignment
    M1,
    /// 2 minute candlesticks, hour alignment
    M2,
    /// 4 minute candlesticks, hour alignment
    M4,
    /// 5 minute candlesticks, hour alignment
    M5,
    /// 10 minute candlesticks, hour alignment
    M10,
    /// 15 minute candlesticks, hour alignment
    M15,
    /// 30 minute candlesticks, hour alignment
    M30,
    /// 1 hour candlesticks, hour alignment
    H1,
    /// 2 hour candlesticks, day alignment
    H2,
    /// 3 hour candlesticks, day alignment
    H3,
    /// 4 hour candlesticks, day alignment
    H4,
    /// 6 hour candlesticks, day alignment
    H6,
    /// 8 hour candlesticks, day alignment
    H8,
    /// 12 hour candlesticks, day alignment
    H12,
    /// 1 day candlesticks, day alignment
    D,
    /// 1 week candlesticks, aligned to start of week
    W,
    /// 1 month candlesticks, aligned to first day of the month
    M,
}

#[derive(Debug, Deserialize)]
pub struct CandleResponse {
    pub instrument: String,
    pub granularity: CandlestickGranularity,
    pub candles: Vec<Candle>,
}

use algorithms::{
    pivots, Atr, IntoRenkoIterator, IntoSupportAndResistance, IntoSwingStatusIter, RenkoCandle,
    SupportAndResistance,
};
use error_stack::{bail, report, Result, ResultExt};
use oanda::{
    client::instrument::Instrument,
    host::Host::Dev,
    model::{candle::CandlestickGranularity as Granularity, instrument::PricingComponent, Candle},
    Client,
};
use std::env;
mod error;
use error::Error;
use tracing::{debug, info, instrument};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up the subscriber with the environment filter and a formatter.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Get a list of open trades
    trade("EUR_USD")
        .await
        .attach_printable_lazy(|| "Instrument: eur_usd")?;
    Ok(())
}

#[instrument]
async fn trade(instrument: &str) -> Result<(), Error> {
    info!("trade start");
    let token = env::var("OANDA_TOKEN").expect("No OANDA_TOKEN environment variable");
    let client = Client::new(token, Dev);
    // Ask for the last candle so we can get the latest bid and ask prices to decide whether to enter the trade or not
    // We're doing it in the background, because I wanted to have the information ready
    // TODO: After consideration, it's probably better and easier to just wait for the last candle at the end
    let last_candle_handle = {
        let client = client.clone();
        let instrument = instrument.to_owned();
        tokio::spawn(async move {
            let eur_usd = client.instrument(instrument);
            eur_usd
                .candles()
                .granularity(Granularity::S5)
                .count(2)
                .price(PricingComponent::default().bid().ask())
                .build()
                .send()
                .await
                .change_context(Error::new("Couldn't get the last candle"))
        })
    };
    // Get 200 historic candles (the maximum the API allows)
    debug!("Getting candles");
    let eur_usd = client.instrument(instrument);
    let response = eur_usd
        .candles()
        .granularity(Granularity::M15)
        .count(200)
        .build()
        .send()
        .await
        .change_context(Error::new("Couldn't download the candles"))?;
    // Get the 14 ATR
    let Some(atr) = response.candles[(response.candles.len() - 14)..]
        .iter()
        .atr() else { bail!(Error::new("Unable to calculate atr for {instrument}."))};
    debug!("atr: {atr:#?}");

    let (support, resistance) = support_and_resistance(&eur_usd, response.candles, atr).await?;
    debug!("support: {support:#?} resistance: {resistance:#?}");

    // Now we have our support and resistance, get the last candle with bid and ask prices to see what we're risking
    let Some(last_candle) = last_candle_handle
        .await
        .map_err(|err| {
            Error::new(format!(
                "Unable to join task that waited for the last candle: {err:#?}"
            ))
        })??
        .candles.into_iter().last() else { bail!(Error::new("Asked for the last candle and got noting"))};
    let Some(gap) = last_candle.bid.as_ref().zip(last_candle.ask.as_ref()).map(|(bid, ask)| ask.c - bid.c) else { return Err(report!(Error::new("last_candle doesn't have bid and ask prices")).attach_printable("last_candle:#?"))};
    debug!(
        "Gap is {gap}. ATR is {atr}. Gap is {}% of ATR",
        gap / atr * 100.0
    );
    // TODO: Find a percent for cutoff. If the gap is too big, don't trade.
    // See if we want to buy or sell
    // If the current price is less than one ATR over support buy
    debug!("last_candle: {last_candle:#?}");
    let Some(last_buy_price) = last_candle.bid.as_ref().map(|bid| bid.c) else {
        return Err(report!(Error::new("The last candle doesn't have a close bid price"))
            .attach_printable(format!("Last candle: {last_candle:#?}")));
    };
    debug!("last_buy_price: {last_buy_price:#?}\nresistance: {resistance:#?}");
    if last_buy_price > resistance && last_buy_price < resistance + atr {
        info!("Buying")
    }
    // todo!("Sell");
    Ok(())
}

/// Returns support and resistance lines given some candles
///
/// Uses the instrument client to get more candes if more are needed
async fn support_and_resistance(
    instrument: &Instrument<'_>,
    mut normal_candles: Vec<Candle>,
    atr: f32,
) -> Result<(f32, f32), Error> {
    // We'll keep looping until we get support and resistance lines
    // NOTE: Consider turning the 200 candles thing into a stream
    // NOTE: Maybe we don't want to just throw away the candles ?
    loop {
        // Turn the candles into renko candles
        let candles: Vec<RenkoCandle> = normal_candles
            .iter()
            .flat_map(|candle| candle.mid.as_ref().map(|mid| mid.c))
            .renko(atr)
            .collect();
        debug!("renko: {candles:#?}");
        // Run higher high, lower low
        let pivots = pivots(candles.as_slice(), 5);
        debug!("pivots: {:#?}", pivots.clone().collect::<Vec<_>>());
        let SupportAndResistance {
            support,
            resistance,
        } = pivots.into_iter().high_low_swing().support_and_resistance();
        if let Some((support, resistance)) = support.zip(resistance) {
            // If we have support and resistance lines, let's go
            break Ok((support, resistance));
        }
        // If we don't have support and resistance lines, go back and get another 200 candles
        debug!(
            "Getting more candles. Currently have {}",
            normal_candles.len()
        );
        // Get the open time from the first candle we have, and ask for candles before that
        let Some(first_candle) = normal_candles.first() else {bail!(Error::new("Couldn't even get the first candle"))};
        let end_time = first_candle.time;
        let mut new_candles = instrument
            .candles()
            .to(end_time)
            .count(200)
            .build()
            .send()
            .await
            .change_context(Error::new("Couldn't download subsequent candles"))?
            .candles;
        debug_assert_ne!(new_candles.last(), normal_candles.first(), "You shouldn't have a duplicate candle in there, delete the last candle from what you receive. Maybe try .include_first(false)");
        new_candles.extend(normal_candles);
        normal_candles = new_candles;
    }
}

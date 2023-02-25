//! Turns an iterator of candles into an iterator of their true_range values.
//! Consumes the first candle for accuracy rather than making up the first value

use std::ops::Deref;

/// Impl this trait for your data to get an TR iterator for it
trait TRCandle {
    fn high(&self) -> f32;
    fn low(&self) -> f32;
    fn close(&self) -> f32;
    /// Calculates the true_range from the previous close
    fn true_range(&self, previous_close: f32) -> f32 {
        // The difference between the day's high and the day's low.
        let hl = self.high() - self.low();
        // The absolute value of the difference between the previous day's close and the current day's high.
        let hpc = (self.high() - previous_close).abs();
        // The absolute value of the difference between the previous day's close and the current day's low.
        let lpc = (self.low() - previous_close).abs();
        // The True Range is calculated as the greatest of these values.
        dbg!(self.close(), previous_close, hl, hpc, lpc);
        dbg!(hl.max(hpc).max(lpc))
    }
}

impl<T, C> TRCandle for T
where
    T: Deref<Target = C>,
    C: TRCandle,
{
    fn high(&self) -> f32 {
        self.deref().high()
    }

    fn low(&self) -> f32 {
        self.deref().low()
    }

    fn close(&self) -> f32 {
        self.deref().close()
    }
}

/// Turn an Iterator of TRCandle into an Iterator of the actual true range values
trait TrueRange<I, C>
where
    I: Iterator<Item = C>,
    C: TRCandle,
{
    /// Take an iterator of `TRCandle` and get an iterator of the actual TR values.
    fn true_range(self) -> TRIter<I, C>;
}

/// The underlying struct that enables our Iterator
struct TRIter<I, C>
where
    I: Iterator<Item = C>,
    C: TRCandle,
{
    iter: I,
    previous_close: Option<f32>,
}

impl<I, C> TrueRange<I, C> for I
where
    I: Iterator<Item = C>,
    C: TRCandle,
{
    fn true_range(self) -> TRIter<I, C> {
        TRIter::new(self)
    }
}

impl<I, C> TRIter<I, C>
where
    I: Iterator<Item = C>,
    C: TRCandle,
{
    fn new(iter: I) -> Self {
        Self {
            iter,
            previous_close: None,
        }
    }
}

impl<I, C> Iterator for TRIter<I, C>
where
    I: Iterator<Item = C>,
    C: TRCandle,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let previous_close = self.previous_close;
            if let Some(candle) = self.iter.next() {
                self.previous_close = Some(candle.close());
                if let Some(previous_close) = previous_close {
                    break Some(candle.true_range(previous_close));
                }
            } else {
                break None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TRCandle, TrueRange};
    use rand::Rng;

    #[derive(Debug, Clone)]
    struct Candle {
        high: f32,
        low: f32,
        close: f32,
        tr: Option<f32>,
    }

    impl TRCandle for Candle {
        fn high(&self) -> f32 {
            self.high
        }

        fn low(&self) -> f32 {
            self.low
        }

        fn close(&self) -> f32 {
            self.close
        }
    }

    fn generate_candles(n: usize) -> Vec<Candle> {
        let mut rng = rand::thread_rng();
        let mut prev_close = rng.gen_range(1.0..100.0);

        let candles = std::iter::repeat_with(|| {
            let high: f32 = rng.gen_range(prev_close..(prev_close + 10.0));
            let low: f32 = rng.gen_range((prev_close - 10.0)..prev_close);
            let close: f32 = rng.gen_range(low..high);
            let tr = Some(
                vec![
                    high - low,
                    (high - prev_close).abs(),
                    (low - prev_close).abs(),
                ]
                .iter()
                .fold(0.0, |a: f32, &b| a.max(b)),
            );
            prev_close = close;
            Candle {
                high,
                low,
                close,
                tr,
            }
        })
        .take(n)
        .collect();

        candles
    }

    #[test]
    fn test_true_range() {
        let candles = generate_candles(20);
        let mut previous_close = None;
        for (n, (candle, tr)) in candles
            .iter()
            .skip(1)
            .zip(candles.iter().true_range())
            .enumerate()
        {
            assert_eq!(
                candle.tr,
                Some(tr),
                "n: {}, pc: {:?} candle: {:?}, tr: {tr}\nh-l={}\nh-pc={}\nl-pc={}",
                n + 1,
                previous_close,
                &candle,
                candle.high - candle.low,
                (candle.high - previous_close.unwrap_or(0.0)).abs(),
                (candle.low - previous_close.unwrap_or(0.0)).abs(),
            );
            previous_close = Some(candle.close);
        }
    }

    #[test]
    fn test_empty_iterator() {
        let candles: Vec<Candle> = vec![];
        let mut true_ranges = candles.iter().true_range();
        assert_eq!(true_ranges.next(), None);
    }

    #[test]
    fn test_true_range_same_values() {
        let candles = vec![
            Candle {
                high: 10.0,
                low: 10.0,
                close: 10.0,
                tr: None,
            },
            Candle {
                high: 10.0,
                low: 10.0,
                close: 10.0,
                tr: Some(0.0),
            },
            Candle {
                high: 10.0,
                low: 10.0,
                close: 10.0,
                tr: Some(0.0),
            },
        ];

        let mut previous_close = None;
        for (n, (candle, tr)) in candles
            .iter()
            .skip(1)
            .zip(candles.iter().true_range())
            .enumerate()
        {
            assert_eq!(
                candle.tr,
                Some(tr),
                "n: {}, pc: {:?} candle: {:?}, tr: {tr}",
                n + 1,
                previous_close,
                &candle,
            );
            previous_close = Some(candle.close);
        }
    }

    #[test]
    fn test_single_candle() {
        // Test that a single candle with no previous close has a true range of None.
        let candles = vec![Candle {
            high: 10.0,
            low: 5.0,
            close: 8.0,
            tr: None,
        }];
        let mut iter = candles.iter().true_range();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn two_candles() {
        // Test that when two consecutive candles return a single value
        // because the first candle is consumed
        let candles = vec![
            Candle {
                high: 20.0,
                low: 10.0,
                close: 15.0,
                tr: None,
            },
            Candle {
                high: 18.0,
                low: 13.0,
                close: 15.0,
                tr: Some(0.0), // hl=5;hc=3;lc=2
            },
        ];
        let mut iter = candles.iter().true_range();
        assert_eq!(iter.next(), Some(5.0)); // tr of second candle
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_same_values() {
        // Test that calling true_range() on an iterator where all candles have the same high, low,
        // and close values returns an iterator with all values equal to zero.
        let candle = Candle {
            high: 10.0,
            low: 10.0,
            close: 10.0,
            tr: None,
        };
        let candles = std::iter::repeat(candle).take(5);
        let mut iter = candles.true_range();
        for tr in iter.by_ref().take(3) {
            assert_eq!(tr, 0.0);
        }
    }
}

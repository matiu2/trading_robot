use crate::{TRCandle, TrueRange};

/// All iterators over f32 get an average function
pub trait Average {
    fn average(self) -> Option<f32>;
}

impl<I> Average for I
where
    I: Iterator<Item = f32>,
{
    fn average(self) -> Option<f32> {
        let (sum, count) = self.fold((0.0, 0), |(sum, count), item| (sum + item, count + 1));
        if count > 0 {
            Some(sum / count as f32)
        } else {
            None
        }
    }
}

/// Iterators over TRCandle get an `atr` function
pub trait Atr {
    fn atr(self) -> Option<f32>;
}

impl<I, C> Atr for I
where
    I: Iterator<Item = C>,
    C: TRCandle,
{
    fn atr(self) -> Option<f32> {
        self.true_range().average()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Candle {
        high: f32,
        low: f32,
        close: f32,
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

    #[test]
    fn test_average_non_empty() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(values.iter().copied().average(), Some(3.0));
    }

    #[test]
    fn test_average_empty() {
        let values: Vec<f32> = vec![];
        assert_eq!(values.iter().copied().average(), None);
    }

    #[test]
    fn test_atr_non_empty() {
        let candles = vec![
            Candle {
                high: 20.0,
                low: 10.0,
                close: 12.0,
            },
            Candle {
                high: 32.0,
                low: 15.0,
                close: 28.0,
            },
            Candle {
                high: 44.0,
                low: 36.0,
                close: 40.0,
            },
            Candle {
                high: 51.0,
                low: 43.0,
                close: 45.0,
            },
            Candle {
                high: 62.0,
                low: 58.0,
                close: 60.0,
            },
        ];
        dbg!(candles.iter().true_range().collect::<Vec<f32>>());
        assert_eq!(candles.into_iter().atr(), Some(16.0));
    }

    #[test]
    fn test_atr_empty() {
        let candles: Vec<Candle> = vec![];
        assert_eq!(candles.into_iter().atr(), None);
    }
}

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
    use crate::candle::test_data::{test_data_1, test_data_2, test_data_3, Candle};
    use crate::candle::{Close, High, Low};

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
    fn test_atr_1() {
        let candles = test_data_1();
        assert_eq!(
            candles.iter().true_range().collect::<Vec<f32>>(),
            [6.0, 4.0, 4.0, 5.0, 5.0, 4.0, 5.0, 6.0]
        );
        assert_eq!(candles.into_iter().atr(), Some(4.875));
    }

    #[test]
    fn test_atr_2() {
        let candles = test_data_2();
        assert_eq!(
            candles.iter().true_range().collect::<Vec<f32>>(),
            [7.0, 6.0, 10.0, 8.0, 7.0, 7.0, 7.0, 7.0, 12.0]
        );
        assert_eq!(candles.into_iter().atr(), Some(7.888889));
    }

    #[test]
    fn test_atr_3() {
        let candles = test_data_3();
        assert_eq!(
            candles.iter().true_range().collect::<Vec<f32>>(),
            [4.0, 3.0, 4.0, 3.0, 4.0, 3.0, 4.0, 4.0, 4.0, 3.0, 4.0]
        );
        assert_eq!(candles.into_iter().atr(), Some(3.6363636363636362));
    }

    #[test]
    fn test_atr_empty() {
        let candles: Vec<Candle> = vec![];
        assert_eq!(candles.into_iter().atr(), None);
    }
}

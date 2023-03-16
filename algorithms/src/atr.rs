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

    #[test]
    fn test_average_non_empty() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(values.iter().copied().average(), Some(3.0));
    }

    #[test]
    fn test_average_empty() {
        let values: Vec<f32> = vec![];
        assert_eq!(None, values.iter().copied().average());
    }

    #[test]
    fn test_atr_1() {
        let candles = test_data_1();
        assert_eq!(
            vec![5.0, 6.0, 4.0, 4.0, 5.0, 5.0, 4.0, 5.0, 6.0],
            candles.iter().true_range().collect::<Vec<f32>>()
        );
        assert_eq!(Some(4.888889), candles.into_iter().atr());
    }

    #[test]
    fn test_atr_2() {
        let candles = test_data_2();
        assert_eq!(
            vec![10.0, 7.0, 6.0, 10.0, 8.0, 7.0, 7.0, 7.0, 7.0, 12.0],
            candles.iter().true_range().collect::<Vec<f32>>()
        );
        assert_eq!(Some(8.1), candles.into_iter().atr());
    }

    #[test]
    fn test_atr_3() {
        let candles = test_data_3();
        assert_eq!(
            vec![4.0, 4.0, 3.0, 4.0, 3.0, 4.0, 3.0, 4.0, 4.0, 4.0, 3.0, 4.0],
            candles.iter().true_range().collect::<Vec<f32>>(),
        );
        assert_eq!(Some(3.6666667), candles.into_iter().atr());
    }

    #[test]
    fn test_atr_empty() {
        let candles: Vec<Candle> = vec![];
        assert_eq!(candles.into_iter().atr(), None);
    }
}

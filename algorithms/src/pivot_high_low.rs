use crate::candle::{High, Low};
use std::fmt::Debug as Dbg;

#[derive(Debug, PartialEq, Clone)]
pub enum Pivot {
    High(f32),
    Low(f32),
    NoChange,
}

/// Finds pivot points in a slice of types implementing `High` and `Low`.
/// A pivot is defined as a point in the slice where a certain number of
/// candles before and after the middle candle are lower or higher than
/// the middle candle.
///
/// This takes a slice rather than an iterator because it's more efficient
/// to get at the Windows that we need
///
/// # Arguments
///
/// * `input` - A reference to a slice of types implementing `High` and `Low`.
/// * `window_size` - The size of the window around each candle to consider.
///
pub fn pivots<'a>(
    input: &'a [impl High + Low + Dbg],
    window_size: usize,
) -> impl Iterator<Item = Pivot> + 'a {
    (0..input.len()).map(move |i| {
        // Make the output length be the same as the input
        if i < window_size / 2 || i >= input.len() - window_size / 2 {
            return Pivot::NoChange;
        }

        let window = &input[i - window_size / 2..i + window_size / 2 + 1];
        let (before, after) = window.split_at(window_size / 2);
        let mid = &after[0];
        let mid_high = mid.high();
        let after = &after[1..];

        if before.iter().all(|x| mid_high > x.high()) && after.iter().all(|x| mid_high > x.high()) {
            Pivot::High(mid_high)
        } else {
            let mid_low = mid.low();
            if before.iter().all(|x| mid_low < x.low()) && after.iter().all(|x| mid_low < x.low()) {
                Pivot::Low(mid_low)
            } else {
                Pivot::NoChange
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::{pivots, Pivot};
    use crate::candle::test_data::{test_data_1, test_data_2};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_1() {
        let data = test_data_1();
        let pivots = pivots(data.as_slice(), 5);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::Low(4.0),
            Pivot::NoChange,
            Pivot::High(11.0),
            Pivot::Low(3.0),
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
        ];
        assert_eq!(expected, pivots.collect::<Vec<_>>());
    }

    #[test]
    fn test_2() {
        let data = test_data_2();
        let pivots = pivots(data.as_slice(), 5);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::High(18.0),
            Pivot::Low(5.0),
            Pivot::NoChange,
            Pivot::High(16.0),
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
        ];
        assert_eq!(pivots.collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_2b() {
        let data = test_data_2();
        let pivots = pivots(data.as_slice(), 3);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::Low(6.0),
            Pivot::High(18.0),
            Pivot::Low(5.0),
            Pivot::NoChange,
            Pivot::High(16.0),
            Pivot::NoChange,
            Pivot::Low(4.0),
            Pivot::NoChange,
        ];
        assert_eq!(pivots.collect::<Vec<_>>(), expected);
    }
}

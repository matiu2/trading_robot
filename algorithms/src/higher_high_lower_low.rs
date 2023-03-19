use crate::Pivot;

/// Represents the four possible types of high-low swings in a series of pivots:
/// - A higher high; creates a new higher resistance
/// - A lower low; creates a new lower resistance
/// - A lower high; creates a new higher support line
/// - A higher low; creates a new lower support line
#[derive(Debug, PartialEq)]
pub enum SwingType {
    HigherHigh,
    LowerLow,
    LowerHigh,
    HigherLow,
    // The below options are needed for when a candle is really tall,
    HigherHighAndHigherLow,
    HigherHighAndLowerLow,
    LowerHighAndHigherLow,
    LowerHighAndLowerLow,
    Hold,
}

#[derive(Debug, PartialEq)]
pub struct SwingStatus {
    swing_type: SwingType,
    support: Option<f32>,
    resistance: Option<f32>,
}

impl SwingStatus {
    pub fn new(swing_type: SwingType, support: Option<f32>, resistance: Option<f32>) -> Self {
        SwingStatus {
            swing_type,
            support,
            resistance,
        }
    }
}

/// Takes a list of high/low pivots and generates support and resistance lines from them
pub struct SwingStatusIter<I> {
    input: I,
    prev_high: Option<f32>,
    prev_low: Option<f32>,
    support: Option<f32>,
    resistance: Option<f32>,
}

impl<I> SwingStatusIter<I>
where
    I: Iterator<Item = Pivot>,
{
    /// Creates a new instance of `SwingStatusIter` with the given iterator of pivots
    pub fn new(input: I) -> Self {
        SwingStatusIter {
            input,
            prev_high: None,
            prev_low: None,
            support: None,
            resistance: None,
        }
    }
}

impl<I> Iterator for SwingStatusIter<I>
where
    I: Iterator<Item = Pivot>,
{
    type Item = SwingStatus;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.input.next()?;

        let high = input.high();
        let low = input.low();
        let prev_high = self.prev_high;
        let prev_low = self.prev_low;

        match (high, self.prev_high) {
            (Some(high), None) => self.prev_high = Some(high),
            (Some(high), Some(_prev)) => {
                self.prev_high = Some(high);
                self.resistance = Some(high);
            }
            _ => (),
        };

        match (low, self.prev_low) {
            (Some(low), None) => self.prev_low = Some(low),
            (Some(low), Some(_prev)) => {
                self.prev_low = Some(low);
                self.support = Some(low);
            }
            _ => (),
        };

        let support = self.support;
        let resistance = self.resistance;
        let status = |swing_type| Some(SwingStatus::new(swing_type, support, resistance));

        let is_higher_high = high
            .zip(prev_high)
            .map(|(high, prev_high)| high >= prev_high)
            .unwrap_or(false);
        let is_lower_low = low
            .zip(prev_low)
            .map(|(low, prev_low)| low <= prev_low)
            .unwrap_or(false);

        if high.is_some() && prev_high.is_some() {
            if low.is_some() && prev_low.is_some() {
                match (is_higher_high, is_lower_low) {
                    (true, true) => status(SwingType::HigherHighAndLowerLow),
                    (true, false) => status(SwingType::HigherHighAndHigherLow),
                    (false, true) => status(SwingType::LowerHighAndLowerLow),
                    (false, false) => status(SwingType::LowerHighAndHigherLow),
                }
            } else if is_higher_high {
                status(SwingType::HigherHigh)
            } else {
                status(SwingType::LowerHigh)
            }
        } else if low.is_some() && prev_low.is_some() {
            if is_lower_low {
                status(SwingType::LowerLow)
            } else {
                status(SwingType::HigherLow)
            }
        } else {
            status(SwingType::Hold)
        }
    }
}

pub trait IntoSwingStatusIter: Iterator<Item = Pivot> {
    fn high_low_swing(self) -> SwingStatusIter<Self>
    where
        Self: Sized,
    {
        SwingStatusIter::new(self)
    }
}

impl<I> IntoSwingStatusIter for I where I: Iterator<Item = Pivot> {}

#[cfg(test)]
mod tests {
    use crate::{candle::test_data::Candle, pivots};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn high_low_swing_iter() {
        let pivots = vec![
            Pivot::NoChange,
            Pivot::High(100.0),
            Pivot::NoChange,
            Pivot::Low(90.0),
            Pivot::NoChange,
            Pivot::High(120.0),
            Pivot::NoChange,
            Pivot::Low(80.0),
            Pivot::NoChange,
            Pivot::Low(70.0),
            Pivot::High(110.0),
            Pivot::Low(60.0),
            Pivot::High(140.0),
            Pivot::Low(50.0),
            Pivot::High(130.0),
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
        ];

        let high_low_swing_iter = SwingStatusIter::new(pivots.into_iter());
        let high_low_swing_vec: Vec<_> = high_low_swing_iter.collect();

        let expected_high_low_swing = vec![
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            // Pivot::High(100.0),
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            // Pivot::Low(90.0),
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            // Pivot::High(120.0), Higher high sets a resistance line
            SwingStatus {
                swing_type: SwingType::HigherHigh,
                support: None,
                resistance: Some(120.0),
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: Some(120.0),
            },
            // Pivot::Low(80.0), Lower low sets the support line
            SwingStatus {
                swing_type: SwingType::LowerLow,
                support: Some(80.0),
                resistance: Some(120.0),
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: Some(80.0),
                resistance: Some(120.0),
            },
            // Pivot::Low(70.0),
            SwingStatus {
                swing_type: SwingType::LowerLow,
                support: Some(70.0),
                resistance: Some(120.0),
            },
            // Pivot::High(110.0),
            SwingStatus {
                swing_type: SwingType::LowerHigh,
                support: Some(70.0),
                resistance: Some(110.0),
            },
            // Pivot::Low(60.0),
            SwingStatus {
                swing_type: SwingType::LowerLow,
                support: Some(60.0),
                resistance: Some(110.0),
            },
            // Pivot::High(140.0),
            SwingStatus {
                swing_type: SwingType::HigherHigh,
                support: Some(60.0),
                resistance: Some(140.0),
            },
            // Pivot::Low(50.0),
            SwingStatus {
                swing_type: SwingType::LowerLow,
                support: Some(50.0),
                resistance: Some(140.0),
            },
            // Pivot::High(130.0),
            SwingStatus {
                swing_type: SwingType::LowerHigh,
                support: Some(50.0),
                resistance: Some(130.0),
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: Some(50.0),
                resistance: Some(130.0),
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: Some(50.0),
                resistance: Some(130.0),
            },
            // Pivot::NoChange,
            SwingStatus {
                swing_type: SwingType::Hold,
                support: Some(50.0),
                resistance: Some(130.0),
            },
        ];

        assert_eq!(high_low_swing_vec, expected_high_low_swing);
    }

    #[test]
    fn candle_high_low_swing_iter() {
        let candles = vec![
            Candle {
                open: 10.0,
                high: 20.0,
                low: 8.0,
                close: 15.0,
            },
            Candle {
                open: 15.0,
                high: 25.0,
                low: 12.0,
                close: 20.0,
            },
            Candle {
                open: 20.0,
                high: 30.0,
                low: 15.0,
                close: 25.0,
            },
            Candle {
                open: 25.0,
                high: 35.0,
                low: 18.0,
                close: 30.0,
            },
        ];

        let expected_swings = vec![todo!()];

        let pivots = pivots(candles.as_slice(), 5);
        let swings: Vec<SwingStatus> = pivots.high_low_swing().collect();

        assert_eq!(swings, expected_swings);
    }
}

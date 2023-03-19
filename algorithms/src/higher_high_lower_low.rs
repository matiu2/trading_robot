use crate::Pivot;

/// Represents the four possible types of high-low swings in a series of pivots:
#[derive(Debug, PartialEq)]
pub enum SwingType {
    /// A new higher resistance line has been created
    HigherHigh,
    /// A new lower support line has been created
    LowerLow,
    /// A new lower resistance line has been created
    LowerHigh,
    /// A new higher support line has been created
    HigherLow,
    /// A tall candle that sets a new higher resistance and higher support line simultaneously
    HigherHighAndHigherLow,
    /// A tall candle that sets a new higher resistance and lower support line simultaneously
    HigherHighAndLowerLow,
    /// A tall candle that sets a new lower resistance and higher support line simultaneously
    LowerHighAndHigherLow,
    /// A tall candle that sets a new lower resistance and lower support line simultaneously
    LowerHighAndLowerLow,
    /// No significant change in support or resistance levels
    Hold,
}

#[derive(Debug, PartialEq)]
pub struct SwingStatus {
    swing_type: SwingType,
    support: Option<f32>,
    resistance: Option<f32>,
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

    /// If the signal is Pivot::High and its value is greater than the
    /// last high we encountered returns Some(SwingType::HigherHigh).
    /// If the signal is Pivot::HighLow and the previous low is None it
    /// does the same , otherwise returns None
    fn check_hh(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::High(high), Some(prev_high), _) if high >= prev_high => {
                Some(SwingType::HigherHigh)
            }
            (Pivot::HighLow { high, .. }, Some(prev_high), None) if high >= prev_high => {
                Some(SwingType::HigherHigh)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::Low and its value is lower than the
    /// last low we encountered, returns Some(SwingType::LowerLow).
    /// If the signal is Pivot::HighLow and the previous high is None,
    /// it does the same, otherwise returns None.
    fn check_ll(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::Low(low), _, Some(prev_low)) if low <= prev_low => Some(SwingType::LowerLow),
            (Pivot::HighLow { low, .. }, None, Some(prev_low)) if low <= prev_low => {
                Some(SwingType::LowerLow)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::High and its value is lower than the
    /// last high we encountered, returns Some(SwingType::LowerHigh).
    /// If the signal is Pivot::HighLow and the previous low is None,
    /// it does the same, otherwise returns None.
    fn check_lh(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::High(high), Some(prev_high), _) if high < prev_high => {
                Some(SwingType::LowerHigh)
            }
            (Pivot::HighLow { high, .. }, Some(prev_high), None) if high < prev_high => {
                Some(SwingType::LowerHigh)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::Low and its value is higher than the
    /// last low we encountered, returns Some(SwingType::HigherLow).
    /// If the signal is Pivot::HighLow and the previous high is None,
    /// it does the same, otherwise returns None.
    fn check_hl(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::Low(low), _, Some(prev_low)) if low > prev_low => Some(SwingType::HigherLow),
            (Pivot::HighLow { low, .. }, None, Some(prev_low)) if low > prev_low => {
                Some(SwingType::HigherLow)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::HighLow and both high and low values are
    /// outside or equal to the last high and low we encountered, returns
    /// Some(SwingType::HigherHighAndHigherLow), otherwise returns None.
    fn check_hhll(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high >= prev_high && low <= prev_low =>
            {
                Some(SwingType::HigherHighAndLowerLow)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::HighLow and the high is greater than
    /// or equal to the previous high and the low is greater than the
    /// previous low, returns Some(SwingType::HigherHighAndHigherLow),
    /// otherwise returns None.
    fn check_hhhl(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high >= prev_high && low > prev_low =>
            {
                Some(SwingType::HigherHighAndHigherLow)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::HighLow and
    /// and the high and low are within the previous
    /// high and low, returns SwingType::LowerHighAndHigherLow
    fn check_lhhl(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high < prev_high && low > prev_low =>
            {
                Some(SwingType::LowerHighAndHigherLow)
            }
            _ => None,
        }
    }

    /// If the signal is Pivot::HighLow and
    /// and the high is less than the previous high and
    /// the low is less than or equal the previous low,
    /// Returns SwingType::LowerHighAndLowerLow
    fn check_lhll(&self, signal: &Pivot) -> Option<SwingType> {
        match (signal, &self.prev_high, &self.prev_low) {
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high < prev_high && low <= prev_low =>
            {
                Some(SwingType::LowerHighAndLowerLow)
            }
            _ => None,
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

        let swing_type = self
            .check_hh(&input)
            .or_else(|| self.check_lh(&input))
            .or_else(|| self.check_hl(&input))
            .or_else(|| self.check_ll(&input))
            .or_else(|| self.check_hhhl(&input))
            .or_else(|| self.check_hhll(&input))
            .or_else(|| self.check_lhhl(&input))
            .or_else(|| self.check_lhll(&input))
            .unwrap_or(SwingType::Hold);

        // Update our internal state
        let high = input.high();
        let low = input.low();
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
        Some(SwingStatus {
            swing_type,
            support,
            resistance,
        })
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

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_small_1() {
        let pivots = vec![Pivot::NoChange];
        let expected = vec![SwingStatus {
            swing_type: SwingType::Hold,
            support: None,
            resistance: None,
        }];
        let got: Vec<_> = SwingStatusIter::new(pivots.into_iter()).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_small_2() {
        let pivots = vec![Pivot::High(10.0)];
        let mut iter = SwingStatusIter::new(pivots.into_iter());
        assert_eq!(
            Some(SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            }),
            iter.next()
        );
        assert_eq!(
            Some(10.0),
            iter.prev_high,
            "It should update its internal state"
        );
        assert_eq!(None, iter.prev_low);
        assert_eq!(None, iter.support);
        assert_eq!(None, iter.resistance);
    }

    #[test]
    fn test_hh() {
        let pivots = vec![Pivot::High(1.0), Pivot::High(2.0)];
        let expected = vec![
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            SwingStatus {
                swing_type: SwingType::HigherHigh,
                support: None,
                resistance: Some(2.0),
            },
        ];
        let got: Vec<_> = SwingStatusIter::new(pivots.into_iter()).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_hl() {
        let pivots = vec![Pivot::Low(2.0), Pivot::Low(3.0)];
        let expected = vec![
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            SwingStatus {
                swing_type: SwingType::HigherLow,
                support: Some(3.0),
                resistance: None,
            },
        ];
        let got: Vec<_> = SwingStatusIter::new(pivots.into_iter()).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_ll() {
        let pivots = vec![Pivot::Low(2.0), Pivot::Low(1.0)];
        let expected = vec![
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            SwingStatus {
                swing_type: SwingType::LowerLow,
                support: Some(1.0),
                resistance: None,
            },
        ];
        let got: Vec<_> = SwingStatusIter::new(pivots.into_iter()).collect();
        assert_eq!(expected, got);
    }

    #[test]
    fn test_lh() {
        let pivots = vec![Pivot::High(2.0), Pivot::High(1.0)];
        let expected = vec![
            SwingStatus {
                swing_type: SwingType::Hold,
                support: None,
                resistance: None,
            },
            SwingStatus {
                swing_type: SwingType::LowerHigh,
                support: None,
                resistance: Some(1.0),
            },
        ];
        let got: Vec<_> = SwingStatusIter::new(pivots.into_iter()).collect();
        assert_eq!(expected, got);
    }

    fn create_swing_status_iter() -> SwingStatusIter<std::iter::Empty<Pivot>> {
        SwingStatusIter::new(std::iter::empty())
    }

    #[test]
    fn check_hh_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::High(110.0);
        // With no previous it should return None
        assert_eq!(ssi.check_hh(&pivot), None);
        // Positive case
        ssi.prev_high = Some(100.0);
        assert_eq!(ssi.check_hh(&pivot), Some(SwingType::HigherHigh));
        // Other negative case
        let pivot = Pivot::High(90.0);
        assert_eq!(ssi.check_hh(&pivot), None);
        // It should also return a higher high for a tall candle when there's no previous low
        let pivot = Pivot::HighLow {
            high: 101.0,
            low: 10.0,
        };
        ssi.prev_high = Some(100.0);
        ssi.prev_low = None;
        assert_eq!(ssi.check_hh(&pivot), Some(SwingType::HigherHigh));
    }

    #[test]
    fn check_lh_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::High(90.0);
        // With no previous it should return None
        assert_eq!(ssi.check_lh(&pivot), None);
        // Positive case
        ssi.prev_high = Some(100.0);
        assert_eq!(ssi.check_lh(&pivot), Some(SwingType::LowerHigh));
        // Other negative case
        let pivot = Pivot::High(110.0);
        assert_eq!(ssi.check_lh(&pivot), None);
    }

    #[test]
    fn check_ll_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::Low(80.0);
        // With no previous it should return None
        assert_eq!(ssi.check_ll(&pivot), None);
        // Positive case
        ssi.prev_low = Some(90.0);
        assert_eq!(ssi.check_ll(&pivot), Some(SwingType::LowerLow));
        // Other negative case
        let pivot = Pivot::Low(100.0);
        assert_eq!(ssi.check_ll(&pivot), None);
    }

    #[test]
    fn check_hl_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::Low(100.0);
        // With no previous it should return None
        assert_eq!(ssi.check_hl(&pivot), None);
        // Positive case
        ssi.prev_low = Some(90.0);
        assert_eq!(ssi.check_hl(&pivot), Some(SwingType::HigherLow));
        // Other negative case
        let pivot = Pivot::Low(80.0);
        assert_eq!(ssi.check_hl(&pivot), None);
    }

    #[test]
    fn check_hhhl_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::HighLow {
            high: 110.0,
            low: 95.0,
        };
        // With all previous values at None, it should return None
        assert_eq!(ssi.check_hhhl(&pivot), None);
        ssi.prev_high = Some(100.0);
        // With one previous value at None, it should return None
        assert_eq!(ssi.check_hhhl(&pivot), None);
        // With both set accordingly it should return Some
        ssi.prev_low = Some(90.0);
        assert_eq!(
            ssi.check_hhhl(&pivot),
            Some(SwingType::HigherHighAndHigherLow)
        );
        // With incorrect values it should go back the None
        let pivot = Pivot::HighLow {
            high: 90.0,
            low: 100.0,
        };
        assert_eq!(ssi.check_hhhl(&pivot), None);
    }

    #[test]
    fn check_lhhl_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::HighLow {
            high: 95.0,
            low: 95.0,
        };
        // With all previous values at None, it should return None
        assert_eq!(ssi.check_lhhl(&pivot), None);
        ssi.prev_high = Some(100.0);
        // With one previous value at None, it should return None
        assert_eq!(ssi.check_lhhl(&pivot), None);
        // With both set accordingly it should return Some
        ssi.prev_low = Some(90.0);
        assert_eq!(
            ssi.check_lhhl(&pivot),
            Some(SwingType::LowerHighAndHigherLow)
        );
        // With incorrect values it should go back the None
        let pivot = Pivot::HighLow {
            high: 110.0,
            low: 100.0,
        };
        assert_eq!(ssi.check_lhhl(&pivot), None);
    }

    #[test]
    fn check_lhll_test() {
        let mut ssi = create_swing_status_iter();
        let pivot = Pivot::HighLow {
            high: 95.0,
            low: 85.0,
        };
        // With all previous values at None, it should return None
        assert_eq!(ssi.check_lhll(&pivot), None);
        ssi.prev_high = Some(100.0);
        // With one previous value at None, it should return None
        assert_eq!(ssi.check_lhll(&pivot), None);
        // With both set accordingly it should return Some
        ssi.prev_low = Some(90.0);
        assert_eq!(
            ssi.check_lhll(&pivot),
            Some(SwingType::LowerHighAndLowerLow)
        );
        // With incorrect values it should go back the None
        let pivot = Pivot::HighLow {
            high: 110.0,
            low: 100.0,
        };
        assert_eq!(ssi.check_lhll(&pivot), None);
    }

    #[test]
    fn check_hhll_test() {
        let mut ssi = create_swing_status_iter();
        // With all previous values at None, it should return None
        let pivot = Pivot::HighLow {
            high: 110.0,
            low: 80.0,
        };
        assert_eq!(ssi.check_hhll(&pivot), None);
        ssi.prev_high = Some(100.0);
        // With one previous value at None, it should return None
        assert_eq!(ssi.check_hhll(&pivot), None);
        // With both set accordingly it should return Some
        ssi.prev_low = Some(90.0);
        let pivot = Pivot::HighLow {
            high: 110.0,
            low: 80.0,
        };
        assert_eq!(
            ssi.check_hhll(&pivot),
            Some(SwingType::HigherHighAndLowerLow)
        );
        // With incorrect values it should go back the None
        let pivot = Pivot::HighLow {
            high: 90.0,
            low: 100.0,
        };
        assert_eq!(ssi.check_hhll(&pivot), None);
    }
}

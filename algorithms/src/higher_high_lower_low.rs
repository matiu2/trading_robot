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

    /// Detects and returns a `HigherHigh` if the current pivot is a new high and
    /// higher than the previous high.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// high pivot and resistance line.
    fn hh(&mut self, current: f32) -> SwingStatus {
        self.prev_high = Some(current);
        self.resistance = Some(current);
        SwingStatus {
            swing_type: SwingType::HigherHigh,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `LowerLow` if the current pivot is a new low and
    /// lower than the previous low.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// low pivot and support line.
    fn ll(&mut self, current: f32) -> SwingStatus {
        self.prev_low = Some(current);
        self.support = Some(current);
        SwingStatus {
            swing_type: SwingType::LowerLow,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `LowerHigh` if the current pivot is a new high but
    /// lower than the previous high.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// high pivot and support line.
    fn lh(&mut self, current: f32) -> SwingStatus {
        self.prev_high = Some(current);
        self.resistance = Some(current);
        SwingStatus {
            swing_type: SwingType::LowerHigh,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `HigherLow` if the current pivot is a new low but
    /// higher than the previous low.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// low pivot and resistance line.
    fn hl(&mut self, current: f32) -> SwingStatus {
        self.prev_low = Some(current);
        self.support = Some(current);
        SwingStatus {
            swing_type: SwingType::HigherLow,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `HigherHighAndLowerLow` if the current pivot is both
    /// a new high higher than the previous high and a new low lower than the
    /// previous low.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// high and low pivots, as well as the support and resistance lines.
    fn hh_and_ll(&mut self, high: f32, low: f32) -> SwingStatus {
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.resistance = Some(high);
        self.support = Some(low);
        SwingStatus {
            swing_type: SwingType::HigherHighAndLowerLow,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `HigherHighAndHigherLow` if the current pivot is both
    /// a new high higher than the previous high and a new low higher than the
    /// previous low.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// high and low pivots, as well as the support and resistance lines.
    fn hh_and_hl(&mut self, high: f32, low: f32) -> SwingStatus {
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.resistance = Some(high);
        self.support = Some(low);
        SwingStatus {
            swing_type: SwingType::HigherHighAndHigherLow,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `LowerHighAndLowerLow` if the current pivot is both
    /// a new high lower than the previous high and a new low lower than the
    /// previous low.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// high and low pivots, as well as the support and resistance lines.
    fn lh_and_ll(&mut self, high: f32, low: f32) -> SwingStatus {
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.resistance = Some(high);
        self.support = Some(low);
        SwingStatus {
            swing_type: SwingType::LowerHighAndLowerLow,
            support: self.support,
            resistance: self.resistance,
        }
    }

    /// Detects and returns a `LowerHighAndHigherLow` if the current pivot is both
    /// a new high lower than the previous high and a new low higher than the
    /// previous low.
    ///
    /// This also updates the internal state of the struct to track the latest
    /// high and low pivots, as well as the support and resistance lines.
    fn lh_and_hl(&mut self, high: f32, low: f32) -> SwingStatus {
        self.prev_high = Some(high);
        self.prev_low = Some(low);
        self.resistance = Some(high);
        self.support = Some(low);
        SwingStatus {
            swing_type: SwingType::LowerHighAndHigherLow,
            support: self.support,
            resistance: self.resistance,
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
        let hold = Some(SwingStatus {
            swing_type: SwingType::Hold,
            support: self.support,
            resistance: self.resistance,
        });
        match (input, &self.prev_high, &self.prev_low) {
            // This high is higher than the previous high
            // This sets a new resistance line
            (Pivot::High(current), Some(prev), _) if current > *prev => Some(self.hh(current)),
            // This low is lower than the previous low
            // This sets a new support line
            (Pivot::Low(current), _, Some(prev)) if current < *prev => Some(self.ll(current)),
            // This high is lower than the previous high
            // This creates new support line higher than the previous one
            (Pivot::High(current), Some(_prev_high), _) => Some(self.lh(current)),
            // This low is higher than the previous lowa
            // This creates new resistance line lower than the previous one
            (Pivot::Low(current), _, Some(_prev_low)) => Some(self.hl(current)),
            // A tall candle makes a new high and low at the same time
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high > *prev_high && low < *prev_low =>
            {
                Some(self.hh_and_ll(high, low))
            }
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high > *prev_high && low > *prev_low =>
            {
                Some(self.hh_and_hl(high, low))
            }
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high < *prev_high && low < *prev_low =>
            {
                Some(self.lh_and_ll(high, low))
            }
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low))
                if high < *prev_high && low > *prev_low =>
            {
                Some(self.lh_and_hl(high, low))
            }
            (Pivot::HighLow { high, low }, Some(prev_high), Some(prev_low)) => hold,
            (Pivot::HighLow { high, low }, _, _) => {
                self.prev_high = Some(high);
                self.prev_low = Some(low);
                hold
            } // There is no new high nor low; just hold
            (Pivot::NoChange, _, _) => hold,
            // We received a high but this is our first one.
            // Store it but issue a hold signal
            (Pivot::High(current), None, _) => {
                self.prev_high = Some(current);
                hold
            }
            // We got a low, but we don't have a previous low
            // Store this low, but issue a Hold Signal
            (Pivot::Low(current), _, None) => {
                self.prev_low = Some(current);
                hold
            }
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

use crate::{Close, High, Low, Open};

#[derive(Debug, PartialEq)]
pub struct RenkoCandle {
    // The floor of the open price divided by size
    pub level: i32,
    pub size: f32,
    pub direction: RenkoDirection,
}

impl Open for RenkoCandle {
    fn open(&self) -> f32 {
        self.level as f32 * self.size
    }
}

impl Close for RenkoCandle {
    fn close(&self) -> f32 {
        (match self.direction {
            RenkoDirection::Up => (self.level + 1) as f32,
            RenkoDirection::Down => (self.level - 1) as f32,
        }) * self.size
    }
}

impl High for RenkoCandle {
    fn high(&self) -> f32 {
        match self.direction {
            RenkoDirection::Up => self.close(),
            RenkoDirection::Down => self.open(),
        }
    }
}

impl Low for RenkoCandle {
    fn low(&self) -> f32 {
        match self.direction {
            RenkoDirection::Up => self.open(),
            RenkoDirection::Down => self.close(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenkoDirection {
    Up,
    Down,
}

pub struct RenkoIterator<I> {
    // Incoming prices of candle closes
    prices: I,
    // Size of the renko candles we'll output
    size: f32,
    // If we're aiming at a level more than `size` candles away, we need to step there one at a time
    last_level: Option<i32>,
    // The open() level of the next cande we will emit
    // Made from the last incoming price or the close of the last renko released
    // It is the (price / size).floor().
    start_level: Option<i32>,
    // The direction of the last renko candle
    // If a candle changes direction, we don't emit it
    last_direction: Option<RenkoDirection>,
}

impl<I> RenkoIterator<I>
where
    I: Iterator<Item = f32>,
{
    fn new(prices: I, size: f32) -> Self {
        Self {
            prices,
            size,
            last_level: None,
            start_level: None,
            last_direction: None,
        }
    }
    /// Consumes the incoming iteator and returns the next
    /// "level"
    /// A level == (price/size).floor()
    ///
    /// For example for a size of 2 these prices produce these levels:
    ///  0: 0
    ///  1: 0
    ///  2: 1
    ///  3: 1
    ///  4: 2
    fn next_level(&mut self) -> Option<i32> {
        self.prices
            .next()
            .map(|price| (price / self.size).floor() as i32)
    }
}

impl<I> Iterator for RenkoIterator<I>
where
    I: Iterator<Item = f32>,
{
    type Item = RenkoCandle;

    fn next(&mut self) -> Option<Self::Item> {
        Some(loop {
            match (self.start_level, self.last_level) {
                // If we have no input
                // Get input and loop
                (None, _) => {
                    self.start_level = Some(self.next_level()?);
                }
                // We have a previous level and need to create a last_level
                (Some(_start_level), None) => {
                    self.last_level = Some(self.next_level()?);
                }
                // Walk toward our last_level and release a candle
                (Some(start_level), Some(last_level)) if start_level != last_level => {
                    let diff = (last_level - start_level).min(1).max(-1);
                    self.start_level = Some(start_level + diff);
                    let candle =  match diff {
                        -1 => {
                            RenkoCandle {
                                level: start_level,
                                size: self.size,
                                direction: RenkoDirection::Down,
                            }
                        }
                        1 => {
                            RenkoCandle {
                                level: start_level,
                                size: self.size,
                                direction: RenkoDirection::Up,
                            }
                        }
                        _ => unreachable!(
                            "start_level: {start_level} last_level: {last_level} self.size: {} self.last_level: {:?} self.last_level: {:?}",
                            self.size,
                            self.start_level,
                            self.last_level
                        ),
                    };
                    // Store the new last_direction
                    let last_direction = self.last_direction;
                    self.last_direction = Some(candle.direction);
                    match (last_direction, candle.direction) {
                        // If we didn't have a last direction before, release this candle
                        (None, _) => break candle,
                        // If the candle is going the same way as the last candle, release the candle
                        (Some(last_direction), _) if last_direction == candle.direction => {
                            break candle
                        }
                        // If we get an up, down, up, down, up, down don't release anything except the first up
                        // until we get two in a row in the same direction
                        _ => (),
                    }
                }
                // Sequential candles are the same, get a new last_level candle
                (Some(_start_level), Some(_last_level)) => {
                    self.last_level = Some(self.next_level()?);
                }
            }
        })
    }
}

pub trait IntoRenkoIterator<I> {
    fn renko(self, size: f32) -> RenkoIterator<I>;
}

impl<I> IntoRenkoIterator<I> for I
where
    I: Iterator<Item = f32>,
{
    fn renko(self, size: f32) -> RenkoIterator<Self> {
        RenkoIterator::new(self, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_open() {
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Up,
        };
        assert_eq!(candle.open(), 10.0);
    }

    #[test]
    fn test_close() {
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Up,
        };
        assert_eq!(candle.close(), 12.0);
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Down,
        };
        assert_eq!(candle.close(), 8.0);
    }

    #[test]
    fn test_high() {
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Up,
        };
        assert_eq!(candle.high(), 12.0);
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Down,
        };
        assert_eq!(candle.high(), 10.0);
    }

    #[test]
    fn test_low() {
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Up,
        };
        assert_eq!(candle.low(), 10.0);
        let candle = RenkoCandle {
            level: 5,
            size: 2.0,
            direction: RenkoDirection::Down,
        };
        assert_eq!(candle.low(), 8.0);
    }

    #[test]
    fn test_renko_candles() {
        let prices = vec![
            10.0, 15.0, 12.0, 17.0, 13.0, 13.5, 13.999, 12.0, 12.1, 11.0, 10.0, 11.999, 11.2,
        ];
        let expected = vec![
            // 10 -> 15
            RenkoCandle {
                level: 5,
                size: 2.0,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 6,
                size: 2.0,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 7,
                size: 2.0,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 7,
                size: 2.0,
                direction: RenkoDirection::Down,
            },
            // 13-11
            RenkoCandle {
                level: 6,
                size: 2.0,
                direction: RenkoDirection::Down,
            },
            // All the rest ignored
        ];
        let got: Vec<RenkoCandle> = prices.into_iter().renko(2.0).collect();
        assert_eq!(expected, got);
    }
}

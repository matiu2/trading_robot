use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct RenkoCandle {
    // The floor of the open price divided by size
    level: i32,
    pub size: f32,
    pub direction: RenkoDirection,
}

impl RenkoCandle {
    pub fn open(&self) -> f32 {
        self.level as f32 * self.size
    }

    pub fn close(&self) -> f32 {
        (match self.direction {
            RenkoDirection::Up => (self.level + 1) as f32,
            RenkoDirection::Down => (self.level - 1) as f32,
        }) * self.size
    }

    pub fn high(&self) -> f32 {
        match self.direction {
            RenkoDirection::Up => self.close(),
            RenkoDirection::Down => self.open(),
        }
    }

    pub fn low(&self) -> f32 {
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

struct RenkoIterator<I> {
    // Incoming prices of candle closes
    prices: I,
    // Size of the renko candles we'll output
    size: f32,
    // If we're aiming at a level more than `size` candles away, we need to step there one at a time
    target: Option<i32>,
    // Made from the last incoming price or the close of the last renko released
    // It is the (price / size).floor().
    next_open: Option<i32>,
}

impl<I, Item> RenkoIterator<I>
where
    I: Iterator<Item = Item>,
    Item: Deref<Target = f32>,
{
    fn new(prices: I, size: f32) -> Self {
        Self {
            prices,
            size,
            target: None,
            next_open: None,
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
            .map(|price| (price.deref() / self.size).floor() as i32)
    }
}

impl<I, Item> Iterator for RenkoIterator<I>
where
    I: Iterator<Item = Item>,
    Item: Deref<Target = f32>,
{
    type Item = RenkoCandle;

    fn next(&mut self) -> Option<Self::Item> {
        Some(loop {
            match (self.next_open, self.target) {
                // If we have no input
                // Get input and loop
                (None, _) => {
                    self.next_open = Some(self.next_level()?);
                }
                // We have a previous level and need to create a target
                (Some(_next_open), None) => {
                    self.target = Some(self.next_level()?);
                }
                // Walk toward our target and release a candle
                (Some(next_open), Some(target)) if next_open != target => {
                    let diff = (target - next_open).min(1).max(-1);
                    self.next_open = Some(next_open + diff);
                    match diff {
                        -1 => {
                            break RenkoCandle {
                                level: next_open,
                                size: self.size,
                                direction: RenkoDirection::Down,
                            }
                        }
                        1 => {
                            break RenkoCandle {
                                level: next_open,
                                size: self.size,
                                direction: RenkoDirection::Up,
                            }
                        }
                        _ => unreachable!(
                            "next_open: {next_open} target: {target} self.size: {} self.last_level: {:?} self.target: {:?}",
                            self.size,
                            self.next_open,
                            self.target
                        ),
                    };
                }
                // Sequential candles are the same, get a new target candle
                (Some(_next_open), Some(_target)) => {
                    self.target = Some(self.next_level()?);
                }
            }
        })
    }
}

trait Renko<I> {
    fn renko(self, size: f32) -> RenkoIterator<I>;
}

impl<I, Item> Renko<I> for I
where
    I: Iterator<Item = Item>,
    Item: Deref<Target = f32>,
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
            // 12
            RenkoCandle {
                level: 7,
                size: 2.0,
                direction: RenkoDirection::Down,
            },
            // 12-17
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
            // 17-13
            RenkoCandle {
                level: 8,
                size: 2.0,
                direction: RenkoDirection::Down,
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
        let got: Vec<RenkoCandle> = prices.iter().renko(2.0).collect();
        assert_eq!(expected, got);
    }
}

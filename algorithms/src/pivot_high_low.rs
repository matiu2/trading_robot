use crate::candle::{High, Low};
use std::fmt::Debug as Dbg;

#[derive(Debug, PartialEq, Clone)]
pub enum Pivot {
    High(f32),
    Low(f32),
    // A tall candle that sets a new high and low at the same time
    HighLow { high: f32, low: f32 },
    NoChange,
}

impl Pivot {
    /// Returns the high value of the pivot, if it exists.
    ///
    /// For a `Pivot::High` or `Pivot::HighLow`, this method returns the high value.
    /// For a `Pivot::Low` or `Pivot::NoChange`, this method returns `None`.
    pub fn high(&self) -> Option<f32> {
        match self {
            Pivot::High(high) => Some(*high),
            Pivot::HighLow { high, .. } => Some(*high),
            _ => None,
        }
    }

    /// Returns the low value of the pivot, if it exists.
    ///
    /// For a `Pivot::Low` or `Pivot::HighLow`, this method returns the low value.
    /// For a `Pivot::High` or `Pivot::NoChange`, this method returns `None`.
    pub fn low(&self) -> Option<f32> {
        match self {
            Pivot::Low(low) => Some(*low),
            Pivot::HighLow { low, .. } => Some(*low),
            _ => None,
        }
    }

    pub fn is_high(&self) -> bool {
        matches!(self, Pivot::High(_))
    }

    pub fn is_low(&self) -> bool {
        matches!(self, Pivot::Low(_))
    }

    pub fn is_high_low(&self) -> bool {
        matches!(self, Pivot::HighLow { .. })
    }

    pub fn is_no_change(&self) -> bool {
        matches!(self, Pivot::NoChange)
    }
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
pub fn pivots(
    input: &[impl High + Low + Dbg],
    window_size: usize,
) -> impl Iterator<Item = Pivot> + Clone + Dbg + '_ {
    // TODO: Make this a compile time check
    assert!(window_size != 0, "Can't have a zero sized sliding window");
    // TODO: Make this an Error instead of a panic?
    assert!(
        window_size <= input.len(),
        "Window size must be <= input length"
    );
    let mid_index = window_size / 2;
    let start = std::iter::repeat(Pivot::NoChange).take(window_size - 1);
    let rest = input.windows(window_size).map(move |window| {
        let mid = &window[mid_index];
        let mid_high = mid.high();
        let mid_low = mid.low();
        let left = window[..mid_index].iter();
        let right = window[mid_index..].iter().skip(1);
        // If the middle candle's high is higher than all the other candles, this is a pivot high
        let is_high = left.clone().all(|candle| mid_high > candle.high())
            && right.clone().all(|candle| mid_high > candle.high());
        // If the middle candle's low is lower than all the other candles, this is a pivot low
        let is_low = left.clone().all(|candle| mid_low < candle.low())
            && right.clone().all(|candle| mid_low < candle.low());
        match (is_high, is_low) {
            (true, true) => Pivot::HighLow {
                high: mid_high,
                low: mid_low,
            },
            (true, false) => Pivot::High(mid_high),
            (false, true) => Pivot::Low(mid_low),
            (false, false) => Pivot::NoChange,
        }
    });
    start.chain(rest)
}

#[cfg(test)]
mod test {
    use super::{pivots, Pivot};
    use crate::{
        candle::test_data::{test_data_1, test_data_2, Candle},
        Close, High, Low, Open, RenkoCandle, RenkoDirection,
    };

    #[test]
    fn test_1_odd_number() {
        let data = test_data_1();
        let pivots = pivots(data.as_slice(), 5);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::Low(4.0),
            Pivot::NoChange,
            Pivot::High(11.0),
            Pivot::Low(3.0),
            Pivot::NoChange,
        ];
        assert_eq!(expected, pivots.collect::<Vec<_>>());
    }

    #[test]
    fn test_1_even_window() {
        let data = test_data_1();
        let pivots = pivots(data.as_slice(), 4);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::Low(4.0),
            Pivot::NoChange,
            Pivot::High(11.0),
            Pivot::Low(3.0),
            Pivot::NoChange,
            Pivot::NoChange,
        ];
        assert_eq!(expected, pivots.collect::<Vec<_>>());
    }

    #[test]
    fn test_2_large() {
        let data = test_data_2();
        let pivots = pivots(data.as_slice(), 5);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::High(18.0),
            Pivot::Low(5.0),
            Pivot::NoChange,
            Pivot::High(16.0),
            Pivot::NoChange,
        ];
        assert_eq!(expected, pivots.collect::<Vec<_>>());
    }

    #[test]
    fn test_2_small() {
        let data = test_data_2();
        let pivots = pivots(data.as_slice(), 3);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::Low(6.0),
            Pivot::High(18.0),
            Pivot::Low(5.0),
            Pivot::NoChange,
            Pivot::High(16.0),
            Pivot::NoChange,
            Pivot::Low(4.0),
        ];
        assert_eq!(expected, pivots.collect::<Vec<_>>());
    }

    #[test]
    fn test_high_low() {
        let data = vec![
            Candle::new(20.0, 10.0, 15.0, 12.0),
            Candle::new(15.0, 8.0, 12.0, 10.0),
            Candle::new(32.0, 6.0, 9.0, 8.0),
            Candle::new(18.0, 11.0, 14.0, 13.0),
        ];

        let pivots = pivots(data.as_slice(), 3);
        let expected = vec![
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::NoChange,
            Pivot::HighLow {
                high: 32.0,
                low: 6.0,
            },
        ];
        assert_eq!(expected, pivots.collect::<Vec<_>>());
    }

    #[test]
    fn pivot_renko() {
        let candles = [
            RenkoCandle {
                level: 9853,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9852,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9851,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9850,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9849,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9845,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9844,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9843,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9843,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9843,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9843,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9844,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9845,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9849,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9850,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9851,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9851,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9850,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9849,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9845,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9845,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9846,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9848,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Up,
            },
            RenkoCandle {
                level: 9847,
                size: 0.00010927235,
                direction: RenkoDirection::Down,
            },
        ];
        for candle in &candles {
            println!(
                "o: {} c: {} h: {} l: {}",
                candle.open(),
                candle.close(),
                candle.high(),
                candle.low()
            );
        }
        let pivots: Vec<_> = pivots(candles.as_slice(), 5).collect();
        println!("pivots: {pivots:#?}");

        create_candlestick_chart(&candles);

        fn create_candlestick_chart(candles: &[RenkoCandle]) {
            use svg::node::element::Line;
            use svg::node::element::Rectangle;
            use svg::Document;

            let width = 1080;
            let height = 300;
            let mut document = Document::new()
                .set("width", width)
                .set("height", height)
                .set("viewBox", (0, 0, width, height));

            let hh = candles
                .iter()
                .map(High::high)
                .reduce(|acc, n| if n > acc { n } else { acc })
                .unwrap();
            let ll = candles
                .iter()
                .map(Low::low)
                .reduce(|acc, n| if n < acc { n } else { acc })
                .unwrap();

            // Scale and translate the data to fit the viewBox
            let scale_y = height as f32 / (hh - ll);
            let translate_y = -ll * scale_y;
            for (i, candle) in candles.iter().enumerate() {
                let x = i as f64 * 36.0;

                let open_y = (candle.open() * scale_y + translate_y) as f64;
                let close_y = (candle.close() * scale_y + translate_y) as f64;
                let high_y = (candle.high() * scale_y + translate_y) as f64;
                let low_y = (candle.low() * scale_y + translate_y) as f64;

                let color = if candle.open() < candle.close() {
                    "green"
                } else {
                    "red"
                };
                let body = Rectangle::new()
                    .set("x", x)
                    .set("y", close_y.min(open_y))
                    .set("width", 20)
                    .set("height", (open_y - close_y).abs())
                    .set("fill", color)
                    .set("stroke", "black")
                    .set("stroke-width", 1);
                document = document.add(body);

                let line = Line::new()
                    .set("x1", x + 10.0)
                    .set("y1", high_y)
                    .set("x2", x + 10.0)
                    .set("y2", low_y)
                    .set("stroke", "black")
                    .set("stroke-width", 1);
                document = document.add(line);
            }

            svg::save("tmp.svg", &document).unwrap();
        }
    }
}

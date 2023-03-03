#[derive(PartialEq, Debug)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub struct HhllSignals<I>
where
    I: Iterator<Item = f32>,
{
    prices: I,
    prev_high: Option<f32>,
    prev_low: Option<f32>,
    trend: Signal,
}

impl<I> HhllSignals<I>
where
    I: Iterator<Item = f32>,
{
    pub fn new(prices: I) -> Self {
        HhllSignals {
            prices,
            prev_high: None,
            prev_low: None,
            trend: Signal::Hold,
        }
    }
}

impl<I> Iterator for HhllSignals<I>
where
    I: Iterator<Item = f32>,
{
    type Item = Signal;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_price = self.prices.next()?;

        let prev_high = self.prev_high.get_or_insert(curr_price);
        let prev_low = self.prev_low.get_or_insert(curr_price);

        if curr_price > *prev_high {
            if self.trend == Signal::Sell || self.trend == Signal::Hold {
                self.trend = Signal::Buy;
                *prev_high = curr_price;
                *prev_low = curr_price;
                return Some(Signal::Buy);
            } else {
                return Some(Signal::Hold);
            }
        } else if curr_price < *prev_low {
            if self.trend == Signal::Buy || self.trend == Signal::Hold {
                self.trend = Signal::Sell;
                *prev_high = curr_price;
                *prev_low = curr_price;
                return Some(Signal::Sell);
            } else {
                return Some(Signal::Hold);
            }
        } else {
            *prev_high = prev_high.max(curr_price);
            *prev_low = prev_low.min(curr_price);
            self.trend = Signal::Hold;
            return Some(Signal::Hold);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn make_vec(
        prices: Vec<f32>,
        expected: Vec<Signal>,
    ) -> (Vec<(f32, Signal)>, Vec<(f32, Signal)>) {
        let got: Vec<(f32, Signal)> = prices
            .iter()
            .cloned()
            .zip(HhllSignals::new(prices.iter().cloned()))
            .collect();
        let expected: Vec<(f32, Signal)> =
            prices.iter().cloned().zip(expected.into_iter()).collect();
        (expected, got)
    }

    #[test]
    fn test_buy_signal() {
        let (expected, got) = make_vec(
            vec![10.0, 11.0, 12.0, 9.0, 8.0, 7.0, 10.0, 11.0, 12.0],
            vec![
                Signal::Hold,
                Signal::Buy,
                Signal::Hold,
                Signal::Sell,
                Signal::Hold,
                Signal::Hold,
                Signal::Buy,
                Signal::Hold,
                Signal::Hold,
            ],
        );
        assert_eq!(expected, got);
    }

    #[test]
    fn test_sell_signal() {
        let (expected, got) = make_vec(
            vec![10.0, 9.0, 8.0, 11.0, 12.0, 13.0, 10.0, 9.0, 8.0],
            vec![
                Signal::Hold,
                Signal::Sell,
                Signal::Hold,
                Signal::Buy,
                Signal::Hold,
                Signal::Hold,
                Signal::Sell,
                Signal::Hold,
                Signal::Hold,
            ],
        );
        assert_eq!(expected, got);
    }

    #[test]
    fn test_hold_signal() {
        let (expected, got) = make_vec(
            vec![10.0, 9.0, 11.0, 8.0, 12.0, 7.0, 13.0, 6.0, 14.0],
            vec![
                Signal::Hold,
                Signal::Sell,
                Signal::Buy,
                Signal::Sell,
                Signal::Buy,
                Signal::Sell,
                Signal::Buy,
                Signal::Sell,
                Signal::Buy,
            ],
        );
        assert_eq!(expected, got);
    }

    #[test]
    fn test_real_world() {
        let (expected, got) = make_vec(
            vec![4.0, 3.0, 3.0, 2.0, 2.0, 1.0, 3.0, 2.0],
            vec![
                Signal::Hold,
                Signal::Sell,
                Signal::Hold,
                Signal::Hold,
                Signal::Hold,
                Signal::Hold,
                Signal::Hold,
                Signal::Hold,
            ],
        );
        assert_eq!(expected, got);
    }
}

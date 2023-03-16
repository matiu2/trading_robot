//! This module defines four traits: High, Low, Open, and Close, which
//! represent the four values of a candlestick chart. It also implements
//! each of these traits for any type that implements Deref to a type
//! that implements the corresponding trait. This allows the values of
//! a candlestick chart to be used without needing to know the specific
//! type of the underlying data structure.
//!
//! Implment these traits for your data types to use the algorithms in
//! this module

use std::ops::Deref;

pub trait High {
    fn high(&self) -> f32;
}

pub trait Low {
    fn low(&self) -> f32;
}

pub trait Open {
    fn open(&self) -> f32;
}

pub trait Close {
    fn close(&self) -> f32;
}

impl<T, H> High for T
where
    T: Deref<Target = H>,
    H: High,
{
    fn high(&self) -> f32 {
        self.deref().high()
    }
}

impl<T, L> Low for T
where
    T: Deref<Target = L>,
    L: Low,
{
    fn low(&self) -> f32 {
        self.deref().low()
    }
}

impl<T, O> Open for T
where
    T: Deref<Target = O>,
    O: Open,
{
    fn open(&self) -> f32 {
        self.deref().open()
    }
}

impl<T, C> Close for T
where
    T: Deref<Target = C>,
    C: Close,
{
    fn close(&self) -> f32 {
        self.deref().close()
    }
}

#[cfg(test)]
pub mod test_data {
    use super::{Close, High, Low, Open};

    #[derive(Default, Debug, PartialEq, Clone)]
    pub struct Candle {
        pub high: f32,
        pub low: f32,
        pub open: f32,
        pub close: f32,
    }

    impl Candle {
        pub fn new(high: f32, low: f32, open: f32, close: f32) -> Self {
            Self {
                high,
                low,
                open,
                close,
            }
        }
    }

    impl High for Candle {
        fn high(&self) -> f32 {
            self.high
        }
    }

    impl Low for Candle {
        fn low(&self) -> f32 {
            self.low
        }
    }

    impl Open for Candle {
        fn open(&self) -> f32 {
            self.open
        }
    }

    impl Close for Candle {
        fn close(&self) -> f32 {
            self.close
        }
    }

    pub fn test_data_1() -> Vec<Candle> {
        // hl means the absolute difference between the high and the low
        // hpc is the absolute difference between the high and the previous close
        // lpc is the absolute difference between the low and the previous close
        // tr is the true range, which is the greatest of the above values
        vec![
            Candle::new(10.0, 5.0, 8.0, 7.0), // hl=5 hpc=None lpc=None tr=5.0
            Candle::new(12.0, 6.0, 9.0, 8.0), // hl=6 hpc=5 lpc=1 tr=6
            Candle::new(8.0, 4.0, 7.0, 6.0),  // hl=4 hpc=0 lpc=2 tr=4
            Candle::new(9.0, 5.0, 8.0, 7.0),  // hl=4 hpc=3 lpc=1 tr=4
            Candle::new(11.0, 6.0, 9.0, 8.0), // hl=5 hpc=4 lpc=2 tr=5
            Candle::new(7.0, 3.0, 6.0, 5.0),  // hl=5 hpc=1 lpc=2 tr=5
            Candle::new(8.0, 4.0, 7.0, 6.0),  // hl=4 hpc=4 lpc=1 tr=4
            Candle::new(10.0, 5.0, 8.0, 7.0), // hl=5 hpc=4 lpc=1 tr=5
            Candle::new(12.0, 6.0, 9.0, 8.0), // hl=6 hpc=5 lpc=1 tr=6
        ]
    }

    pub fn test_data_2() -> Vec<Candle> {
        vec![
            Candle::new(20.0, 10.0, 15.0, 12.0),
            Candle::new(15.0, 8.0, 12.0, 10.0),
            Candle::new(12.0, 6.0, 9.0, 8.0),
            Candle::new(18.0, 11.0, 14.0, 13.0),
            Candle::new(10.0, 5.0, 8.0, 7.0),
            Candle::new(14.0, 7.0, 11.0, 9.0),
            Candle::new(16.0, 9.0, 13.0, 11.0),
            Candle::new(13.0, 6.0, 10.0, 8.0),
            Candle::new(11.0, 4.0, 8.0, 7.0),
            Candle::new(19.0, 12.0, 16.0, 14.0),
        ]
    }

    pub fn test_data_3() -> Vec<Candle> {
        vec![
            Candle::new(11.0, 7.0, 8.0, 9.0),
            Candle::new(8.0, 5.0, 6.0, 7.0),
            Candle::new(9.0, 6.0, 7.0, 8.0),
            Candle::new(12.0, 8.0, 9.0, 10.0),
            Candle::new(10.0, 7.0, 8.0, 9.0),
            Candle::new(8.0, 5.0, 6.0, 7.0),
            Candle::new(9.0, 6.0, 7.0, 8.0),
            Candle::new(12.0, 8.0, 9.0, 10.0),
            Candle::new(11.0, 7.0, 8.0, 9.0),
            Candle::new(8.0, 5.0, 6.0, 7.0),
            Candle::new(9.0, 6.0, 7.0, 8.0),
            Candle::new(12.0, 8.0, 9.0, 10.0),
        ]
    }

    pub fn generate_random_test_data(n: usize) -> Vec<Candle> {
        use rand::distributions::{Distribution, Uniform};

        let mut rng = rand::thread_rng();
        let dist = Uniform::from(1.0..=100.0);
        let mut candles = Vec::with_capacity(n);
        for _ in 0..n {
            let high = dist.sample(&mut rng);
            let low = dist.sample(&mut rng);
            let open = dist.sample(&mut rng);
            let close = dist.sample(&mut rng);
            candles.push(Candle::new(high, low, open, close));
        }
        candles
    }
}

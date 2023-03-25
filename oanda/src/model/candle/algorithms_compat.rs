use super::Candle;
use algorithms::{Close, High, Low, Open};

impl High for Candle {
    fn high(&self) -> f32 {
        self.mid.as_ref().unwrap().h
    }
}
impl Low for Candle {
    fn low(&self) -> f32 {
        self.mid.as_ref().unwrap().l
    }
}
impl Open for Candle {
    fn open(&self) -> f32 {
        self.mid.as_ref().unwrap().o
    }
}
impl Close for Candle {
    fn close(&self) -> f32 {
        self.mid.as_ref().unwrap().c
    }
}

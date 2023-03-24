//! Given high low swing gives you a support and resistance
use super::higher_high_lower_low::SwingStatus;

#[derive(Default)]
pub struct SupportAndResistance {
    pub support: Option<f32>,
    pub resistance: Option<f32>,
}

pub trait IntoSupportAndResistance: Iterator<Item = SwingStatus> + Sized {
    fn support_and_resistance(self) -> SupportAndResistance {
        self.last()
            .map(|swing_status| SupportAndResistance {
                support: swing_status.support,
                resistance: swing_status.resistance,
            })
            .unwrap_or_default()
    }
}

impl<T: Iterator<Item = SwingStatus>> IntoSupportAndResistance for T {}

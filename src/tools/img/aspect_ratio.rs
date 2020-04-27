//! Formula implementation.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Aspect-ratio enumeration.
#[load]
#[derive(Clone)]
pub enum AspectRatio {
    /// Square. 1:1.
    Square,
    /// Classic photographic film. 3:2.
    Classic,
    /// Golden ratio. ((1+sqrt(5))/2):1
    Golden,
    /// Silver ratio. (1+sqrt(2)):1
    Silver,
    /// Standard. 16:9
    Standard,
    /// Widescreen. 43:18
    Widescreen,
    /// IPhone XS. (1125 x 2436)
    IPhoneXS,
    /// IPhone 7. (750 x 1334)
    IPhone7,
}

impl AspectRatio {
    /// Get the pixel ratios.
    #[inline]
    #[must_use]
    pub fn ratio(&self) -> f64 {
        match self {
            Self::Square => 1.0,
            Self::Classic => 3.0 / 2.0,
            Self::Golden => (1.0 + 5.0_f64.sqrt()) / 2.0,
            Self::Silver => 1.0 + 2.0_f64.sqrt(),
            Self::Standard => 16.0 / 9.0,
            Self::Widescreen => 43.0 / 18.0,
            Self::IPhoneXS => 1125.0 / 2436.0,
            Self::IPhone7 => 750.0 / 1334.0,
        }
    }
}

impl Display for AspectRatio {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}",
            match self {
                Self::Square => "Square",
                Self::Classic => "Classic",
                Self::Golden => "Golden",
                Self::Silver => "Silver",
                Self::Standard => "Standard",
                Self::Widescreen => "Widescreen",
                Self::IPhoneXS => "IPhone XS",
                Self::IPhone7 => "IPhone 7",
            }
        )
    }
}

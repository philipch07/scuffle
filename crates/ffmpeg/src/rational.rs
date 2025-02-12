use std::num::NonZero;

use rusty_ffmpeg::ffi::AVRational;

/// A rational number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    /// Numerator.
    pub numerator: i32,
    /// Denominator.
    pub denominator: NonZero<i32>,
}

impl Default for Rational {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rational {
    /// The zero rational number.
    pub const ZERO: Rational = Rational::static_new::<0, 1>();

    /// Create a new rational number.
    pub const fn new(numerator: i32, denominator: NonZero<i32>) -> Self {
        Self { numerator, denominator }
    }

    /// Construct a new rational number at compile time.
    ///
    /// # Panics
    ///
    /// This will panic if the denominator is 0.
    pub const fn static_new<const N: i32, const D: i32>() -> Self {
        const {
            assert!(D != 0, "denominator is 0");
        }

        Self::new(N, NonZero::new(D).expect("denominator is 0"))
    }

    /// Get the rational number as a floating point number.
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator.get() as f64
    }

    /// Create a new rational number from a floating point number.
    /// The number might be truncated.
    pub fn from_f64_rounded(value: f64) -> Self {
        let denominator = value.abs().recip();
        let numerator = (value * denominator).round() as i32;
        Self {
            numerator,
            denominator: NonZero::new(denominator as i32).expect("denominator is 0"),
        }
    }
}

impl From<AVRational> for Rational {
    fn from(rational: AVRational) -> Self {
        if rational.den == 0 {
            return Self::ZERO;
        }

        Self {
            numerator: rational.num,
            denominator: NonZero::new(rational.den).expect("denominator is 0"),
        }
    }
}

impl From<Rational> for AVRational {
    fn from(rational: Rational) -> Self {
        Self {
            num: rational.numerator,
            den: rational.denominator.get(),
        }
    }
}

impl From<i32> for Rational {
    fn from(value: i32) -> Self {
        Self {
            numerator: value,
            denominator: NonZero::new(1).expect("1 is not 0"),
        }
    }
}

impl From<Rational> for f32 {
    fn from(rational: Rational) -> Self {
        rational.numerator as f32 / rational.denominator.get() as f32
    }
}

impl From<Rational> for f64 {
    fn from(rational: Rational) -> Self {
        rational.numerator as f64 / rational.denominator.get() as f64
    }
}

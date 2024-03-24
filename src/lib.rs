#![cfg_attr(not(feature = "use_std"), no_std)]

use core::{
    f64::consts::{FRAC_PI_2, PI},
    ops::{Add, Sub},
};
use libm::{cos, fabs, sin};
#[cfg(feature = "use_std")]
use std::fmt::{Display, Formatter};

pub type Radians = f64;
pub type Degrees = f64;

pub const RADIANS_90_DEGREES: Radians = FRAC_PI_2;

#[derive(Copy, Clone)]
pub struct Angle {
    value: f64,
}

impl Angle {
    pub fn radians(value: Radians) -> Self {
        Self { value }.normalize()
    }

    pub fn degrees(value: Degrees) -> Self {
        Self::radians(value.to_radians())
    }

    pub fn as_radians(&self) -> Radians {
        self.value
    }

    pub fn as_degrees(&self) -> Degrees {
        self.value.to_degrees()
    }

    pub fn abs(&self) -> Self {
        Self {
            value: fabs(self.value),
        }
    }

    pub fn cos(&self) -> f64 {
        cos(self.as_radians())
    }

    pub fn sin(&self) -> f64 {
        sin(self.as_radians())
    }

    pub fn is_within(&self, other: &Angle, difference: Angle) -> bool {
        (self.clone() - other.clone()).abs().as_radians() < difference.as_radians()
    }

    fn normalize(self) -> Self {
        let value = self.value % (2.0 * PI);

        let value = if value > PI {
            value - 2.0 * PI
        } else if value < -PI {
            value + 2.0 * PI
        } else {
            value
        };

        Self { value }
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value;
        Self { value }.normalize()
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.value - rhs.value;
        Self { value }.normalize()
    }
}

#[cfg(feature = "use_std")]
impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}deg", self.as_degrees())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within() {
        let a1 = Angle::radians(RADIANS_90_DEGREES);
        let a2 = Angle::radians(RADIANS_90_DEGREES);

        assert!(a1.is_within(&a2, Angle::degrees(0.001)));
    }

    #[test]
    fn deg_to_rad() {
        let a1 = Angle::radians(RADIANS_90_DEGREES);
        let a2 = Angle::degrees(90.0);

        assert!(a1.is_within(&a2, Angle::degrees(0.001)));
    }

    #[test]
    fn norm() {
        let a1 = Angle::degrees(90.0);
        let a2 = Angle::degrees(90.0 + 360.0 * 7.0);
        let a3 = Angle::degrees(90.0 - 360.0 * 7.0);

        assert!(a1.is_within(&a2, Angle::degrees(0.001)));
        assert!(a1.is_within(&a3, Angle::degrees(0.001)));
    }

    #[test]
    fn add() {
        let a1 = Angle::degrees(90.0);
        let a2 = Angle::degrees(5.0);
        let r = Angle::degrees(90.0 + 5.0);

        assert!((a1 + a2).is_within(&r, Angle::degrees(0.001)));
    }

    #[test]
    fn sub() {
        let a1 = Angle::degrees(90.0);
        let a2 = Angle::degrees(5.0);
        let r = Angle::degrees(90.0 - 5.0);

        assert!((a1 - a2).is_within(&r, Angle::degrees(0.001)));
    }

    #[test]
    fn add_normalize() {
        let a1 = Angle::degrees(90.0);
        let a2 = Angle::degrees(180.0);
        let r = Angle::degrees(-90.0);

        assert!((a1 + a2).is_within(&r, Angle::degrees(0.001)));
        assert!((a1 + a2 + a2).is_within(&a1, Angle::degrees(0.001)));
        assert!((a1 + a2 + a2 + a2).is_within(&r, Angle::degrees(0.001)));
    }

    #[test]
    fn sub_normalize() {
        let a1 = Angle::degrees(90.0);
        let a2 = Angle::degrees(180.0);
        let r = Angle::degrees(-90.0);

        assert!((a1 - a2).is_within(&r, Angle::degrees(0.001)));
        assert!((a1 - a2 - a2).is_within(&a1, Angle::degrees(0.001)));
        assert!((a1 - a2 - a2 - a2).is_within(&r, Angle::degrees(0.001)));
    }

    #[test]
    fn sin_cos() {
        let sin_alpha_cos_beta = [
            (0.0, 0.0),
            (15.0, 0.2588),
            (30.0, 0.5),
            (45.0, 0.7071),
            (60.0, 0.8660),
            (80.0, 0.9848),
            (90.0, 1.0),
        ];

        for (a, sin_alpha) in sin_alpha_cos_beta {
            let alpha = Angle::degrees(a);
            let beta = Angle::degrees(90.0 - a);

            assert!(fabs(alpha.sin() - sin_alpha) < 0.001);
            assert!(fabs(beta.cos() - sin_alpha) < 0.001);
        }
    }
}

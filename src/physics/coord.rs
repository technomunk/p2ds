//! Coordinate system used by the physics engine.
// TODO: explain assumptions formed by the coordinate system

use std::ops::*;

use bevy::{math::Vec2, prelude::Component};

/// Single-dimensional point. A unit of most calculations.
pub type P1 = i32;

/// Two-dimensional point. Can represent arbitrary quantities.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct P2 {
    pub x: P1,
    pub y: P1,
}

impl P2 {
    /// Construct a new point.
    #[inline(always)]
    pub fn new(x: P1, y: P1) -> Self {
        Self { x, y }
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at the numeric bounds instead of overflowing.
    #[inline(always)]
    pub fn saturating_add(self, rhs: P2) -> Self {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating at the numeric bounds instead of overflowing.
    pub fn saturating_sub(self, rhs: P2) -> Self {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }

    /// Saturating integer multiplication. Computes `self * rhs`, saturating at the numeric bounds instead of overflowing.
    pub fn saturating_mul(self, rhs: P2) -> Self {
        Self {
            x: self.x.saturating_mul(rhs.x),
            y: self.y.saturating_mul(rhs.y),
        }
    }
}

/// Position within the physical world.
///
/// An Entity must have Position component to be affected by physics.
#[derive(Clone, Copy, Debug, Default, Component, PartialEq, Eq)]
pub struct Position(pub P2);

impl Position {
    /// Construct a new position at provided coordinates.
    #[inline(always)]
    pub fn new(x: P1, y: P1) -> Self {
        Self(P2::new(x, y))
    }

    /// Convert the integer position into a floating point position within the provided range.
    pub fn to_range(self, min: Vec2, max: Vec2) -> Vec2 {
        let range = (max - min).abs();
        let middle = (min + max) / 2.0;
        // these now vary between -1 and 1
        let x = self.0.x as f32 / P1::MAX as f32;
        let y = self.0.y as f32 / P1::MAX as f32;
        let result = middle + range * Vec2::new(x, y) * 0.5;
        result
    }
}

impl AsRef<P2> for Position {
    #[inline(always)]
    fn as_ref(&self) -> &P2 {
        &self.0
    }
}

impl AsMut<P2> for Position {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut P2 {
        &mut self.0
    }
}

macro_rules! op {
    (impl $op_trait:ident fn $op:ident) => {
        impl $op_trait<P2> for P2 {
            type Output = P2;
            #[inline(always)]
            fn $op(self, rhs: P2) -> Self::Output {
                Self::Output {
                    x: self.x.$op(rhs.x),
                    y: self.y.$op(rhs.y),
                }
            }
        }

        impl $op_trait<&P2> for P2 {
            type Output = P2;
            #[inline(always)]
            fn $op(self, rhs: &P2) -> Self::Output {
                Self::Output {
                    x: self.x.$op(rhs.x),
                    y: self.y.$op(rhs.y),
                }
            }
        }
    };
}

macro_rules! op_assign {
    (impl $op_trait:ident fn $op:ident) => {
        impl $op_trait<P2> for P2 {
            #[inline(always)]
            fn $op(&mut self, rhs: P2) {
                self.x.$op(rhs.x);
                self.y.$op(rhs.y);
            }
        }

        impl $op_trait<&P2> for P2 {
            #[inline(always)]
            fn $op(&mut self, rhs: &P2) {
                self.x.$op(rhs.x);
                self.y.$op(rhs.y);
            }
        }
    };
}

op! { impl Add fn add }
op_assign! { impl AddAssign fn add_assign }
op! { impl Sub fn sub }
op_assign! { impl SubAssign fn sub_assign }

op! { impl BitAnd fn bitand }
op_assign! { impl BitAndAssign fn bitand_assign }
op! { impl BitOr fn bitor }
op_assign! { impl BitOrAssign fn bitor_assign }
op! { impl BitXor fn bitxor }
op_assign! { impl BitXorAssign fn bitxor_assign }

op! { impl Mul fn mul }
op_assign! { impl MulAssign fn mul_assign }
op! { impl Div fn div }
op_assign! { impl DivAssign fn div_assign }
op! { impl Rem fn rem }
op_assign! { impl RemAssign fn rem_assign }

impl From<[P1; 2]> for P2 {
    #[inline(always)]
    fn from(original: [P1; 2]) -> Self {
        Self::new(original[0], original[1])
    }
}

impl From<(P1, P1)> for P2 {
    #[inline(always)]
    fn from((x, y): (P1, P1)) -> Self {
        Self::new(x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operators() {
        assert_eq!(P2::new(1, 2) & P2::new(3, 0), P2::new(1, 0));
        assert_eq!(P2::new(1, 2) | P2::new(3, 1), P2::new(3, 3));
        assert_eq!(P2::new(1, 2) ^ P2::new(!1, 2), P2::new(1, !2));

        assert_eq!(P2::new(1, 2) + P2::new(3, 4), P2::new(4, 6));
        assert_eq!(P2::new(3, 4) - P2::new(1, 2), P2::new(2, 2));

        assert_eq!(P2::new(1, 2) * P2::new(3, 4), P2::new(3, 8));
        assert_eq!(P2::new(3, 4) / P2::new(2, 5), P2::new(1, 0));
        assert_eq!(P2::new(3, 4) % P2::new(2, 2), P2::new(1, 0));
    }

    #[test]
    fn test_saturating_operators() {
        assert_eq!(
            P2::new(14, -55).saturating_sub([55, i32::MAX].into()),
            P2::new(-41, i32::MIN)
        );
        assert_eq!(
            P2::new(14, 55).saturating_add(P2::new(i32::MAX, 55)),
            P2::new(i32::MAX, 110)
        );
        assert_eq!(
            P2::new(1_000_000_000, 11).saturating_mul((16_000, 2).into()),
            P2::new(i32::MAX, 22)
        );
    }
}

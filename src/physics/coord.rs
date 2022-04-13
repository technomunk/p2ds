//! Coordinate system used by the physics engine.
// TODO: explain assumptions formed by the coordinate system

use bevy::{math::Vec2, prelude::Component};
use nalgebra as na;

/// The underlying coordinate representation.
pub type CoordT = i32;

/// Two-dimensional point. Can represent arbitrary quantities.
pub type Point2 = na::Point2<CoordT>;
/// A directed two-dimensional magnitude.
pub type Vector2 = na::Vector2<CoordT>;

/// Move the provided point by the provided vector up to representable limit.
///
/// Returns the distance not covered due to hitting the limit.
pub fn spill_add(point: &mut Point2, vector: Vector2) -> Vector2 {
    let x = unspill_add(&mut point[0], vector[0]);
    let y = unspill_add(&mut point[1], vector[1]);

    Vector2::new(x, y)
}

/// Perform a typical sum, but unspill over and underflow.
fn unspill_add(a: &mut CoordT, b: CoordT) -> CoordT {
    let oa = *a;

    *a = a.wrapping_add(b);

    if oa >= 0 && b >= 0 && *a < 0 {
        // overflow
        let oa = *a;
        *a = CoordT::MAX;
        return oa.wrapping_sub(CoordT::MAX);
    }

    if oa < 0 && b < 0 && *a > 0 {
        // underflow
        let oa = *a;
        *a = CoordT::MIN;
        return oa.wrapping_sub(CoordT::MIN);
    }

    0
}

/// Position within the physical world.
///
/// An Entity must have Position component to be affected by physics.
#[derive(Clone, Copy, Debug, Default, Component, PartialEq, Eq)]
pub struct Position(pub Point2);

impl Position {
    /// Construct a new position at provided coordinates.
    #[inline(always)]
    pub fn new(x: CoordT, y: CoordT) -> Self {
        Self(na::point![x, y])
    }

    /// Convert the integer position into a floating point position within the provided range.
    pub fn to_range(self, min: Vec2, max: Vec2) -> Vec2 {
        let range = (max - min).abs();
        let middle = (min + max) / 2.0;
        // these now vary between -1 and 1
        let x = self.0[0] as f32 / CoordT::MAX as f32;
        let y = self.0[1] as f32 / CoordT::MAX as f32;
        let result = middle + range * Vec2::new(x, y) * 0.5;
        result
    }
}

impl AsRef<Point2> for Position {
    #[inline(always)]
    fn as_ref(&self) -> &Point2 {
        &self.0
    }
}

impl AsMut<Point2> for Position {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Point2 {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unspill_add() {
        let mut a = CoordT::MAX - 100;

        assert_eq!(unspill_add(&mut a, 60), 0);
        assert_eq!(unspill_add(&mut a, 60), 20);

        a = CoordT::MIN + 100;
        assert_eq!(unspill_add(&mut a, -60), 0);
        assert_eq!(unspill_add(&mut a, -60), -20);
    }
}

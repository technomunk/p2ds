//! Conservation of momentum. Ie the tendency of physical entities to keep their state.

use bevy::prelude::{Component, Query};

use super::coord::{spill_add, Position, Vector2, CoordT};

/// Change of position with time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Component)]
pub struct Velocity(pub Vector2);

/// Conservation of energy in a system. Each entity with velocity tries to keep it as much as possible.
pub fn momentum(mut entities: Query<(&mut Position, &mut Velocity)>) {
    for (mut position, mut velocity) in entities.iter_mut() {
        let spill = spill_add(&mut position.0, velocity.0);

        for i in 0..2 {
            if spill[i] != 0 {
                velocity.0[i] /= -2;
            }
        }
    }
}

impl Velocity {
    /// Construct a new velocity with provided values.
    #[inline(always)]
    pub fn new(x: CoordT, y: CoordT) -> Self {
        Self(Vector2::new(x, y))
    }
}

impl AsRef<Vector2> for Velocity {
    #[inline(always)]
    fn as_ref(&self) -> &Vector2 {
        &self.0
    }
}

impl AsMut<Vector2> for Velocity {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Vector2 {
        &mut self.0
    }
}

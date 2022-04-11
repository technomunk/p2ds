//! Conservation of momentum. Ie the tendency of physical entities to keep their state.

use bevy::prelude::{Component, Query};

use crate::physics::coord::P1;

use super::coord::{Position, P2};

/// Change of position with time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Component)]
pub struct Velocity(pub P2);

impl AsRef<P2> for Velocity {
    #[inline(always)]
    fn as_ref(&self) -> &P2 {
        &self.0
    }
}

impl AsMut<P2> for Velocity {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut P2 {
        &mut self.0
    }
}

/// Conservation of energy in a system. Each entity with velocity tries to keep it as much as possible.
pub fn momentum(mut entities: Query<(&mut Position, &mut Velocity)>) {
    const FRICTION_LOSS: i32 = 1 << 16;
    for (mut position, mut velocity) in entities.iter_mut() {
        position.0 = position.0.saturating_add(velocity.0);

        if velocity.0.x >= 0 {
            if velocity.0.x > FRICTION_LOSS {
                velocity.0.x -= FRICTION_LOSS;
            } else {
                velocity.0.x = 0;
            }
        } else {
            if velocity.0.x < -FRICTION_LOSS {
                velocity.0.x += FRICTION_LOSS;
            } else {
                velocity.0.x = 0;
            }
        }

        if velocity.0.y >= 0 {
            if velocity.0.y > FRICTION_LOSS {
                velocity.0.y -= FRICTION_LOSS;
            } else {
                velocity.0.y = 0;
            }
        } else {
            if velocity.0.y < -FRICTION_LOSS {
                velocity.0.y += FRICTION_LOSS;
            } else {
                velocity.0.y = 0;
            }
        }

        bounce(&mut velocity.0.x, position.0.x);
        bounce(&mut velocity.0.y, position.0.y);
    }
}

fn bounce(velocity: &mut P1, position: P1) {
    if (position == P1::MIN && *velocity < 0) || (position == P1::MAX && *velocity > 0) {
        *velocity = *velocity / -16 * 15;
    }
}

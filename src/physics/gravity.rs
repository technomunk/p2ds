//! Omnipresent downward force.

use bevy::prelude::*;

use super::momentum::Velocity;

/// Add velocity facing downwards.
pub fn gravity(mut entities: Query<&mut Velocity>) {
    for mut velocity in entities.iter_mut() {
        velocity.0.y = velocity.0.y.saturating_sub(1 << 20);
    }
}

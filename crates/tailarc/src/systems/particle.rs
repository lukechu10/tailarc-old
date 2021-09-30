use std::time::Duration;

use bevy_core::{Time, Timer};
use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{ParticleBundle, ParticleLifetime, Position, Renderable};

pub struct ParticleRequest {
    position: Position,
    renderable: Renderable,
    lifetime: Duration,
}

#[derive(Default)]
pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>,
}

impl ParticleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request(&mut self, position: Position, renderable: Renderable, lifetime: Duration) {
        self.requests.push(ParticleRequest {
            position,
            renderable,
            lifetime,
        });
    }
}

pub fn spawn_particles_system(
    mut commands: Commands,
    mut particle_builder: ResMut<ParticleBuilder>,
) {
    for req in particle_builder.requests.drain(..) {
        commands.spawn_bundle(ParticleBundle {
            particle_lifetime: ParticleLifetime {
                timer: Timer::new(req.lifetime, false),
            },
            position: req.position,
            renderable: req.renderable,
        });
    }
}

/// Renders particles and deletes dead particles.
pub fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut ParticleLifetime, &Position, &Renderable)>,
) {
    for (entity, mut lifetime, pos, renderable) in particles.iter_mut() {
        // Age out particles.
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

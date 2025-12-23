use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::{GameState, MAX_X, MAX_Y, MIN_X, MIN_Y};

const ACCELERATION: f32 = -1000.0;
const GROUND_Y: f32 = MIN_Y + 100.0;

pub(crate) struct DebrisPlugin;

impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<DebrisData>(
            serde_json::from_str(include_str!("../assets/debris.json"))
                .expect("expected correctly format debris definition"),
        )
        .add_systems(Update, fall.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, Default)]
pub(crate) struct Debris {
    velocity: f32,
}

impl Debris {
    pub(crate) fn new(
        definition: &DebrisDefinition,
        asset_server: &AssetServer,
        transform: Transform,
    ) -> impl Bundle + use<> {
        (
            Debris::default(),
            Sprite::from_image(asset_server.load(definition.sprite_path.clone())),
            transform.with_scale(Vec3::splat(definition.scale)),
        )
    }

    pub(crate) fn new_random(data: &DebrisData, asset_server: &AssetServer) -> impl Bundle {
        let idx = rand::rng().random_range(0..data.definitions.len());
        let x = rand::rng().random_range(MIN_X..MAX_X);
        Debris::new(
            &data.definitions[idx],
            asset_server,
            Transform::from_translation(Vec3::new(x as f32, MAX_Y + 50., 1.)),
        )
    }
}

fn fall(
    mut commands: Commands,
    mut debris: Query<(&mut Transform, &mut Debris, Entity)>,
    time: Res<Time>,
) {
    for (mut transform, mut debris, entity) in debris.iter_mut() {
        if transform.translation.y <= GROUND_Y {
            commands.entity(entity).despawn();
            continue;
        }

        let translation = debris.velocity * time.delta_secs();
        debris.velocity += ACCELERATION * time.delta_secs();
        transform.translation.y += translation
    }
}

#[derive(Serialize, Deserialize, Resource)]
pub(crate) struct DebrisData {
    definitions: Vec<DebrisDefinition>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DebrisDefinition {
    name: String,
    sprite_path: String,
    scale: f32,
}

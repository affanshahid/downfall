use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::{InGameState, ScreenConstraints};

const ACCELERATION: f32 = -1000.0;
const GROUND_Y_DELTA: f32 = 100.0;

pub(crate) struct DebrisPlugin;

impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<DebrisData>(
            serde_json::from_str(include_str!("../assets/debris.json"))
                .expect("expected correctly formatted debris definition"),
        )
        .add_systems(Update, fall.run_if(in_state(InGameState::Running)))
        .add_systems(
            Update,
            handle_resize
                .run_if(in_state(InGameState::Running).and(resource_changed::<ScreenConstraints>)),
        );
    }
}

#[derive(Component, Default)]
pub(crate) struct Debris {
    pub(crate) definition_idx: usize,
    velocity: f32,
}

impl Debris {
    pub(crate) fn new(
        definition_idx: usize,
        definition: &DebrisDefinition,
        asset_server: &AssetServer,
        constraints: ScreenConstraints,
        transform: Transform,
    ) -> impl Bundle + use<> {
        (
            Debris {
                definition_idx,
                velocity: 0.0,
            },
            Sprite::from_image(asset_server.load(definition.sprite_path.clone())),
            transform.with_scale(Vec3::splat(definition.scale * constraints.scale)),
            // Uncomment to visualize collision rectangles
            // children![(
            //     Sprite::from_color(
            //         Color::srgba(1.0, 1.0, 1.0, 0.3),
            //         Vec2::new(definition.coll_width, definition.coll_height)
            //     ),
            //     Transform::default().with_scale(Vec3::splat(1.0 / definition.scale))
            // )],
        )
    }

    pub(crate) fn new_random(
        data: &DebrisData,
        constraints: ScreenConstraints,
        asset_server: &AssetServer,
    ) -> impl Bundle {
        let idx = rand::rng().random_range(0..data.definitions.len());
        let x = rand::rng().random_range(constraints.min_x..constraints.max_x);
        Debris::new(
            idx,
            &data.definitions[idx],
            asset_server,
            constraints,
            Transform::from_translation(Vec3::new(
                x as f32,
                constraints.max_y + (50. * constraints.scale),
                1.,
            )),
        )
    }
}

fn fall(
    mut commands: Commands,
    mut debris: Query<(&mut Transform, &mut Debris, Entity)>,
    time: Res<Time>,
    constraints: Res<ScreenConstraints>,
) {
    let ground_y = constraints.min_y + (GROUND_Y_DELTA * constraints.scale);

    for (mut transform, mut debris, entity) in debris.iter_mut() {
        if transform.translation.y <= ground_y {
            commands.entity(entity).despawn();
            continue;
        }

        let translation = debris.velocity * time.delta_secs();
        debris.velocity += ACCELERATION * constraints.scale * time.delta_secs();
        transform.translation.y += translation
    }
}

fn handle_resize(
    constraints: Res<ScreenConstraints>,
    debris_data: Res<DebrisData>,
    mut debris: Query<(&mut Transform, &Debris)>,
    mut previous_scale: Local<Option<f32>>,
) {
    let scale_ratio = match *previous_scale {
        Some(prev) => constraints.scale / prev,
        None => constraints.scale,
    };

    for (mut transform, debris_el) in debris.iter_mut() {
        let def = &debris_data.definitions[debris_el.definition_idx];
        transform.scale = Vec3::splat(constraints.scale * def.scale);
        transform.translation.y *= scale_ratio;
        transform.translation.x *= scale_ratio;
    }

    *previous_scale = Some(constraints.scale);
}

#[derive(Serialize, Deserialize, Resource)]
pub(crate) struct DebrisData {
    pub(crate) definitions: Vec<DebrisDefinition>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct DebrisDefinition {
    pub(crate) name: String,
    pub(crate) sprite_path: String,
    pub(crate) scale: f32,
    pub(crate) coll_width: f32,
    pub(crate) coll_height: f32,
}

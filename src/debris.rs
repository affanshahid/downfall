use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub(crate) struct DebrisPlugin;

impl Plugin for DebrisPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<DebrisData>(
            serde_json::from_str(include_str!("../assets/debris.json"))
                .expect("expected correctly format debris definition"),
        );
    }
}

#[derive(Component)]
pub(crate) struct Debris;

impl Debris {
    pub(crate) fn new(
        definition: &DebrisDefinition,
        asset_server: &AssetServer,
    ) -> impl Bundle + use<> {
        (
            Debris,
            Sprite::from_image(asset_server.load(definition.sprite_path.clone())),
            Transform::default().with_scale(Vec3::splat(definition.scale)),
        )
    }

    pub(crate) fn new_random(data: &DebrisData, asset_server: &AssetServer) -> impl Bundle {
        let idx = rand::rng().random_range(0..data.definitions.len());
        Debris::new(&data.definitions[idx], asset_server)
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

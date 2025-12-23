use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

use crate::game::InGameState;

#[derive(Error, Debug)]
#[error("{0}")]
pub(crate) struct SpritesheetParsingError(String);

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationTextureAtlasLayout>()
            .add_systems(Update, animate.run_if(in_state(InGameState::Running)));
    }
}

#[derive(Component)]
#[require(Sprite)]
pub(crate) struct AnimatedSprite {
    pub(crate) animation_name: String,
    layout: Handle<AnimationTextureAtlasLayout>,
    timer: Timer,
}

impl AnimatedSprite {
    pub(crate) fn new(
        image: Handle<Image>,
        layout_handle: Handle<AnimationTextureAtlasLayout>,
        layout: &AnimationTextureAtlasLayout,
        animation: &str,
        fps: u8,
    ) -> impl Bundle + use<> {
        (
            Sprite::from_atlas_image(
                image,
                TextureAtlas {
                    layout: layout.raw_layout.clone(),
                    index: layout.get_first(animation).unwrap_or(0),
                },
            ),
            AnimatedSprite {
                layout: layout_handle,
                animation_name: animation.to_string(),
                timer: Timer::from_seconds(1.0 / fps as f32, TimerMode::Repeating),
            },
        )
    }
}

fn animate(
    mut query: Query<(&mut AnimatedSprite, &mut Sprite)>,
    time: Res<Time>,
    layouts: Res<Assets<AnimationTextureAtlasLayout>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        if !animation.timer.tick(time.delta()).just_finished() {
            continue;
        };

        let Some(layout) = layouts.get(&animation.layout) else {
            continue;
        };

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        let Some(next_idx) = layout.cycle_next_multi(
            &animation.animation_name,
            atlas.index,
            animation.timer.times_finished_this_tick() as usize,
        ) else {
            continue;
        };

        atlas.index = next_idx;
    }
}

#[derive(Asset, TypePath, Clone)]
pub(crate) struct AnimationTextureAtlasLayout {
    raw_layout: Handle<TextureAtlasLayout>,
    indices: BTreeMap<String, Vec<usize>>,
}

impl AnimationTextureAtlasLayout {
    pub(crate) fn from_json(
        json: &str,
        assets: &mut Assets<TextureAtlasLayout>,
    ) -> Result<Self, BevyError> {
        let mut raw = TextureAtlasLayout::new_empty(UVec2::ZERO);
        let spritesheet_data: SpritesheetData = serde_json::from_str(json)?;

        let mut temp_indices_map: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();

        for texture in spritesheet_data.sub_texture {
            let idx = raw.add_texture(URect::from_corners(
                UVec2::new(texture.x as u32, texture.y as u32),
                UVec2::new(
                    (texture.x + texture.width) as u32,
                    (texture.y + texture.height) as u32,
                ),
            ));
            let mut split = texture.name.split("_");
            let animation = split.next().ok_or(SpritesheetParsingError(
                "Texture name format incorrect".to_string(),
            ))?;
            let position: usize = split
                .next()
                .ok_or("Texture name format incorrect".to_string())?
                .parse()?;

            temp_indices_map
                .entry(animation.to_string())
                .and_modify(|e| e.push((position, idx)))
                .or_insert(vec![]);
        }

        let mut indices = BTreeMap::new();
        for (name, mut list) in temp_indices_map.into_iter() {
            list.sort_by(|a, b| a.0.cmp(&b.0));
            indices.insert(name, list.into_iter().map(|e| e.1).collect());
        }

        let handle = assets.add(raw);

        Ok(AnimationTextureAtlasLayout {
            raw_layout: handle,
            indices,
        })
    }

    pub(crate) fn get_first(&self, name: &str) -> Option<usize> {
        self.indices.get(name).and_then(|v| v.first().copied())
    }

    #[allow(unused)]
    pub(crate) fn cycle_next(&self, name: &str, current: usize) -> Option<usize> {
        self.cycle_next_multi(name, current, 1)
    }

    pub(crate) fn cycle_next_multi(
        &self,
        name: &str,
        current: usize,
        count: usize,
    ) -> Option<usize> {
        let indices = self.indices.get(name)?;

        let Some(cur_idx) = indices.iter().position(|&e| e == current) else {
            return indices.first().copied();
        };

        Some(indices[(cur_idx + count) % indices.len()])
    }

    #[allow(unused)]
    pub(crate) fn get_nth(&self, name: &str, n: usize) -> Option<usize> {
        self.indices.get(name)?.get(n).copied()
    }

    #[allow(unused)]
    pub(crate) fn has_animation(&self, name: &str) -> bool {
        self.indices.contains_key(name)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpritesheetData {
    image_path: String,
    #[serde(rename = "SubTexture")]
    sub_texture: Vec<SubTexture>,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubTexture {
    frame_width: i64,
    y: i64,
    frame_height: i64,
    width: i64,
    frame_x: i64,
    height: i64,
    name: String,
    frame_y: i64,
    x: i64,
}

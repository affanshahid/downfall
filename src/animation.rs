use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub(crate) struct SpritesheetParsingError(String);

pub(crate) struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, _app: &mut App) {}
}

pub(crate) struct NamedTextureAtlasLayout {
    pub(crate) layout: Handle<TextureAtlasLayout>,
    indices: BTreeMap<String, Vec<usize>>,
}

impl NamedTextureAtlasLayout {
    pub(crate) fn from_json(
        json: &str,
        assets: &mut ResMut<Assets<TextureAtlasLayout>>,
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

        Ok(NamedTextureAtlasLayout {
            layout: handle,
            indices,
        })
    }

    pub(crate) fn get_first(&self, name: &str) -> Option<usize> {
        self.indices.get(name).and_then(|v| v.first().copied())
    }

    pub(crate) fn get_next(&self, name: &str, current: usize) -> Option<usize> {
        let indices = self.indices.get(name)?;
        let Ok(cur_idx) = indices.binary_search(&current) else {
            return indices.first().copied();
        };

        if cur_idx == indices.len() - 1 {
            indices.first().copied()
        } else {
            Some(indices[cur_idx + 1])
        }
    }

    pub(crate) fn get_nth(&self, name: &str, n: usize) -> Option<usize> {
        self.indices.get(name)?.get(n).copied()
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

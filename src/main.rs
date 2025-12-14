#![warn(clippy::all)]

use bevy::prelude::*;
use downfall::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}

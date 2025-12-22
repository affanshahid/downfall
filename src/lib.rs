#![warn(clippy::all)]
#![allow(clippy::new_ret_no_self)]

mod animation;
mod game;
mod level;
mod menu;
mod player;

pub use game::GamePlugin;

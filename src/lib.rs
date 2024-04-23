#![allow(clippy::type_complexity)]

mod actions;
mod book;
mod book_content;
mod graph;
mod loading;
mod menu;
mod utils;

use crate::actions::ActionsPlugin;
use crate::book::BookPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((
                LoadingPlugin,
                MenuPlugin,
                ActionsPlugin,
                BookPlugin,
                AudioPlugin,
            ))
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

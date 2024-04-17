use crate::{
    resources::{Animations, BookFont, Illustrations},
    GameState,
};
use bevy::{asset::LoadState, prelude::*};

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Loading),
            (load_book_model, load_font, load_illustrations),
        )
        .add_systems(Update, check_loading.run_if(in_state(GameState::Loading)));
    }
}

fn load_illustrations(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(Illustrations::new(&server));
}

fn load_book_model(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: server.load("models/book.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.insert_resource(Animations(vec![
        server.load("models/book.gltf#Animation0"),
        server.load("models/book.gltf#Animation1"),
    ]));
}

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let font_handle = asset_server.load("fonts/BouwsUnc.ttf");
    let font_handle = asset_server.load("fonts/OldLondon.ttf");
    commands.insert_resource(BookFont(font_handle));
}

// TODO: Check more assets.
fn check_loading(
    server: Res<AssetServer>,
    illustrations: Res<Illustrations>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let load_state = server
        .get_load_state(&illustrations.0.get("normal-dragon").unwrap().clone())
        .unwrap();
    if load_state == LoadState::Loaded {
        next_state.set(GameState::Menu);
    }
}

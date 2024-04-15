use crate::{
    resources::{Animations, BookFont},
    GameState,
};
use bevy::{asset::LoadState, prelude::*};
use bevy_talks::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Loading),
            (load_talks, load_book_model, load_font),
        )
        .add_systems(Update, check_loading.run_if(in_state(GameState::Loading)));
    }
}

#[derive(Resource)]
pub struct SimpleTalkAsset {
    pub handle: Handle<TalkData>,
}

fn load_talks(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<TalkData> = server.load("talks/hello.talk.ron");
    commands.insert_resource(SimpleTalkAsset { handle });
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
    let font_handle = asset_server.load("fonts/BouwsUnc.ttf");
    commands.insert_resource(BookFont(font_handle));
}

fn check_loading(
    server: Res<AssetServer>,
    simple_talk_asset: Res<SimpleTalkAsset>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let load_state = server.get_load_state(&simple_talk_asset.handle).unwrap();
    if load_state == LoadState::Loaded {
        next_state.set(GameState::Menu);
    }
}

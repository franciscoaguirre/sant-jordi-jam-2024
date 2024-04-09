use crate::GameState;
use bevy::{asset::LoadState, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_talks::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_talks)
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

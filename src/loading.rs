use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<FontAssets>()
                .load_collection::<ModelAssets>()
                .load_collection::<AnimationAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Enchanted Land.otf")]
    pub normal: Handle<Font>,
    #[asset(path = "fonts/GoudyIni.ttf")]
    pub initials: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/normal-dragon.png")]
    pub normal_dragon: Handle<Image>,
    #[asset(path = "textures/sant-jordi-disguised-as-dragon.png")]
    pub sant_jordi_disguised_as_dragon: Handle<Image>,
    #[asset(path = "textures/princess-go-kill-dragon.png")]
    pub princess_go_kill_dragon: Handle<Image>,
    #[asset(path = "textures/princess-excited-to-be-picked.png")]
    pub princess_excited_to_be_picked: Handle<Image>,
    #[asset(path = "textures/dragon-with-cow.png")]
    pub dragon_with_cow: Handle<Image>,
    #[asset(path = "textures/jordi-dragon-with-cow.png")]
    pub jordi_dragon_with_cow: Handle<Image>,
    #[asset(path = "textures/king-picks-princess.png")]
    pub king_picks_princess: Handle<Image>,
    #[asset(path = "textures/sant-jordi-making-marshmallows.png")]
    pub sant_jordi_making_marshmallows: Handle<Image>,
    #[asset(path = "textures/sant-jordi-warrior.png")]
    pub sant_jordi_warrior: Handle<Image>,
    #[asset(path = "textures/sant-jordi-roses.png")]
    pub sant_jordi_roses: Handle<Image>,
    #[asset(path = "textures/princess-punches-jordi-dragon.png")]
    pub princess_punches_jordi_dragon: Handle<Image>,
    #[asset(path = "textures/jordi-dragon-confesses.png")]
    pub jordi_dragon_confesses: Handle<Image>,
    #[asset(path = "textures/princess-x-dragon.png")]
    pub princess_x_dragon: Handle<Image>,
    #[asset(path = "textures/princess-analyzing-jordi-dragon.png")]
    pub princess_analyzing_jordi_dragon: Handle<Image>,
    #[asset(path = "textures/princess-unmasks-jordi-dragon.png")]
    pub princess_unmasks_jordi_dragon: Handle<Image>,
    #[asset(path = "textures/princess-leaves-with-dragon.png")]
    pub dragon_returns_from_holidays: Handle<Image>,
    #[asset(path = "textures/sant-jordi-fighting-alone.png")]
    pub sant_jordi_fighting_alone: Handle<Image>,
    #[asset(path = "textures/sant-jordi-with-dragon-head.png")]
    pub sant_jordi_with_dragon_head: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ModelAssets {
    #[asset(path = "models/book.gltf#Scene0")]
    pub book: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct AnimationAssets {
    #[asset(path = "models/book.gltf#Animation0")]
    pub page_flip: Handle<AnimationClip>,
}

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
                .load_collection::<Illustrations>()
                .load_collection::<UiTextures>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Seagram tfb.ttf")]
    pub normal: Handle<Font>,
    #[asset(path = "fonts/GoudyIni.ttf")]
    pub first_letter: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(path = "audio/background-music.ogg")]
    pub background_music: Handle<AudioSource>,
    #[asset(path = "audio/page-flip.ogg")]
    pub page_flip: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct UiTextures {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/play-button.png")]
    pub play_button: Handle<Image>,
    #[asset(path = "textures/fancy-underline.png")]
    pub fancy_underline: Handle<Image>,
    #[asset(path = "textures/rabbit-troubadour.png")]
    pub rabbit_troubadour: Handle<Image>,
    #[asset(path = "textures/roses-frame.png")]
    pub roses_frame: Handle<Image>,
    #[asset(path = "textures/keyboard.png")]
    pub keyboard: Handle<Image>,
    #[asset(path = "textures/mouse.png")]
    pub mouse: Handle<Image>,
    #[asset(path = "textures/warrior-bunny.png")]
    pub warrior_bunny: Handle<Image>,
    #[asset(path = "textures/bat.png")]
    pub bat: Handle<Image>,
    #[asset(path = "textures/cat.png")]
    pub cat: Handle<Image>,
    #[asset(path = "textures/snail-boy.png")]
    pub snail_boy: Handle<Image>,
    #[asset(path = "textures/end-button.png")]
    pub end_button: Handle<Image>,
    #[asset(path = "textures/end-button-hover.png")]
    pub end_button_hover: Handle<Image>,
    #[asset(path = "textures/arrow.png")]
    pub arrow: Handle<Image>,
}

// TODO: Not using IMG_25.png
#[derive(AssetCollection, Resource)]
pub struct Illustrations {
    #[asset(path = "illustrations/normal-dragon.png")]
    pub normal_dragon: Handle<Image>,
    #[asset(path = "illustrations/sant-jordi-disguised-as-dragon.png")]
    pub sant_jordi_disguised_as_dragon: Handle<Image>,
    #[asset(path = "illustrations/princess-go-kill-dragon.png")]
    pub princess_go_kill_dragon: Handle<Image>,
    #[asset(path = "illustrations/princess-excited-to-be-picked.png")]
    pub princess_excited_to_be_picked: Handle<Image>,
    #[asset(path = "illustrations/dragon-with-cow.png")]
    pub dragon_with_cow: Handle<Image>,
    #[asset(path = "illustrations/jordi-dragon-with-cow.png")]
    pub jordi_dragon_with_cow: Handle<Image>,
    #[asset(path = "illustrations/king-picks-princess.png")]
    pub king_picks_princess: Handle<Image>,
    #[asset(path = "illustrations/sant-jordi-making-marshmallows.png")]
    pub sant_jordi_making_marshmallows: Handle<Image>,
    #[asset(path = "illustrations/sant-jordi-warrior.png")]
    pub sant_jordi_warrior: Handle<Image>,
    #[asset(path = "illustrations/sant-jordi-roses.png")]
    pub sant_jordi_roses: Handle<Image>,
    #[asset(path = "illustrations/princess-punches-jordi-dragon.png")]
    pub princess_punches_jordi_dragon: Handle<Image>,
    #[asset(path = "illustrations/jordi-dragon-confesses.png")]
    pub jordi_dragon_confesses: Handle<Image>,
    #[asset(path = "illustrations/princess-x-dragon.png")]
    pub princess_x_dragon: Handle<Image>,
    #[asset(path = "illustrations/princess-analyzing-jordi-dragon.png")]
    pub princess_analyzing_jordi_dragon: Handle<Image>,
    #[asset(path = "illustrations/princess-unmasks-jordi-dragon.png")]
    pub princess_unmasks_jordi_dragon: Handle<Image>,
    #[asset(path = "illustrations/princess-leaves-with-dragon.png")]
    pub dragon_returns_from_holidays: Handle<Image>,
    #[asset(path = "illustrations/sant-jordi-fighting-alone.png")]
    pub sant_jordi_fighting_alone: Handle<Image>,
    #[asset(path = "illustrations/sant-jordi-with-dragon-head.png")]
    pub sant_jordi_with_dragon_head: Handle<Image>,
    #[asset(path = "illustrations/sensual-dragon-coming-out-of-cave.png")]
    pub sensual_dragon_coming_out_of_cave: Handle<Image>,
    #[asset(path = "illustrations/jordi-dragon-accepts-princess.png")]
    pub jordi_dragon_accepts_princess: Handle<Image>,
    #[asset(path = "illustrations/princess-dragon.png")]
    pub princess_dragon: Handle<Image>,
    #[asset(path = "illustrations/dragon-x-jordi-dragon.png")]
    pub dragon_x_jordi_dragon: Handle<Image>,
    #[asset(path = "illustrations/jordi-dragon-rejects-princess.png")]
    pub jordi_dragon_rejects_princess: Handle<Image>,
    #[asset(path = "illustrations/dragon-chases-jordi-dragon.png")]
    pub dragon_chases_jordi_dragon: Handle<Image>,
    #[asset(path = "illustrations/princess-thinking.png")]
    pub princess_thinking: Handle<Image>,
    #[asset(path = "illustrations/dragon-x-sant-jordi.png")]
    pub dragon_x_sant_jordi: Handle<Image>,
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

use std::collections::HashMap;

use bevy::log;
use bevy::prelude::*;
use bevy_talks::prelude::*;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct BookFont(pub Handle<Font>);

#[derive(Resource)]
pub struct SimpleTalkAsset {
    pub handle: Handle<TalkData>,
}

/// Illustrations for the options in the game.
/// Maps node indices to arrays of illustrations.
/// The array of illustrations corresponds with the array of options.
#[derive(Resource)]
pub struct Illustrations(pub HashMap<usize, Vec<Illustration>>);

impl Illustrations {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let mut map = HashMap::new();
        map.insert(
            2,
            vec![
                Illustration::new("textures/normal-dragon.png", asset_server),
                Illustration::new("textures/sant-jordi-disguised-as-dragon.png", asset_server),
            ],
        );
        Self(map)
    }
}

#[derive(Resource)]
pub struct Illustration {
    pub handle: Handle<Image>,
}

impl Illustration {
    pub fn new(file: &'static str, asset_server: &Res<AssetServer>) -> Self {
        Self {
            handle: asset_server.load(file),
        }
    }
}

#[derive(Resource, Default)]
pub struct BookStateMachine {
    pub state: BookState,
}

impl BookStateMachine {
    /// The state transition function of the state machine.
    pub fn transition(&mut self, transition: &BookTransition) {
        use BookState::*;
        use BookTransition::*;
        log::info!("Transition: ({:?}, {:?})", &self.state, transition);
        self.state = match (&self.state, transition) {
            (Start, ShowFirstTalk) => ShowingFirstTalk,
            (ShowingFirstTalk, StartChoosing) => Choosing,
            (Choosing, ChooseOption { index }) => ShowingChoice {
                chosen_option: *index,
            },
            (ShowingChoice { chosen_option }, StartPageFlip) => PageFlipStarted {
                chosen_option: *chosen_option,
            },
            (PageFlipStarted { chosen_option }, EndPageFlip) => PageFlipEnded {
                chosen_option: *chosen_option,
            },
            (PageFlipEnded { .. }, Redraw) => DrawingOptions,
            (DrawingOptions, OptionsDrawn) => Choosing,
            (state, transition) => {
                panic!(
                    "Invalid state transition: ({:?}, {:?}). Shouldn't be allowed. ",
                    state, transition
                )
            }
        };
        log::info!("New state: {:?}", self.state);
    }
}

#[derive(Resource, Debug)]
pub enum BookState {
    Start,
    ShowingFirstTalk,
    Choosing,
    ShowingChoice { chosen_option: usize },
    PageFlipStarted { chosen_option: usize },
    PageFlipEnded { chosen_option: usize },
    DrawingOptions,
}

impl Default for BookState {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Event, Debug)]
pub enum BookTransition {
    ShowFirstTalk,
    StartChoosing,
    ChooseOption { index: usize },
    StartPageFlip,
    EndPageFlip,
    Redraw,
    OptionsDrawn,
}

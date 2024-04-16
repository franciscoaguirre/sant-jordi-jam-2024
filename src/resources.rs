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
            (Choosing, ChooseOption) => ShowingChoice,
            (ShowingChoice, StartPageFlip) => PageFlipStarted,
            (PageFlipStarted, EndPageFlip) => PageFlipEnded,
            (PageFlipEnded, Redraw) => Choosing,
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
    ShowingChoice,
    PageFlipStarted,
    PageFlipEnded,
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
    ChooseOption,
    StartPageFlip,
    EndPageFlip,
    Redraw,
}

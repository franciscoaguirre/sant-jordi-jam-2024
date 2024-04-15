use bevy::prelude::*;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct BookFont(pub Handle<Font>);

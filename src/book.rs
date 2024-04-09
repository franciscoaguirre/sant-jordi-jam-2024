use bevy::prelude::*;
use bevy_talks::prelude::*;

use crate::{loading::SimpleTalkAsset, GameState};

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_talk, setup_book))
            .add_systems(
                Update,
                (interact, print_text).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct MainText;

fn setup_book(mut commands: Commands) {
    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(90.0),
                height: Val::Percent(90.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|children| {
            children.spawn((
                TextBundle::from_section(
                    "Hola",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
                MainText,
            ));
        });
}

fn setup_talk(
    mut commands: Commands,
    talks: Res<Assets<TalkData>>,
    simple_talk_asset: Res<SimpleTalkAsset>,
) {
    let simple_talk = talks.get(&simple_talk_asset.handle).unwrap();
    let talk_builder = TalkBuilder::default().fill_with_talk_data(simple_talk);

    commands.spawn_talk(talk_builder);
}

fn interact(
    input: Res<Input<KeyCode>>,
    mut next_action_events: EventWriter<NextNodeRequest>,
    talks: Query<Entity, With<Talk>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_action_events.send(NextNodeRequest::new(talks.single()));
    }
}

fn print_text(
    mut text_events: EventReader<TextNodeEvent>,
    mut label: Query<&mut Text, With<MainText>>,
) {
    for text_event in text_events.read() {
        label.single_mut().sections[0].value = text_event.text.clone();
    }
}

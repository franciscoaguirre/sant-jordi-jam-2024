use bevy::prelude::*;
use bevy_talks::prelude::*;

use crate::{
    loading::SimpleTalkAsset,
    resources::{Animations, BookFont},
    GameState,
};

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_talk, setup_book))
            .add_systems(
                Update,
                (start_animation, interact, print_text, print_choices)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct FirstPage;

#[derive(Component)]
pub struct SecondPage;

fn setup_book(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 9.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(8.0),
                    right: Val::Percent(8.0),
                    ..default()
                },
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            // First page.
            children.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
                FirstPage,
            ));

            // Second page.
            children.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
                SecondPage,
            ));
        });
}

fn start_animation(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in players.iter_mut() {
        player.play(animations.0[0].clone_weak()).repeat();
    }
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
        next_action_events.send(NextNodeRequest::new(talks.single()));
    }
}

fn print_text(
    mut text_events: EventReader<TextNodeEvent>,
    first_page: Query<Entity, With<FirstPage>>,
    mut commands: Commands,
    book_font: Res<BookFont>,
) {
    for text_event in text_events.read() {
        let first_page = first_page.single();
        commands.entity(first_page).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &text_event.text,
                TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    font: book_font.0.clone(),
                },
            ));
        });
    }
}

fn print_choices(
    mut choices_events: EventReader<ChoiceNodeEvent>,
    second_page: Query<Entity, With<SecondPage>>,
    mut commands: Commands,
) {
    for choices_event in choices_events.read() {
        let second_page = second_page.single();
        commands.entity(second_page).with_children(|parent| {
            for (_, choice) in choices_event.choices.iter().enumerate() {
                parent.spawn(TextBundle::from_section(
                    &choice.text,
                    TextStyle {
                        font_size: 20.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ));
            }
        });
    }
}

use bevy::prelude::*;
use bevy_talks::prelude::*;

use crate::{
    book_text,
    resources::{Animations, BookFont, BookState, BookStateMachine, BookTransition, Illustrations},
    GameState,
};

pub struct BookPlugin;
impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BookStateMachine>()
            .add_event::<BookTransition>()
            .add_systems(OnEnter(GameState::Playing), (setup_book, setup_talk))
            .add_systems(
                Update,
                (
                    show_first_talk,
                    flip_page,
                    transition_state,
                    print_text,
                    print_options,
                    choose_options,
                    clear_book_content,
                    draw_new_book_content,
                    page_flip_listener,
                    draw_options,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

const HOVER_BUTTON_COLOR: Color = Color::RED;
const NORMAL_BUTTON_COLOR: Color = Color::BLACK;
const PRESSED_BUTTON_COLOR: Color = Color::rgba(0.7, 0., 0., 1.);

#[derive(Component)]
pub struct Erasable;

#[derive(Component)]
pub struct FirstPage;

#[derive(Component)]
pub struct SecondPage;

/// An option of a Talk, signifying a fork in the road.
/// The inner number is the option id to advance the talk graph.
#[derive(Component)]
pub struct TalkOption(pub usize);

fn setup_talk(
    mut commands: Commands,
    mut event_writer: EventWriter<BookTransition>,
    illustrations: Res<Illustrations>,
) {
    let talk_builder = book_text::get_book_text(&illustrations);
    commands.spawn_talk(talk_builder);
    event_writer.send(BookTransition::ShowFirstTalk);
}

fn transition_state(
    mut events: EventReader<BookTransition>,
    mut book_state_machine: ResMut<BookStateMachine>,
) {
    for event in events.read() {
        book_state_machine.transition(event);
    }
}

fn clear_book_content(
    mut commands: Commands,
    book_state_machine: Res<BookStateMachine>,
    erasable_query: Query<Entity, With<Erasable>>,
) {
    // We can safely ignore `chosen_option` here, it's being handled in `page_flip_listener`.
    if let BookState::PageFlipStarted { chosen_option: _ } = book_state_machine.state {
        for entity in erasable_query.iter() {
            commands.get_entity(entity).unwrap().despawn_recursive();
        }
    }
}

fn draw_new_book_content(
    book_state_machine: Res<BookStateMachine>,
    mut choose_action_events: EventWriter<ChooseNodeRequest>,
    talks: Query<Entity, With<Talk>>,
    choices: Query<&ChoiceNode, With<CurrentNode>>,
    mut event_writer: EventWriter<BookTransition>,
) {
    if let BookState::PageFlipEnded { chosen_option } = book_state_machine.state {
        let talk_entity = talks.single();
        let choice_node = choices.single();
        // 1. Advance the talk graph on the `chosen_option` branch.
        choose_action_events.send(ChooseNodeRequest::new(
            talk_entity,
            choice_node.0[chosen_option].next,
        ));
        // 2. Send transition to draw next text node.
        event_writer.send(BookTransition::Redraw);
    }
}

/// Had to add this new system because you can't choose an option and ask for the next
/// node on the same system call.
fn draw_options(
    book_state_machine: Res<BookStateMachine>,
    talks: Query<Entity, With<Talk>>,
    mut next_action_events: EventWriter<NextNodeRequest>,
    mut event_writer: EventWriter<BookTransition>,
) {
    if let BookState::DrawingOptions = book_state_machine.state {
        let talk_entity = talks.single();
        // 3. Receive transition and request the next node to fill in the second page.
        next_action_events.send(NextNodeRequest::new(talk_entity));
        event_writer.send(BookTransition::OptionsDrawn);
    }
}

fn flip_page(
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer>,
    book_state_machine: Res<BookStateMachine>,
    mut event_writer: EventWriter<BookTransition>,
) {
    // We ignore `chosen_option` because this is not the place for it to be used.
    // It's going to be used in `draw_new_book_content`.
    if let BookState::ShowingChoice { .. } = book_state_machine.state {
        for mut player in players.iter_mut() {
            if keyboard_input.just_pressed(KeyCode::Space) {
                player.start(animations.0[0].clone_weak());
                event_writer.send(BookTransition::StartPageFlip);
            }
        }
    }
}

fn page_flip_listener(
    book_state_machine: Res<BookStateMachine>,
    mut event_writer: EventWriter<BookTransition>,
    players: Query<&AnimationPlayer>,
) {
    // We ignore `chosen_option` because this is not the place for it to be used.
    // It's going to be used in `draw_new_book_content`.
    if let BookState::PageFlipStarted { .. } = book_state_machine.state {
        for player in players.iter() {
            if player.is_finished() {
                event_writer.send(BookTransition::EndPageFlip);
            }
        }
    }
}

fn show_first_talk(
    mut next_action_events: EventWriter<NextNodeRequest>,
    talks: Query<Entity, With<Talk>>,
    book_state_machine: Res<BookStateMachine>,
    mut event_writer: EventWriter<BookTransition>,
) {
    if let BookState::ShowingFirstTalk = book_state_machine.state {
        next_action_events.send(NextNodeRequest::new(talks.single()));
        next_action_events.send(NextNodeRequest::new(talks.single()));
        event_writer.send(BookTransition::StartChoosing);
    }
}

fn choose_options(
    interaction_query: Query<
        (&Interaction, &Children, &TalkOption),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    book_state_machine: Res<BookStateMachine>,
    mut event_writer: EventWriter<BookTransition>,
) {
    if let BookState::Choosing = book_state_machine.state {
        for (interaction, children, talk_option) in interaction_query.iter() {
            let mut text = text_query.get_mut(children[0]).unwrap();
            match *interaction {
                Interaction::Hovered => {
                    text.sections[0].style.color = HOVER_BUTTON_COLOR;
                }
                Interaction::None => {
                    text.sections[0].style.color = NORMAL_BUTTON_COLOR;
                }
                Interaction::Pressed => {
                    text.sections[0].style.color = PRESSED_BUTTON_COLOR;
                    event_writer.send(BookTransition::ChooseOption {
                        index: talk_option.0,
                    });
                }
            }
        }
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
            parent.spawn((
                TextBundle::from_section(
                    &text_event.text,
                    TextStyle {
                        font_size: 30.0,
                        color: Color::BLACK,
                        font: book_font.0.clone(),
                    },
                ),
                Erasable,
            ));
        });
    }
}

fn print_options(
    mut choices_events: EventReader<ChoiceNodeEvent>,
    second_page: Query<Entity, With<SecondPage>>,
    mut commands: Commands,
    book_font: Res<BookFont>,
) {
    for choices_event in choices_events.read() {
        let second_page = second_page.single();
        commands.entity(second_page).with_children(|parent| {
            for (index, choice) in choices_event.choices.iter().enumerate() {
                parent
                    .spawn((
                        ButtonBundle {
                            background_color: Color::NONE.into(),
                            ..default()
                        },
                        TalkOption(index),
                        Erasable,
                    ))
                    .with_children(|children| {
                        children.spawn(TextBundle::from_section(
                            &choice.text,
                            TextStyle {
                                font_size: 20.0,
                                color: Color::BLACK,
                                font: book_font.0.clone(),
                            },
                        ));
                    });
            }
        });
    }
}

fn setup_book(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-0.5, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
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
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|children| {
            // First page.
            children.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(80.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        // border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    // border_color: Color::RED.into(),
                    ..default()
                },
                FirstPage,
            ));

            // Second page.
            children.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(30.0),
                        height: Val::Percent(80.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        // border: UiRect::all(Val::Px(2.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        margin: UiRect::left(Val::Px(40.0)),
                        ..default()
                    },
                    // border_color: Color::RED.into(),
                    ..default()
                },
                SecondPage,
            ));
        });
}

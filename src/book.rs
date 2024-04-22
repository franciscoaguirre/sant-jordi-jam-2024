#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    book_content::{self, BookGraph},
    graph::Node,
    loading::{AnimationAssets, AudioAssets, FontAssets, Illustrations, UiTextures},
    menu::{FirstPage, SecondPage},
    GameState,
};

pub const BUTTON_HOVER_COLOR: Color = Color::rgba(1., 0., 0., 0.5);
pub const BUTTON_NORMAL_COLOR: Color = Color::NONE;
pub const BUTTON_PRESSED_COLOR: Color = Color::rgba(0.7, 0., 0., 0.7);

pub const FIRST_LETTER_COLOR: Color = Color::rgba(0.235, 0.039, 0.337, 1.);

pub struct BookPlugin;
impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Transition>()
            .add_event::<AdvanceSimpleNode>()
            .add_event::<EraseEverything>()
            .add_event::<PageFlipEnded>()
            .add_event::<OptionChosen>()
            .add_systems(OnEnter(GameState::Playing), (setup_graph, setup_lifecycle))
            .add_systems(
                Update,
                (
                    transition_listener,
                    show_current_node_and_transition,
                    interact_with_options,
                    erase_everything_listener,
                    draw_chosen_option,
                    advance_simple_node_listener,
                    flip_page,
                    flip_page_listener,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Event)]
pub struct OptionChosen {
    index: usize,
    text: String,
    image: Option<Handle<Image>>,
}

#[derive(Event, Default)]
pub struct EraseEverything;

#[derive(Event, Default)]
pub struct AdvanceSimpleNode;

#[derive(Event, Default)]
pub struct PageFlipEnded;

#[derive(Event, Default)]
pub struct Transition;

#[derive(Resource)]
pub struct LifecycleManager(pub Lifecycle);

#[derive(Resource)]
pub enum Lifecycle {
    ShowNode,
    Choosing,
    Chosen,
    Transitioning,
    SimpleNode,
}

impl Lifecycle {
    pub fn next(&self) -> Self {
        use Lifecycle::*;
        match self {
            ShowNode => Choosing,
            Choosing => Chosen,
            Chosen => Transitioning,
            Transitioning => ShowNode,
            SimpleNode => Transitioning,
        }
    }
}

fn setup_lifecycle(mut commands: Commands) {
    commands.insert_resource(LifecycleManager(Lifecycle::ShowNode));
}

fn draw_chosen_option(
    mut commands: Commands,
    first_page: Query<Entity, With<FirstPage>>,
    second_page: Query<Entity, With<SecondPage>>,
    fonts: Res<FontAssets>,
    mut events: EventReader<OptionChosen>,
    mut graph: ResMut<BookGraph>,
) {
    for event in events.read() {
        let current_node = graph.get_current_node();
        let Node::Fork { choices, .. } = current_node else {
            unreachable!("An option was chosen, it's a fork.");
        };
        // TODO: I could get everything from `current_node`.
        let OptionChosen { index, text, image } = event;
        let chosen_option = &choices[*index];
        let mut first_page = commands.entity(first_page.single());
        first_page.with_children(|parent| {
            parent.spawn((get_formatted_text(text, &fonts), Erasable));
        });
        let mut second_page = commands.entity(second_page.single());
        second_page.with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    (chosen_option.additional_text)(&graph.context),
                    TextStyle {
                        font: fonts.normal.clone(),
                        font_size: 30.,
                        color: Color::BLACK,
                    },
                ),
                Erasable,
            ));
            if let Some(image) = image {
                parent.spawn((
                    ImageBundle {
                        image: image.clone().into(),
                        style: Style {
                            height: Val::Percent(50.),
                            ..default()
                        },
                        ..default()
                    },
                    Erasable,
                ));
            }
        });
        graph.choose(*index);
    }
}

fn erase_everything_listener(
    mut commands: Commands,
    erasable_query: Query<Entity, With<Erasable>>,
    mut events: EventReader<EraseEverything>,
) {
    for _ in events.read() {
        for entity in erasable_query.iter() {
            commands.get_entity(entity).unwrap().despawn_recursive();
        }
    }
}

fn flip_page(
    lifecycle: Res<LifecycleManager>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut AnimationPlayer>,
    animations: Res<AnimationAssets>,
    mut event_writer: EventWriter<Transition>,
    mut erase_everything: EventWriter<EraseEverything>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    if matches!(lifecycle.0, Lifecycle::Chosen | Lifecycle::SimpleNode)
        && keyboard_input.just_pressed(KeyCode::Space)
    {
        audio.play(audio_assets.page_flip.clone());
        for mut player in players.iter_mut() {
            player.start(animations.page_flip.clone());
            event_writer.send_default();
            erase_everything.send_default();
        }
    }
}

fn advance_simple_node_listener(
    mut events: EventReader<AdvanceSimpleNode>,
    mut graph: ResMut<BookGraph>,
) {
    for _ in events.read() {
        graph.advance();
    }
}

fn flip_page_listener(
    lifecycle: Res<LifecycleManager>,
    players: Query<&AnimationPlayer>,
    mut event_writer: EventWriter<Transition>,
) {
    if let Lifecycle::Transitioning = lifecycle.0 {
        for player in players.iter() {
            if player.is_finished() {
                event_writer.send_default();
            }
        }
    }
}

fn transition_listener(
    mut lifecycle: ResMut<LifecycleManager>,
    mut transition_events: EventReader<Transition>,
) {
    for _ in transition_events.read() {
        lifecycle.0 = lifecycle.0.next();
    }
}

fn show_current_node_and_transition(
    graph: Res<BookGraph>,
    mut lifecycle: ResMut<LifecycleManager>,
    mut transition: EventWriter<Transition>,
    mut advance_simple_node: EventWriter<AdvanceSimpleNode>,
    first_page: Query<Entity, With<FirstPage>>,
    second_page: Query<Entity, With<SecondPage>>,
    mut commands: Commands,
    fonts: Res<FontAssets>,
    textures: Res<UiTextures>,
) {
    if let Lifecycle::ShowNode = lifecycle.0 {
        let first_page = first_page.single();
        let second_page = second_page.single();
        let is_simple = show_current_node(
            &graph,
            first_page,
            second_page,
            &mut commands,
            &fonts,
            &textures,
        );
        if is_simple {
            lifecycle.0 = Lifecycle::SimpleNode;
            advance_simple_node.send_default();
        } else {
            transition.send_default();
        }
    }
}

fn interact_with_options(
    lifecycle: Res<LifecycleManager>,
    mut interaction_query: Query<
        (&Interaction, &ChoicesOption, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut option_chosen: EventWriter<OptionChosen>,
    mut transition: EventWriter<Transition>,
    mut erase_everything: EventWriter<EraseEverything>,
) {
    if let Lifecycle::Choosing = lifecycle.0 {
        for (interaction, choice, mut background_color) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Hovered => {
                    *background_color = BUTTON_HOVER_COLOR.into();
                }
                Interaction::None => {
                    *background_color = BUTTON_NORMAL_COLOR.into();
                }
                Interaction::Pressed => {
                    *background_color = BUTTON_PRESSED_COLOR.into();
                    option_chosen.send(OptionChosen {
                        index: choice.index,
                        text: choice.text.clone(),
                        image: choice.image.clone(),
                    });
                    erase_everything.send_default();
                    transition.send_default();
                }
            }
        }
    }
}

#[derive(Component)]
pub struct ChoicesOption {
    pub index: usize,
    pub image: Option<Handle<Image>>,
    pub text: String,
}

#[derive(Component)]
pub struct MainText;

#[derive(Component)]
pub struct Erasable;

/// Returns whether or not the node is simple.
fn show_current_node(
    graph: &BookGraph,
    first_page: Entity,
    second_page: Entity,
    commands: &mut Commands,
    fonts: &Res<FontAssets>,
    textures: &Res<UiTextures>,
) -> bool {
    let node = graph.get_current_node();
    match node {
        Node::Fork { content, choices } => {
            let content = (content)(&graph.context);
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((get_formatted_text(content, fonts), Erasable));
                parent.spawn((
                    ImageBundle {
                        image: textures.fancy_underline.clone().into(),
                        style: Style {
                            width: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    Erasable,
                ));
            });
            commands.entity(second_page).with_children(|parent| {
                let number_of_choices = choices.len();
                for (index, choice) in choices.iter().enumerate() {
                    let text = (choice.text)(&graph.context).to_string();
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::NONE.into(),
                                style: Style {
                                    width: Val::Percent(100.),
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(if index == 0 { 0. } else { 10. })),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                border_color: Color::BLACK.into(),
                                ..default()
                            },
                            ChoicesOption {
                                index,
                                image: choice.illustration.clone(),
                                text: text.clone(),
                            },
                            Erasable,
                        ))
                        .with_children(|parent| {
                            if let Some(ref illustration) = choice.illustration {
                                parent.spawn(ImageBundle {
                                    image: illustration.clone().into(),
                                    style: Style {
                                        height: Val::Px(150.),
                                        ..default()
                                    },
                                    ..default()
                                });
                            }
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: text,
                                        style: TextStyle {
                                            font: fonts.normal.clone(),
                                            font_size: if number_of_choices == 3 {
                                                25.
                                            } else {
                                                30.
                                            },
                                            color: Color::BLACK,
                                        },
                                    }],
                                    ..default()
                                },
                                style: Style { ..default() },
                                ..default()
                            });
                        });
                }
            });
            false
        }
        Node::Simple { content, extra, .. } => {
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    get_formatted_text((content)(&graph.context), fonts),
                    Erasable,
                ));
            });
            commands.entity(second_page).with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                width: Val::Percent(100.),
                                ..default()
                            },
                            ..default()
                        },
                        Erasable,
                    ))
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: textures.roses_frame.clone().into(),
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(0.),
                                left: Val::Px(0.),
                                margin: UiRect {
                                    top: Val::Px(-15.),
                                    left: Val::Px(-15.),
                                    ..default()
                                },
                                height: Val::Px(50.),
                                width: Val::Px(50.),
                                ..default()
                            },
                            ..default()
                        });
                        let mut flipped_roses_frame: UiImage = textures.roses_frame.clone().into();
                        flipped_roses_frame.flip_x = true;
                        flipped_roses_frame.flip_y = true;
                        parent.spawn(ImageBundle {
                            image: flipped_roses_frame,
                            style: Style {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(0.),
                                right: Val::Px(0.),
                                margin: UiRect {
                                    bottom: Val::Px(-15.),
                                    right: Val::Px(-15.),
                                    ..default()
                                },
                                height: Val::Px(50.),
                                width: Val::Px(50.),
                                ..default()
                            },
                            ..default()
                        });
                        parent.spawn((
                            TextBundle::from_section(
                                (extra.additional_text)(&graph.context),
                                TextStyle {
                                    font: fonts.normal.clone(),
                                    font_size: 30.,
                                    color: Color::BLACK,
                                },
                            ),
                            Erasable,
                        ));
                        // parent
                        // .spawn(NodeBundle {
                        //     style: Style {
                        //         position_type: PositionType::Relative,
                        //         top: Val::Px(0.),
                        //         left: Val::Px(0.),
                        //         margin: UiRect {
                        //             left: Val::Percent(-50.),
                        //             top: Val::Percent(-50.),
                        //             ..default()
                        //         },
                        //         ..default()
                        //     },
                        //     ..default()
                        // })
                        // .with_children(|parent| {
                        // });
                    });
                if let Some(ref illustration) = extra.illustration {
                    parent.spawn((
                        ImageBundle {
                            image: illustration.clone().into(),
                            style: Style {
                                width: Val::Percent(100.),
                                ..default()
                            },
                            ..default()
                        },
                        Erasable,
                    ));
                }
            });
            true
        }
    }
}

fn setup_graph(mut commands: Commands, illustrations: Res<Illustrations>) {
    let graph = book_content::get_book_content(&illustrations);
    // TODO: For testing, remove.
    // graph.set_current_node(1);
    commands.insert_resource(graph);
}

fn get_formatted_text(text: &str, fonts: &Res<FontAssets>) -> TextBundle {
    let mut chars_iter = text.chars();
    let first_letter = chars_iter.next().unwrap();
    let rest: String = chars_iter.collect();
    TextBundle::from_sections(vec![
        TextSection {
            value: first_letter.to_string(),
            style: TextStyle {
                font: fonts.first_letter.clone(),
                font_size: 100.,
                color: FIRST_LETTER_COLOR,
            },
        },
        TextSection {
            value: rest,
            style: TextStyle {
                font: fonts.normal.clone(),
                font_size: 30.,
                color: Color::BLACK,
            },
        },
    ])
}

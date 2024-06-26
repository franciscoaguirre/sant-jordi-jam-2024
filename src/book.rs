#![allow(clippy::too_many_arguments)]

use std::iter;

use bevy::{log, prelude::*};
use bevy_kira_audio::prelude::*;

use crate::{
    book_content::{self, BookGraph, TextStyles},
    graph::Node,
    loading::{AnimationAssets, AudioAssets, FontAssets, Illustrations, UiTextures},
    menu::{FirstPage, SecondPage},
    utils, GameState,
};

pub const BUTTON_HOVER_COLOR: Color = Color::rgba(1., 0., 0., 0.5);
pub const BUTTON_NORMAL_COLOR: Color = Color::NONE;
pub const BUTTON_PRESSED_COLOR: Color = Color::rgba(0.7, 0., 0., 0.7);

pub fn default_text_styles(fonts: &Res<FontAssets>, too_many_options: bool) -> TextStyles {
    let normal_font_size = if too_many_options { 25. } else { 30. };
    TextStyles {
        first_letter: TextStyle {
            color: Color::rgb(0.235, 0.039, 0.337),
            font: fonts.first_letter.clone(),
            font_size: 100.,
        },
        normal: TextStyle {
            color: Color::hex("3a1e0d").unwrap(),
            font: fonts.normal.clone(),
            font_size: normal_font_size,
        },
        highlighted: TextStyle {
            font: fonts.normal.clone(),
            font_size: normal_font_size,
            color: Color::rgb(0.678, 0.047, 0.109),
        },
    }
}

pub struct BookPlugin;
impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Transition>()
            .add_event::<AdvanceSimpleNode>()
            .add_event::<EraseEverything>()
            .add_event::<OptionChosen>()
            .add_event::<ShowArrow>()
            .add_event::<GameEnded>()
            .add_systems(OnEnter(GameState::Playing), (setup_graph, setup_lifecycle))
            .add_systems(
                Update,
                (
                    transition_listener,
                    show_current_node_and_transition,
                    interact_with_options,
                    erase_everything_listener,
                    draw_chosen_option,
                    show_arrow_system,
                    advance_simple_node_listener,
                    end_game_listener,
                    flip_page,
                    flip_page_listener,
                    interact_with_end_button,
                    interact_with_arrow,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Event, Default)]
pub struct GameEnded;

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
pub struct Transition;

#[derive(Event, Default)]
pub struct ShowArrow;

#[derive(Resource)]
pub struct LifecycleManager(pub Lifecycle);

#[derive(Resource)]
pub enum Lifecycle {
    ShowNode,
    Choosing,
    Chosen,
    Transitioning,
    SimpleNode,
    End,
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
            End => ShowNode,
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
        let Node::Fork {
            choices, content, ..
        } = current_node
        else {
            unreachable!("An option was chosen, it's a fork.");
        };
        // TODO: I could get everything from `current_node`.
        let OptionChosen { index, text, image } = event;
        let chosen_option = &choices[*index];
        let mut first_page = commands.entity(first_page.single());
        first_page.with_children(|parent| {
            parent.spawn((
                get_formatted_text(
                    text,
                    &content
                        .text_styles
                        .clone()
                        .unwrap_or(default_text_styles(&fonts, false)),
                ),
                Erasable,
            ));
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

fn end_game_listener(mut events: EventReader<GameEnded>, mut lifecycle: ResMut<LifecycleManager>) {
    for _ in events.read() {
        log::info!("Received GameEnded event");
        lifecycle.0 = Lifecycle::End;
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
        do_flip_page(
            &audio,
            &audio_assets,
            &mut players,
            &animations,
            &mut event_writer,
            &mut erase_everything,
        );
    }
}

fn do_flip_page(
    audio: &Res<Audio>,
    audio_assets: &Res<AudioAssets>,
    players: &mut Query<&mut AnimationPlayer>,
    animations: &Res<AnimationAssets>,
    event_writer: &mut EventWriter<Transition>,
    erase_everything: &mut EventWriter<EraseEverything>,
) {
    audio.play(audio_assets.page_flip.clone());
    for mut player in players.iter_mut() {
        player.start(animations.page_flip.clone());
        event_writer.send_default();
        erase_everything.send_default();
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
    graph: Res<BookGraph>,
    mut show_arrow: EventWriter<ShowArrow>,
) {
    if let Lifecycle::Transitioning = lifecycle.0 {
        for player in players.iter() {
            if player.is_finished() {
                event_writer.send_default();
                match graph.get_current_node() {
                    Node::Simple { next, .. } if next.is_some() => {
                        show_arrow.send_default();
                    }
                    _ => {}
                };
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
    mut game_ended: EventWriter<GameEnded>,
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
            &mut game_ended,
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
    mut show_arrow: EventWriter<ShowArrow>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
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
                    audio.play(audio_assets.scribble.clone()).with_volume(5.);
                    *background_color = BUTTON_PRESSED_COLOR.into();
                    option_chosen.send(OptionChosen {
                        index: choice.index,
                        text: choice.text.clone(),
                        image: choice.image.clone(),
                    });
                    erase_everything.send_default();
                    transition.send_default();
                    show_arrow.send_default();
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Arrow;

fn show_arrow_system(
    mut commands: Commands,
    textures: Res<UiTextures>,
    mut events: EventReader<ShowArrow>,
    second_page: Query<Entity, With<SecondPage>>,
) {
    for _ in events.read() {
        commands
            .entity(second_page.single())
            .with_children(|parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(0.),
                                right: Val::Px(0.),
                                height: Val::Px(80.),
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        },
                        Erasable,
                        Arrow,
                    ))
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: textures.arrow.clone().into(),
                            ..default()
                        });
                    });
            });
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

#[derive(Component)]
pub struct EndButton;

/// Returns whether or not the node is simple.
fn show_current_node(
    graph: &BookGraph,
    first_page: Entity,
    second_page: Entity,
    commands: &mut Commands,
    fonts: &Res<FontAssets>,
    textures: &Res<UiTextures>,
    game_ended: &mut EventWriter<GameEnded>,
) -> bool {
    let node = graph.get_current_node();
    match node {
        Node::Fork { content, choices } => {
            let text = (content.text)(&graph.context);
            let number_of_choices = choices.len();
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    get_formatted_text(
                        text,
                        &content
                            .text_styles
                            .clone()
                            .unwrap_or(default_text_styles(&fonts, false)),
                    ),
                    Erasable,
                ));
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
                                    position_type: PositionType::Relative,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(if index == 0 { 0. } else { 10. })),
                                    padding: UiRect::all(Val::Px(5.)),
                                    ..default()
                                },
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
                            parent.spawn((
                                ImageBundle {
                                    image: textures.choice_frame.clone().into(),
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        top: Val::Px(0.),
                                        bottom: Val::Px(0.),
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                Erasable,
                            ));
                            let image = if let Some(ref illustration) = choice.illustration {
                                Some(ImageBundle {
                                    image: illustration.clone().into(),
                                    style: Style {
                                        height: Val::Px(150.),
                                        ..default()
                                    },
                                    ..default()
                                })
                            } else {
                                None
                            };
                            let sections = utils::process_string_asterisks(&text)
                                .into_iter()
                                .enumerate()
                                .map(|(index, string)| TextSection {
                                    value: string,
                                    style: if index % 2 == 0 {
                                        default_text_styles(&fonts, number_of_choices == 3).normal
                                    } else {
                                        default_text_styles(&fonts, number_of_choices == 3)
                                            .highlighted
                                    },
                                });

                            if index % 2 == 0 {
                                if let Some(image) = image {
                                    parent.spawn(image);
                                }

                                parent.spawn(TextBundle::from_sections(sections));
                            } else {
                                parent.spawn(TextBundle::from_sections(sections));

                                if let Some(image) = image {
                                    parent.spawn(image);
                                }
                            }
                        });
                }
            });
            false
        }
        Node::Simple {
            content,
            extra,
            next,
        } => {
            if next.is_none() {
                log::info!("Sending GameEnded event");
                game_ended.send_default();
            }
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    get_formatted_text(
                        (content.text)(&graph.context),
                        &content
                            .text_styles
                            .clone()
                            .unwrap_or(default_text_styles(&fonts, false)),
                    ),
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
                    });
                if let Some(ref illustration) = extra.illustration {
                    parent.spawn((
                        ImageBundle {
                            image: illustration.clone().into(),
                            style: Style {
                                width: Val::Percent(90.),
                                ..default()
                            },
                            ..default()
                        },
                        Erasable,
                    ));
                }
                for decoration in extra.decorations.iter() {
                    parent.spawn((
                        ImageBundle {
                            image: decoration.clone().into(),
                            style: Style {
                                max_height: Val::Percent(30.),
                                margin: UiRect::top(Val::Px(20.)),
                                ..default()
                            },
                            ..default()
                        },
                        Erasable,
                    ));
                }
                if let None = next {
                    parent
                        .spawn((ButtonBundle::default(), EndButton, Erasable))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: textures.end_button.clone().into(),
                                ..default()
                            });
                        });
                }
            });
            true
        }
    }
}

fn interact_with_end_button(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<EndButton>),
    >,
    textures: Res<UiTextures>,
    mut graph: ResMut<BookGraph>,
    mut transition: EventWriter<Transition>,
    mut erase_everything: EventWriter<EraseEverything>,
) {
    for (interaction, mut image) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                image.texture = textures.end_button_hover.clone();
            }
            Interaction::None => {
                image.texture = textures.end_button.clone();
            }
            Interaction::Pressed => {
                log::info!("Pressed end button");
                graph.reset();
                transition.send_default();
                erase_everything.send_default();
            }
        }
    }
}

fn interact_with_arrow(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<Arrow>)>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut players: Query<&mut AnimationPlayer>,
    animations: Res<AnimationAssets>,
    mut event_writer: EventWriter<Transition>,
    mut erase_everything: EventWriter<EraseEverything>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                do_flip_page(
                    &audio,
                    &audio_assets,
                    &mut players,
                    &animations,
                    &mut event_writer,
                    &mut erase_everything,
                );
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn setup_graph(
    mut commands: Commands,
    illustrations: Res<Illustrations>,
    fonts: Res<FontAssets>,
    ui_textures: Res<UiTextures>,
) {
    let graph = book_content::get_book_content(&illustrations, &fonts, &ui_textures);
    // TODO: For testing, remove.
    // graph.set_current_node(18);
    commands.insert_resource(graph);
}

fn get_formatted_text(text: &str, text_styles: &TextStyles) -> TextBundle {
    let mut chars_iter = text.chars();
    let first_letter = chars_iter.next().unwrap();
    let first_letter_section = iter::once(TextSection {
        value: first_letter.to_string(),
        style: text_styles.first_letter.clone(),
    });
    let rest: String = chars_iter.collect();
    let rest_sections = utils::process_string_asterisks(&rest)
        .into_iter()
        .enumerate()
        .map(|(index, text)| TextSection {
            value: text,
            style: if index % 2 == 0 {
                text_styles.normal.clone()
            } else {
                text_styles.highlighted.clone()
            },
        });
    let sections: Vec<TextSection> = first_letter_section.chain(rest_sections).collect();
    TextBundle::from_sections(sections)
}

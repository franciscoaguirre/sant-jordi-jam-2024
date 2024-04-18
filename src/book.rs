use bevy::prelude::*;

use crate::{
    book_content::{self, BookGraph, BookNode},
    graph::{GetNextNode, Graph, Node},
    loading::{AnimationAssets, FontAssets, ModelAssets, TextureAssets},
    GameState,
};

const BUTTON_HOVER_COLOR: Color = Color::rgba(1., 0., 0., 0.5);
const BUTTON_NORMAL_COLOR: Color = Color::NONE;
const BUTTON_PRESSED_COLOR: Color = Color::rgba(0.7, 0., 0., 0.7);

pub struct BookPlugin;
impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Transition>()
            .add_event::<PageFlipStarted>()
            .add_event::<PageFlipEnded>()
            .add_systems(
                OnEnter(GameState::Playing),
                (setup_book, setup_graph, setup_lifecycle),
            )
            .add_systems(
                Update,
                (
                    transition_listener,
                    show_current_node_and_transition,
                    interact_with_options,
                    clear_book_content,
                    leave_only_chosen_option,
                    flip_page,
                    flip_page_listener,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Event, Default)]
pub struct PageFlipStarted;

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
}

impl Lifecycle {
    pub fn next(&self) -> Self {
        use Lifecycle::*;
        match self {
            ShowNode => Choosing,
            Choosing => Chosen,
            Chosen => Transitioning,
            Transitioning => ShowNode,
        }
    }
}

fn setup_lifecycle(mut commands: Commands) {
    commands.insert_resource(LifecycleManager(Lifecycle::ShowNode));
}

fn leave_only_chosen_option(
    mut commands: Commands,
    lifecycle: Res<LifecycleManager>,
    mut chosen_option_query: Query<(Entity, &mut BackgroundColor, &mut Style), With<ChosenOption>>,
    other_options_query: Query<Entity, (With<ChoicesOption>, Without<ChosenOption>)>,
    first_page: Query<Entity, With<FirstPage>>,
) {
    if let Lifecycle::Chosen = lifecycle.0 {
        for entity in other_options_query.iter() {
            commands.get_entity(entity).unwrap().despawn_recursive();
        }
        let mut first_page = commands.get_entity(first_page.single()).unwrap();
        let (chosen_option, mut background_color, mut style) = chosen_option_query.single_mut();
        *background_color = Color::NONE.into();
        style.justify_self = JustifySelf::End;
        style.align_self = AlignSelf::Center;
        first_page.add_child(chosen_option);
    }
}

fn clear_book_content(
    mut commands: Commands,
    erasable_query: Query<Entity, With<Erasable>>,
    mut events: EventReader<PageFlipStarted>,
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
    mut page_flip_started_writer: EventWriter<PageFlipStarted>,
) {
    if let Lifecycle::Chosen = lifecycle.0 {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for mut player in players.iter_mut() {
                player.start(animations.page_flip.clone());
                event_writer.send_default();
                page_flip_started_writer.send_default();
            }
        }
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
    mut event_writer: EventWriter<Transition>,
    first_page: Query<Entity, With<FirstPage>>,
    second_page: Query<Entity, With<SecondPage>>,
    mut commands: Commands,
    fonts: Res<FontAssets>,
) {
    if let Lifecycle::ShowNode = lifecycle.0 {
        let first_page = first_page.single();
        let second_page = second_page.single();
        let is_simple = show_current_node(
            graph.get_current_node(),
            first_page,
            second_page,
            &mut commands,
            &fonts,
        );
        if is_simple {
            lifecycle.0 = Lifecycle::Chosen;
        } else {
            event_writer.send(Transition);
        }
    }
}

fn interact_with_options(
    mut commands: Commands,
    lifecycle: Res<LifecycleManager>,
    mut interaction_query: Query<
        (&Interaction, &ChoicesOption, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<ChoicesOption>),
    >,
    mut graph: ResMut<BookGraph>,
    mut event_writer: EventWriter<Transition>,
) {
    if let Lifecycle::Choosing = lifecycle.0 {
        for (interaction, choice, mut background_color, entity) in interaction_query.iter_mut() {
            match *interaction {
                Interaction::Hovered => {
                    *background_color = BUTTON_HOVER_COLOR.into();
                }
                Interaction::None => {
                    *background_color = BUTTON_NORMAL_COLOR.into();
                }
                Interaction::Pressed => {
                    *background_color = BUTTON_PRESSED_COLOR.into();
                    graph.choose(choice.0);
                    event_writer.send(Transition);
                    commands.get_entity(entity).unwrap().insert(ChosenOption);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct ChoicesOption(pub usize);

#[derive(Component)]
pub struct ChosenOption;

#[derive(Component)]
pub struct Erasable;

/// Returns whether or not the node is simple.
fn show_current_node(
    node: &BookNode,
    first_page: Entity,
    second_page: Entity,
    commands: &mut Commands,
    fonts: &Res<FontAssets>,
) -> bool {
    match node {
        Node::Fork { content, choices } => {
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        *content,
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 50.,
                            color: Color::BLACK,
                        },
                    ),
                    Erasable,
                ));
            });
            commands.entity(second_page).with_children(|parent| {
                for (index, choice) in choices.iter().enumerate() {
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::NONE.into(),
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(if index == 0 { 0. } else { 10. })),
                                    ..default()
                                },
                                ..default()
                            },
                            ChoicesOption(index),
                            Erasable,
                        ))
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: choice.illustration.clone().into(),
                                style: Style {
                                    width: Val::Px(150.),
                                    ..default()
                                },
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                choice.text,
                                TextStyle {
                                    font: fonts.normal.clone(),
                                    font_size: 20.,
                                    color: Color::BLACK,
                                },
                            ));
                        });
                }
            });
            false
        }
        Node::Simple { content, .. } => {
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        *content,
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 50.,
                            color: Color::BLACK,
                        },
                    ),
                    Erasable,
                ));
            });
            // TODO: Missing illustration. Should it go on a NodeSimple generic?
            true
        }
    }
}

fn setup_graph(mut commands: Commands, textures: Res<TextureAssets>) {
    let graph = book_content::get_book_content(&textures);
    commands.insert_resource(graph);
}

#[derive(Component)]
pub struct FirstPage;

#[derive(Component)]
pub struct SecondPage;

fn setup_book(mut commands: Commands, models: Res<ModelAssets>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: models.book.clone(),
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        // border: UiRect::all(Val::Px(2.)),
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
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        margin: UiRect::left(Val::Px(40.0)),
                        // border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    // border_color: Color::RED.into(),
                    ..default()
                },
                SecondPage,
            ));
        });
}

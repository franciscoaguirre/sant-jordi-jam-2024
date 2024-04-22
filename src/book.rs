#![allow(clippy::too_many_arguments)]

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

use crate::{
    book_content::{self, BookGraph},
    graph::Node,
    loading::{AnimationAssets, FontAssets, Illustrations, UiTextures},
    menu::{FirstPage, SecondPage, TargetCameras},
    GameState,
};

pub const BUTTON_HOVER_COLOR: Color = Color::rgba(1., 0., 0., 0.5);
pub const BUTTON_NORMAL_COLOR: Color = Color::NONE;
pub const BUTTON_PRESSED_COLOR: Color = Color::rgba(0.7, 0., 0., 0.7);

pub const FIRST_LETTER_COLOR: Color = Color::rgba(
    0.23529411764705882,
    0.0392156862745098,
    0.33725490196078434,
    1.,
);

pub struct BookPlugin;
impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Transition>()
            .add_event::<AdvanceSimpleNode>()
            .add_event::<EraseEverything>()
            .add_event::<PageFlipEnded>()
            .add_event::<OptionChosen>()
            .insert_resource(SelectedOption { index: 0 })
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
                    highlight_selected_option,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Event)]
pub struct OptionChosen {
    index: usize,
    text: String,
    image: Handle<Image>,
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
    fonts: Res<FontAssets>,
    mut events: EventReader<OptionChosen>,
    mut graph: ResMut<BookGraph>,
    target_cameras: Res<TargetCameras>,
    mut images: ResMut<Assets<Image>>,
    textures: Res<UiTextures>,
) {
    for event in events.read() {
        let current_node = graph.get_current_node();
        let Node::Fork { choices, .. } = current_node else {
            unreachable!("An option was chosen, it's a fork.");
        };
        // TODO: I could get everything from `current_node`.
        let OptionChosen { index, text, image } = event;
        let chosen_option = &choices[*index];
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(700.),
                        height: Val::Percent(100.),
                        padding: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(target_cameras.first_page),
            ))
            .with_children(|parent| {
                parent.spawn(get_formatted_text(text, &fonts).with_style(Style {
                    width: Val::Px(300.),
                    ..default()
                }));
            });
        let size = Extent3d {
            width: 1024,
            height: 1024,
            ..default()
        };
        let mut temp_image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        temp_image.resize(size);
        let temp_image_handle = images.add(temp_image);
        let temp_image_camera = commands
            .spawn(Camera2dBundle {
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(temp_image_handle.clone()),
                    ..default()
                },
                ..default()
            })
            .id();
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::Flex,
                        padding: UiRect {
                            left: Val::Px(150.),
                            top: Val::Px(20.),
                            right: Val::Px(20.),
                            bottom: Val::Px(20.),
                        },
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(temp_image_camera),
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: textures.paper.clone().into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        left: Val::Px(0.),
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                });
                parent.spawn(
                    TextBundle::from_section(
                        (chosen_option.additional_text)(&graph.context),
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 30.,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        width: Val::Percent(50.),
                        ..default()
                    }),
                );
                parent.spawn(ImageBundle {
                    image: image.clone().into(),
                    style: Style {
                        height: Val::Percent(50.),
                        ..default()
                    },
                    ..default()
                });
            });
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(target_cameras.turning_page),
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture: temp_image_handle.clone(),
                        flip_x: true,
                        ..default()
                    },
                    ..default()
                });
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
) {
    if matches!(lifecycle.0, Lifecycle::Chosen | Lifecycle::SimpleNode)
        && keyboard_input.just_pressed(KeyCode::Space)
    {
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
    target_cameras: Res<TargetCameras>,
    mut images: ResMut<Assets<Image>>,
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
            &target_cameras,
            &mut images,
        );
        if is_simple {
            lifecycle.0 = Lifecycle::SimpleNode;
            advance_simple_node.send_default();
        } else {
            transition.send_default();
        }
    }
}

#[derive(Resource)]
pub struct SelectedOption {
    pub index: usize,
}

fn interact_with_options(
    lifecycle: Res<LifecycleManager>,
    choices_query: Query<&ChoicesOption>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_option: ResMut<SelectedOption>,
    mut option_chosen: EventWriter<OptionChosen>,
    mut transition: EventWriter<Transition>,
    mut erase_everything: EventWriter<EraseEverything>,
) {
    if let Lifecycle::Choosing = lifecycle.0 {
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            selected_option.index += 1;
        } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            selected_option.index -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            for choice in choices_query.iter() {
                if selected_option.index == choice.index {
                    option_chosen.send(OptionChosen {
                        index: selected_option.index,
                        image: choice.image.clone().expect("Handle no textures"),
                        text: choice.text.clone(),
                    });
                    erase_everything.send_default();
                    transition.send_default();
                }
            }
        }
    }
}

fn highlight_selected_option(
    selected_option: Res<SelectedOption>,
    mut choices_query: Query<(&ChoicesOption, &mut BackgroundColor)>,
) {
    for (choice, mut background_color) in choices_query.iter_mut() {
        if selected_option.index == choice.index {
            *background_color = Color::RED.into();
        } else {
            *background_color = Color::NONE.into();
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
    target_cameras: &Res<TargetCameras>,
    images: &mut ResMut<Assets<Image>>,
) -> bool {
    let node = graph.get_current_node();
    match node {
        Node::Fork { content, choices } => {
            let content = (content)(&graph.context);
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(700.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            padding: UiRect::all(Val::Px(20.)),
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        ..default()
                    },
                    TargetCamera(target_cameras.first_page),
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: textures.paper.clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.),
                            left: Val::Px(0.),
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn((
                        get_formatted_text(content, &fonts).with_style(Style {
                            width: Val::Px(300.),
                            ..default()
                        }),
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
            let size = Extent3d {
                width: 1024,
                height: 1024,
                ..default()
            };
            let mut temp_image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            };
            temp_image.resize(size);
            let temp_image_handle = images.add(temp_image);
            let temp_image_camera = commands
                .spawn(Camera2dBundle {
                    camera: Camera {
                        order: -1,
                        target: RenderTarget::Image(temp_image_handle.clone()),
                        ..default()
                    },
                    ..default()
                })
                .id();
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::Flex,
                            padding: UiRect {
                                left: Val::Px(150.),
                                top: Val::Px(20.),
                                right: Val::Px(20.),
                                bottom: Val::Px(20.),
                            },
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        background_color: Color::PINK.into(),
                        ..default()
                    },
                    TargetCamera(temp_image_camera),
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: textures.paper.clone().into(),
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.),
                            left: Val::Px(0.),
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    });
                    let number_of_choices = choices.len();
                    for (index, choice) in choices.iter().enumerate() {
                        let text = (choice.text)(&graph.context).to_string();
                        parent
                            .spawn((
                                NodeBundle {
                                    background_color: Color::NONE.into(),
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(35.),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::top(Val::Px(if index == 0 {
                                            0.
                                        } else {
                                            50.
                                        })),
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
                                            height: Val::Px(250.),
                                            width: Val::Px(150.),
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
                                                    20.
                                                } else {
                                                    25.
                                                },
                                                color: Color::BLACK,
                                            },
                                        }],
                                        ..default()
                                    },
                                    style: Style {
                                        width: Val::Px(350.),
                                        margin: UiRect::top(Val::Px(50.)),
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    }
                });
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        ..default()
                    },
                    TargetCamera(target_cameras.turning_page),
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            texture: temp_image_handle.clone(),
                            flip_x: true,
                            ..default()
                        },
                        ..default()
                    });
                });
            false
        }
        Node::Simple { content, extra, .. } => {
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    get_formatted_text((content)(&graph.context), &fonts),
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
    let mut graph = book_content::get_book_content(&illustrations);
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

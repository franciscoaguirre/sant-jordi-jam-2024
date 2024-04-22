use crate::book::{BUTTON_HOVER_COLOR, BUTTON_NORMAL_COLOR};
use crate::loading::{FontAssets, MaterialAssets, ModelAssets, UiTextures};
use crate::GameState;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, Face, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};

pub const MENU_BUTTON_RED: Color = Color::rgba(
    0.6784313725490196,
    0.047058823529411764,
    0.10588235294117647,
    1.,
);

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Menu),
            (setup_book, setup_menu.after(setup_book)),
        )
        .add_systems(
            Update,
            (
                change_test_text,
                interact_with_test_button,
                click_play_button,
            )
                .run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct Menu;

#[derive(Component)]
pub struct FirstPage;

#[derive(Component)]
pub struct SecondPage;

#[derive(Component)]
pub struct TestButton;

#[derive(Component)]
pub struct TestText;

#[derive(Resource)]
pub struct TargetCameras {
    pub first_page: Entity,
    pub second_page: Entity,
    pub turning_page: Entity,
}

fn change_test_text(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<&mut Text, With<TestText>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = "Vamo arriba!".to_string();
        }
    }
}

fn interact_with_test_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<TestButton>)>,
    mut text_query: Query<&mut Text, With<TestText>>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                let mut text = text_query.single_mut();
                text.sections[0].value = "Vamooo!".to_string();
            }
            _ => {}
        }
    }
}

fn setup_book(
    mut commands: Commands,
    models: Res<ModelAssets>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials_collection: Res<MaterialAssets>,
) {
    // Image holding the page texture.
    let size = Extent3d {
        width: 1024,
        height: 1024,
        ..default()
    };
    let mut first_page_texture = Image {
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
    // Fill image with zeros.
    first_page_texture.resize(size);
    let first_page_texture_handle = images.add(first_page_texture);
    // Pre-pass camera.
    let first_page_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(first_page_texture_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    let mut second_page_texture = Image {
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
    second_page_texture.resize(size);
    let second_page_texture_handle = images.add(second_page_texture);
    let second_page_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                order: -1, // TODO: Do I need to change this?
                target: RenderTarget::Image(second_page_texture_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    let mut turning_page_texture = Image {
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
    turning_page_texture.resize(size);
    let turning_page_texture_handle = images.add(turning_page_texture);
    let turning_page_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                order: -1, // TODO: Do I need to change this?
                target: RenderTarget::Image(turning_page_texture_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    commands.insert_resource(TargetCameras {
        first_page: first_page_camera,
        second_page: second_page_camera,
        turning_page: turning_page_camera,
    });

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
    // Render to image.
    // commands
    //     .spawn((
    //         NodeBundle {
    //             style: Style {
    //                 width: Val::Px(700.),
    //                 height: Val::Percent(100.),
    //                 display: Display::Flex,
    //                 flex_direction: FlexDirection::Column,
    //                 justify_content: JustifyContent::Center,
    //                 align_items: AlignItems::Center,
    //                 ..default()
    //             },
    //             background_color: Color::PINK.into(),
    //             ..default()
    //         },
    //         TargetCamera(first_page_camera),
    //     ))
    //     .with_children(|parent| {
    //         parent
    //             .spawn((ButtonBundle::default(), TestButton))
    //             .with_children(|parent| {
    //                 parent.spawn((
    //                     TextBundle::from_section(
    //                         "Funciona!",
    //                         TextStyle {
    //                             font_size: 50.,
    //                             ..default()
    //                         },
    //                     ),
    //                     TestText,
    //                 ));
    //             });
    //     });
    // Modify pages material with new texture.
    let first_page_material = materials
        .get_mut(materials_collection.left_page.clone())
        .unwrap();
    first_page_material.base_color_texture = Some(first_page_texture_handle);
    let second_page_material = materials
        .get_mut(materials_collection.right_page.clone())
        .unwrap();
    second_page_material.base_color_texture = Some(second_page_texture_handle);
    let turning_page_material = materials
        .get_mut(materials_collection.turning_page.clone())
        .unwrap();
    turning_page_material.base_color_texture = Some(turning_page_texture_handle);
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            border_color: Color::GREEN.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Vw(80.0),
                        height: Val::Vh(80.0),
                        padding: UiRect {
                            left: Val::Percent(8.0),
                            right: Val::Percent(8.0),
                            ..default()
                        },
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    border_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|children| {
                    // First page.
                    children.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Vw(30.0),
                                height: Val::Vh(80.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            border_color: Color::RED.into(),
                            ..default()
                        },
                        FirstPage,
                    ));

                    // Second page.
                    children.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Vw(30.0),
                                height: Val::Vh(80.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceAround,
                                border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            border_color: Color::RED.into(),
                            ..default()
                        },
                        SecondPage,
                    ));
                });
        });
}

fn setup_menu(
    mut commands: Commands,
    second_page: Query<Entity, With<SecondPage>>,
    textures: Res<UiTextures>,
    fonts: Res<FontAssets>,
    target_cameras: Res<TargetCameras>,
) {
    let mut second_page = commands.entity(second_page.single());
    second_page.with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                Menu,
                // TargetCamera(target_cameras.second_page),
            ))
            .with_children(|parent| {
                // Play button.
                parent
                    .spawn(ButtonBundle::default())
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: textures.play_button.clone().into(),
                            style: Style {
                                height: Val::Px(40.),
                                ..default()
                            },
                            ..default()
                        });
                    });
                // Language buttons.
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::SpaceAround,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(ButtonBundle {
                                background_color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Català",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size: 30.,
                                        color: MENU_BUTTON_RED,
                                    },
                                ));
                            });
                        parent
                            .spawn(ButtonBundle {
                                background_color: Color::NONE.into(),
                                style: Style {
                                    margin: UiRect::left(Val::Px(20.)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Castellano",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size: 30.,
                                        color: MENU_BUTTON_RED,
                                    },
                                ));
                            });
                    });
                // Controls.
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::SpaceAround,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Mouse.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: textures.mouse.clone().into(),
                                    style: Style {
                                        height: Val::Px(40.),
                                        ..default()
                                    },
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Mouse",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size: 20.,
                                        color: Color::BLACK,
                                    },
                                ));
                            });
                        // Spacebar.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::left(Val::Px(30.)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: textures.keyboard.clone().into(),
                                    style: Style {
                                        height: Val::Px(40.),
                                        ..default()
                                    },
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "[Space bar]",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size: 20.,
                                        color: Color::BLACK,
                                    },
                                ));
                            });
                    });
                // Credits.
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::SpaceAround,
                            height: Val::Percent(30.),
                            width: Val::Percent(100.),
                            border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        border_color: Color::RED.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        let font_size = 15.;
                        let color = Color::rgba(105. / 256., 82. / 256., 46. / 256., 1.);
                        // Pablo.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    height: Val::Percent(100.),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: textures.warrior_bunny.clone().into(),
                                    style: Style {
                                        height: Val::Percent(60.),
                                        ..default()
                                    },
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Pablo Ferrer",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size,
                                        color,
                                    },
                                ));
                            });
                        // Alex.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: textures.snail_boy.clone().into(),
                                    style: Style {
                                        height: Val::Percent(60.),
                                        ..default()
                                    },
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Álex Pérez",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size,
                                        color,
                                    },
                                ));
                            });
                        // Fran.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: textures.rabbit_troubadour.clone().into(),
                                    style: Style {
                                        height: Val::Percent(60.),
                                        ..default()
                                    },
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Francisco Aguirre",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size,
                                        color,
                                    },
                                ));
                            });
                        // Clau.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    height: Val::Percent(100.),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(ImageBundle {
                                    image: textures.cat.clone().into(),
                                    style: Style {
                                        height: Val::Percent(60.),
                                        ..default()
                                    },
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Claudia Mohedano",
                                    TextStyle {
                                        font: fonts.normal.clone(),
                                        font_size,
                                        color,
                                    },
                                ));
                            });
                    });
            });
    });
}

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

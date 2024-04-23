use crate::book::{BUTTON_HOVER_COLOR, BUTTON_NORMAL_COLOR};
use crate::loading::{AudioAssets, FontAssets, ModelAssets, UiTextures};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub const MENU_BUTTON_RED: Color = Color::rgb(0.678, 0.047, 0.109);

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Language::Catalan)
            .add_systems(
                OnEnter(GameState::Menu),
                (
                    start_background_music,
                    setup_book,
                    setup_menu.after(setup_book),
                ),
            )
            .add_systems(
                Update,
                (interact_with_language_buttons, click_play_button)
                    .run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn start_background_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.background_music.clone()).looped();
}

#[derive(Resource, Clone)]
pub enum Language {
    Catalan,
    Spanish,
}

#[derive(Component)]
pub struct PickLanguage(pub Language);

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
struct Menu;

#[derive(Component)]
pub struct FirstPage;

#[derive(Component)]
pub struct SecondPage;

fn interact_with_language_buttons(
    mut interaction_query: Query<(&Interaction, &PickLanguage), Changed<Interaction>>,
    mut language: ResMut<Language>,
) {
    for (interaction, pick_language) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *language = pick_language.0.clone();
            }
            Interaction::None => {}
            _ => {}
        }
    }
}

fn setup_book(mut commands: Commands, models: Res<ModelAssets>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-0.1, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
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
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                // border: UiRect::all(Val::Px(2.)),
                ..default()
            },
            // border_color: Color::GREEN.into(),
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
                        // border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    // border_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|children| {
                    // First page.
                    children.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Vw(30.0),
                                height: Val::Vh(80.0),
                                padding: UiRect::all(Val::Px(30.0)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
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
                                width: Val::Vw(30.0),
                                height: Val::Vh(80.0),
                                padding: UiRect::all(Val::Px(30.0)),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceAround,
                                // border: UiRect::all(Val::Px(2.)),
                                ..default()
                            },
                            // border_color: Color::RED.into(),
                            ..default()
                        },
                        SecondPage,
                    ));
                });
        });
}

fn setup_menu(
    mut commands: Commands,
    first_page: Query<Entity, With<FirstPage>>,
    second_page: Query<Entity, With<SecondPage>>,
    textures: Res<UiTextures>,
    fonts: Res<FontAssets>,
) {
    let mut first_page = commands.entity(first_page.single());
    first_page.with_children(|parent| {
        parent.spawn((
            ImageBundle {
                image: textures.cover.clone().into(),
                style: Style {
                    max_width: Val::Percent(100.),
                    max_height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            Menu,
        ));
    });
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
            ))
            .with_children(|parent| {
                // Play button.
                parent
                    .spawn((ButtonBundle::default(), PlayButton))
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: textures.play_button.clone().into(),
                            style: Style {
                                height: Val::Px(100.),
                                ..default()
                            },
                            ..default()
                        });
                    });
                // Language buttons.
                // parent
                //     .spawn(NodeBundle {
                //         style: Style {
                //             display: Display::Flex,
                //             justify_content: JustifyContent::SpaceAround,
                //             ..default()
                //         },
                //         ..default()
                //     })
                //     .with_children(|parent| {
                //         parent
                //             .spawn((
                //                 ButtonBundle {
                //                     background_color: Color::NONE.into(),
                //                     border_color: MENU_BUTTON_RED.into(),
                //                     ..default()
                //                 },
                //                 PickLanguage(Language::Catalan),
                //             ))
                //             .with_children(|parent| {
                //                 parent.spawn(TextBundle::from_section(
                //                     "Català",
                //                     TextStyle {
                //                         font: fonts.normal.clone(),
                //                         font_size: 30.,
                //                         color: MENU_BUTTON_RED,
                //                     },
                //                 ));
                //             });
                //         parent
                //             .spawn((
                //                 ButtonBundle {
                //                     background_color: Color::NONE.into(),
                //                     border_color: MENU_BUTTON_RED.into(),
                //                     style: Style {
                //                         margin: UiRect::left(Val::Px(20.)),
                //                         ..default()
                //                     },
                //                     ..default()
                //                 },
                //                 PickLanguage(Language::Spanish),
                //             ))
                //             .with_children(|parent| {
                //                 parent.spawn(TextBundle::from_section(
                //                     "Castellano",
                //                     TextStyle {
                //                         font: fonts.normal.clone(),
                //                         font_size: 30.,
                //                         color: MENU_BUTTON_RED,
                //                     },
                //                 ));
                //             });
                //     });
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
                            // border: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        // border_color: Color::RED.into(),
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
        (Changed<Interaction>, With<PlayButton>),
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

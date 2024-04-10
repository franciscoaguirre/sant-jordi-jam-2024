use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
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

fn setup_book(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 9.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/book.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children.spawn((
                        TextBundle::from_section(
                            "Hola",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        MainText,
                    ));
                });

            // Second page.
            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    children.spawn(TextBundle::from_section(
                        "Adios",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
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

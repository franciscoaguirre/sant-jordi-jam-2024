use bevy::prelude::*;

use crate::{
    graph::{GetNextNode, Graph, Node},
    loading::{FontAssets, ModelAssets, TextureAssets},
    GameState,
};

pub struct BookPlugin;
impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Transition>()
            .add_systems(
                OnEnter(GameState::Playing),
                (setup_book, setup_graph, setup_lifecycle),
            )
            .add_systems(
                Update,
                (transition_listener, show_current_node_and_transition)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Event)]
pub struct Transition;

#[derive(Resource)]
pub struct LifecycleManager(pub Lifecycle);

#[derive(Resource)]
pub enum Lifecycle {
    ShowNode,
    Choosing,
    Transitioning,
}

impl Lifecycle {
    pub fn next(&self) -> Self {
        use Lifecycle::*;
        match self {
            ShowNode => Choosing,
            Choosing => Transitioning,
            Transitioning => ShowNode,
        }
    }
}

fn setup_lifecycle(mut commands: Commands) {
    commands.insert_resource(LifecycleManager(Lifecycle::ShowNode));
}

type BookGraph = Graph<&'static str, NodeChoice>;
type BookNode = Node<&'static str, NodeChoice>;

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
    lifecycle: Res<LifecycleManager>,
    mut event_writer: EventWriter<Transition>,
    first_page: Query<Entity, With<FirstPage>>,
    second_page: Query<Entity, With<SecondPage>>,
    mut commands: Commands,
    fonts: Res<FontAssets>,
) {
    if let Lifecycle::ShowNode = lifecycle.0 {
        let first_page = first_page.single();
        let second_page = second_page.single();
        show_current_node(
            graph.get_current_node(),
            first_page,
            second_page,
            &mut commands,
            &fonts,
        );
        event_writer.send(Transition);
    }
}

#[derive(Component)]
pub struct ChoicesOption;

#[derive(Component)]
pub struct Erasable;

fn show_current_node(
    node: &BookNode,
    first_page: Entity,
    second_page: Entity,
    commands: &mut Commands,
    fonts: &Res<FontAssets>,
) {
    match node {
        Node::Fork { content, choices } => {
            commands.entity(first_page).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        *content,
                        TextStyle {
                            font: fonts.normal.clone(),
                            font_size: 100.,
                            color: Color::BLACK,
                        },
                    ),
                    Erasable,
                ));
            });
            commands.entity(second_page).with_children(|parent| {
                for choice in choices.iter() {
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: Color::NONE.into(),
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ChoicesOption,
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
                                    font_size: 50.,
                                    color: Color::BLACK,
                                },
                            ));
                        });
                }
            });
        }
        _ => unimplemented!("No simple nodes here"),
    }
}

struct NodeChoice {
    pub text: &'static str,
    pub illustration: Handle<Image>,
    pub next: usize,
}

impl GetNextNode for NodeChoice {
    fn next_node(&self) -> usize {
        self.next
    }
}

fn setup_graph(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut graph = Graph::new();
    graph.add_node(
        0,
        Node::Fork {
            content: "Hello",
            choices: vec![
                NodeChoice {
                    text: "un dragón",
                    illustration: textures.normal_dragon.clone(),
                    next: 1,
                },
                NodeChoice {
                    text: "un tipo disfrazado de dragón",
                    illustration: textures.sant_jordi_disguised_as_dragon.clone(),
                    next: 2,
                },
            ],
        },
    );
    graph.add_node(
        1,
        Node::Simple {
            content: "Fin",
            next: None,
        },
    );
    graph.add_node(
        2,
        Node::Simple {
            content: "Fin",
            next: None,
        },
    );
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

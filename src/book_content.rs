use bevy::prelude::*;

use crate::{
    graph::{GetNextNode, Graph, Node},
    loading::TextureAssets,
};

pub type BookGraph = Graph<&'static str, SimpleExtra, NodeChoice>;
pub type BookNode = Node<&'static str, SimpleExtra, NodeChoice>;

pub struct SimpleExtra {
    pub illustration: Handle<Image>,
}

pub struct NodeChoice {
    pub text: &'static str,
    pub illustration: Handle<Image>,
    pub next: usize,
}

impl GetNextNode for NodeChoice {
    fn next_node(&self) -> usize {
        self.next
    }
}

pub fn get_book_content(textures: &Res<TextureAssets>) -> BookGraph {
    let mut graph = Graph::new();
    graph.add_node(
        0,
        Node::Fork {
            content: "Érase una vez...",
            choices: vec![
                NodeChoice {
                    text: "...un dragón, bastante normal, probablemente con problemas de autoestima, que atemorizaba la villa de Montblancun",
                    illustration: textures.normal_dragon.clone(),
                    next: 1,
                },
                NodeChoice {
                    text: "...un humano con un disfraz de dragón cutre, que atemorizaba la villa de Montblanc",
                    illustration: textures.sant_jordi_disguised_as_dragon.clone(),
                    next: 2,
                },
            ],
        },
    );
    graph.add_node(
        1,
        Node::Simple {
            content: "Para tenerlo contento y alejado de la villa, los vecinos ofrecieron animales",
            extra: SimpleExtra {
                illustration: textures.dragon_with_cow.clone(),
            },
            next: Some(3),
        },
    );
    graph.add_node(
        2,
        Node::Simple {
            content: "Para tenerlo contento y alejado de la villa, los vecinos ofrecieron animales",
            extra: SimpleExtra {
                illustration: textures.jordi_dragon_with_cow.clone(),
            },
            next: Some(4),
        },
    );
    graph.add_node(
        3,
        Node::Fork {
            content: "Pero no fue suficiente para alejarlo, por lo que tomaron otras medidas.",
            choices: vec![
                NodeChoice {
                    text: "La princesa Cleodolinda, cansada de los inútiles intentos de la gente de la villa por calmar la situación, se ofreció voluntaria para matar al dragón",
                    illustration: textures.princess_go_kill_dragon.clone(),
                    next: 100
                },
                NodeChoice {
                    text: "La princesa Cleodolinda, deseosa por conocer a un dragón de verdad, se ofreció voluntaria y utilizar sus extensos conicimientos de dragones para solventar la situación",
                    illustration: textures.princess_excited_to_be_picked.clone(),
                    next: 100
                },
                NodeChoice {
                    text: "Para sorpresa de todos, el propio Rey fue elegido en el sorteo. Preso de su propia cobardía, les dijo a todos que era la Princesa quien había salido.",
                    illustration: textures.king_picks_princess.clone(),
                    next: 100
                }
            ],
        },
    );
    graph.add_node(
        4,
        Node::Fork {
            content: "Pero no fue suficiente para alejarlo, por lo que tomaron otras medidas.",
            choices: vec![
                NodeChoice {
                    text: "La princesa Cleodolinda, cansada de los inútiles intentos de la gente de la villa por calmar la situación, se ofreció voluntaria para matar al dragón",
                    illustration: textures.princess_go_kill_dragon.clone(),
                    next: 4
                },
                NodeChoice {
                    text: "La princesa Cleodolinda, deseosa por conocer a un dragón de verdad, se ofreció voluntaria y utilizar sus extensos conicimientos de dragones para solventar la situación",
                    illustration: textures.princess_excited_to_be_picked.clone(),
                    next: 4
                },
                NodeChoice {
                    text: "Para sorpresa de todos, el propio Rey fue elegido en el sorteo. Preso de su propia cobardía, les dijo a todos que era la Princesa quien había salido.",
                    illustration: textures.king_picks_princess.clone(),
                    next: 4
                }
            ],
        },
    );
    graph.add_node(
        5,
        Node::Simple {
            content: "Cleodolinda salió de las murallas y se dirigió a su destino. Donde se encontró... Al dragón sentado junto a una hoguera asando malvaviscos. Actividad algo extraña para un dragón...",
            extra: SimpleExtra {
                illustration: textures.sant_jordi_making_marshmallows.clone(),
            },
            next: Some(5)
        }
    );
    graph.add_node(
        6,
        Node::Fork {
            content:
                "Cleodolinda salió de las murallas y se dirigió a su destino. Donde se encontró...",
            choices: vec![
                NodeChoice {
                    text: "A un Sant Jordi muy flipado preparándose para la batalla con el dragón",
                    illustration: textures.sant_jordi_warrior.clone(),
                    next: 6,
                },
                NodeChoice {
                    text: "A Sant Jordi, sonrojado y sosteniendo un ramo de rosas.",
                    illustration: textures.sant_jordi_roses.clone(),
                    next: 6,
                },
            ],
        },
    );
    // graph.add_node(
    //     7,
    //     Node::Simple { content: "Pero Cleodolinda, decidida a llevar a cabo su tarea, se partió la crisma con el dragón.", next: None }
    // );
    // graph.add_node(
    //     8,
    //     Node::Simple {
    //         content: "Cleodolinda salió de las murallas y se dirigió a su destino. Donde se encontró... Al dragón sentado junto a una hoguera asando malvaviscos. Actividad algo extraña para un dragón...",
    //         next: Some(9)
    //     }
    // );
    // graph.add_node(
    //     9,
    //     Node::Simple { content: "Pero era difícil engañar a la fan número uno de los dragones. Ese dragón era claramente un humano.", next: None }
    // );
    graph
}

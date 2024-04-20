use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    graph::{ChoiceTrait, Graph, Node},
    loading::TextureAssets,
};

pub type BookGraph = Graph<&'static str, SimpleExtra, NodeChoice, BookContext>;
pub type BookNode = Node<&'static str, SimpleExtra, NodeChoice>;

pub struct SimpleExtra {
    pub illustration: Handle<Image>,
    pub additional_text: &'static str,
}

#[derive(Clone)]
pub struct NodeChoice {
    pub text: &'static str,
    pub illustration: Handle<Image>,
    pub additional_text: Arc<dyn Fn(&BookContext) -> &'static str + Sync + Send>,
    pub state_change: Arc<dyn Fn(&mut BookContext) + Sync + Send>,
    pub next: Arc<dyn Fn(&BookContext) -> usize + Sync + Send>,
}

impl ChoiceTrait<BookContext> for NodeChoice {
    fn next_node(&self, context: &BookContext) -> usize {
        (self.next)(context)
    }

    fn change_state(&self, context: &mut BookContext) {
        (self.state_change)(context);
    }
}

#[derive(Default, Clone)]
pub struct BookContext {
    santjordi_disfrazado: bool,
    dragon_normal: bool,
    princesa_guerrera: bool,
    fan_dragones: bool,
    princesa_rechazada: bool,
    encuentra_santjordi_disfrazado: bool,
    encuentra_santjordi_flipado: bool,
    encuentra_santjordi_enamorado: bool,
    entrar_cueva: bool,
    salir_cueva: bool,
    chivarse: bool,
    guardar_secreto: bool,
    dejarse_llevar: bool,
    rechazo: bool,
}

pub fn get_book_content(textures: &Res<TextureAssets>) -> BookGraph {
    let mut graph = Graph::new();
    graph.add_node(
        0,
        Node::Fork {
            content: "Érase una vez, un terrible dragón que atemorizaba la villa de Montblanc...",
            choices: vec![
                NodeChoice {
                    text: "Érase una vez, un hombre claramente disfrazado de dragón que, por algún motivo, atemorizaba la villa de Montblanc...",
                    illustration: textures.sant_jordi_disguised_as_dragon.clone(),
                    additional_text: Arc::new(|_| "Quizás fuera que se veían pocos dragones o que realmente tenían ganas de poder decir que habían visto uno, pero sea como fuere, la villa entera parecía convencida de ello."),
                    state_change: Arc::new(|context| context.santjordi_disfrazado = true),
                    next: Arc::new(|_| 1),
                },
                NodeChoice {
                    text: "Érase una vez, un dragón normalito, con sus problemas y sus cosas, cuya presencia atemorizaba la villa de Montblanc...",
                    illustration: textures.normal_dragon.clone(),
                    additional_text: Arc::new(|_| "Realmente no les hacía nada, pero un dragón gigante escupefuego era el objetivo perfecto sobre el que proyectar sus problemas."),
                    state_change: Arc::new(|context| context.dragon_normal = true),
                    next: Arc::new(|_| 2),
                },
            ],
        },
    );
    graph.add_node(
        1,
        Node::Simple {
            content: "Todavía inmersos en sus delirios, los habitantes de la villa empezaron a mandarle reses y animales, esperando que eso saciara su \"ira\". Sin embargo, no tuvo mucho efecto...",
            extra: SimpleExtra {
                illustration: textures.jordi_dragon_with_cow.clone(),
                additional_text: "De hecho, algunas reses eran casi tan grandes como el \"dragón\"...",
            },
            next: Some(2),
        },
    );
    graph.add_node(
        2,
        Node::Fork {
            content: "Con la villa desesperada, el rey no tuvo más alternativa que hacer un sorteo para ofrecerle a la bestia sacrificios humanos, ignorando que el destino, confuso y sibilino, se conjuraría en su contra con el sacrificio de su propia hija...",
            choices: vec![
                NodeChoice {
                    text: "La princesa Cleodolinda, cansada de los inútiles intentos de la gente de la villa por calmar la situación, se ofreció voluntaria para matar al dragón",
                    illustration: textures.princess_go_kill_dragon.clone(),
                    additional_text: Arc::new(|_| "Espada en mano y paso decididio, se dirigió a la cueva donde se escondía el dragón."),
                    state_change: Arc::new(|context| context.princesa_guerrera = true),
                    next: Arc::new(|_| 3),
                },
                NodeChoice {
                    text: "La princesa Cleodolinda, deseosa por conocer a un dragón de verdad, se ofreció voluntaria para utilizar sus extensos conocimientos de dragones para solventar la situación",
                    illustration: textures.princess_excited_to_be_picked.clone(),
                    additional_text: Arc::new(|_| "Con su enciclopedia favorita de dragones bajo el brazo, se dirigió a la cueva de la bestia sin ningún temor."),
                    state_change: Arc::new(|context| context.fan_dragones = true),
                    next: Arc::new(|_| 3),
                },
                NodeChoice {
                    text: "...pero cuando vió que él mismo fue el escogido en el sorteo, preso de su propia cobardía, les dijo a todos que la princesa Cleodolinda, su propia hija, fue la desaventurada víctima de la fortuna...",
                    illustration: textures.king_picks_princess.clone(),
                    additional_text: Arc::new(|_| "No fue una gran sorpresa para Cleodolinda, pero aún asi aceptó su destino y se encaminó hacia la guarida del dragón."),
                    state_change: Arc::new(|context| context.princesa_rechazada = true),
                    next: Arc::new(|_| 3),
                },
            ],
        },
    );
    graph.add_node(
        3,
        Node::Fork {
            content: "Cleodolinda aceptó su destino valientemente y tras un corto viaje llegó a la guarida del dragón, donde se encontró...",
            choices: vec![
                NodeChoice {
                    text: "Con un misterioso hombre disfrazado de dragón asando malvaviscos.",
                    illustration: textures.sant_jordi_making_marshmallows.clone(),
                    additional_text: Arc::new(|context| "Es bien sabido que no hay nada que un dragón disfrute más que sentarse como un humano a asar dulces delante del fuego..."),
                    state_change: Arc::new(|context| context.encuentra_santjordi_disfrazado = true),
                    next: Arc::new(|context| if context.princesa_guerrera { 4 } else if context.fan_dragones { 8 } else if context.princesa_rechazada { 10 } else { unreachable!("Some flag must be true") }),
                },
                NodeChoice {
                    text: "Con un apuesto caballero haciéndose pinturas tribales de guerra, alentándose a sí mismo...",
                    illustration: textures.sant_jordi_warrior.clone(),
                    additional_text: Arc::new(|_| "Su libro favorito era El arte de la guerra y especulaba con terrenos"),
                    state_change: Arc::new(|context| context.encuentra_santjordi_flipado = true),
                    next: Arc::new(|_| 12),
                },
                NodeChoice {
                    text: "Con un apuesto caballero frente a la guarida de la temible bestia sosteniendo ferozmente un... ¿ramo de flores?",
                    illustration: textures.sant_jordi_roses.clone(),
                    additional_text: Arc::new(|context| "No todos los caballeros tienen que ser agresivos, seguramente Sant Jordi tendría sus \"métodos\"..."),
                    state_change: Arc::new(|context| context.encuentra_santjordi_enamorado = true),
                    next: Arc::new(|context| 19),
                }
            ],
        },
    );
    graph.add_node(
        4,
        Node::Simple {
            content: "Algo decepcionada ante el inocente y algo adorable dragonzuelo que se encontraba ante ella, Cleodolinda no se amilanó y le atizó a la bestia tremendo capón en la sesera...",
            extra: SimpleExtra {
                illustration: textures.princess_punches_jordi_dragon.clone(),
                additional_text: "Estaba claro que la princesa no había hecho todo este viaje para quedarse ahora de brazos cruzados.",
            },
            next: Some(5),
        },
    );
    graph.add_node(
        5,
        Node::Simple {
            content: "Tal fue la contundencia del mamporrazo que la cabeza del \"dragón\" salió volando, revelando al hombre que se había estado haciendo pasar por la bestia todo este tiempo: ¡Sant Jordi!",
            extra: SimpleExtra {
                illustration: textures.jordi_dragon_confesses.clone(),
                additional_text: "La princesa, iracunda, exigió explicaciones a Sant Jordi, indignada ante semejante deshonra a la caballería.",
            },
            next: Some(6),
        }
    );
    graph.add_node(
        6,
        Node::Simple {
            content: "Sant Jordi confesó que durante todo este tiempo había estado disfrazándose de dragón, aprovechándose de la gente de Montblanc que, aterrada, no paraba de darle regalos y cosas gratis.",
            extra: SimpleExtra {
                illustration: textures.dragon_returns_from_holidays.clone(),
                additional_text: "Pero antes de poder terminar sus explicaciones y justificarse, el dragón (que se ve que había estado de vacaciones) regresó, dejando helado al pobre caballero.",
            },
            next: Some(7),
        },
    );
    graph.add_node(
        7,
        Node::Simple {
            content: "Al final, el dragón y la princesa se vieron convertidos en unos improbables aliados ante la idiotez de Sant Jordi y el pueblo, por lo que acordaron una manera que, por fin, pondría un final a la disputa... (dibujo de Cleodolinda pisando la cabeza del Sant Jordi disfrazado de dragón delante de Montblanc mientras el dragón observa orgulloso de fondo oculto).",
            extra: SimpleExtra {
                illustration: textures.princess_x_dragon.clone(),
                additional_text: "Y así, la villa de Montblanc regresó a la normalidad y tranquilidad que la caracterizaba... Al menos, hasta que apareciese el siguiente \"dragón\"...",
            },
            next: None,
        },
    );
    graph.add_node(
        8,
        Node::Simple {
            content: "Fascinada por el extraño ejemplar ante el que se econtraba, empezó a examinarlo exhaustivamente",
            extra: SimpleExtra {
                illustration: textures.princess_analyzing_jordi_dragon.clone(),
                additional_text: "Mientras que a cualquier otro habitante de la villa le temblarían las manos de pavor, a ella le temblaban de pura emoción.",
            },
            next: Some(9),
        },
    );
    graph.add_node(
        9,
        Node::Simple {
            content: "Tras su concienzudo análisis, para su decepción, vio claramente que se encontraba ante un disfraz. ¡Nunca hubo dragón! La princesa, triste, acusó a la bestia, quien resultó ser... ¿Sant Jordi?",
            extra: SimpleExtra {
                illustration: textures.jordi_dragon_confesses.clone(),
                additional_text: "Oscilando entre el puchero y la ira, Cleodolinda exigió explicaciones al vil caballero.",
            },
            next: Some(6),
        },
    );
    graph.add_node(
        10,
        Node::Simple {
            content: "A diferencia del resto de la villa, Cleodolinda tenía alguna que otra luz en la sesera y reconoció rápidamente que lo que tenía delante de ella no era un dragón...",
            extra: SimpleExtra {
                illustration: textures.princess_unmasks_jordi_dragon.clone(),
                additional_text: "De hecho, fijándose bien, se podían ver claramente las marcas de costura en el traje de dragón.",
            },
            next: Some(11),
        },
    );
    graph.add_node(
        11,
        Node::Simple {
            content: "La princesa, cansada ya un poco de tanta tontería, le arrancó la máscara al falso dragón y reveló que detrás de toda esta farsa estaba... ¡Sant Jordi!",
            extra: SimpleExtra {
                illustration: textures.jordi_dragon_confesses.clone(),
                additional_text: "Lejos de enfadarse o indignarse, Cleodolinda se vio inundada por una terrible ola de frustración.",
            },
            next: Some(6),
        },
    );
    graph.add_node(
        12,
        Node::Fork {
            content: "Sant Jordi, sacando pecho y guiñándole un ojo, le dijo a Cleodolinda: \"tranquila princesa, yo me encargo del dragoncito\". Y sin esperar respuesta, el caballero se adentró en las profundidades de la cueva de la bestia. Entonces, Cleodolinda decidió...",
            choices: vec![
                NodeChoice {
                    text: "...entrar con Sant Jordi en la cueva.",
                    illustration: textures.bevy.clone(), // TODO: I can't handle not having an illustration right now...
                    additional_text: Arc::new(|context| if context.princesa_guerrera { "Cleodolinda, todavía con ganas de algo de acción, siguió al caballero procurando que no la viera." } else if context.fan_dragones { "Cleodolinda, demasiado ansiosa por la posibilidad de ver un dragón real, siguió de cerca a Sant Jordi y se adentró tras él en la guarida de la criatura." } else if context.princesa_rechazada { "No creyéndose del todo al quizás algo flipado caballero, Cleodolinda le siguió de cerca y se adentró en la guarida de la bestia." } else { unreachable!("Some flag should have been set.") }),
                    state_change: Arc::new(|context| context.entrar_cueva = true),
                    next: Arc::new(|_| 13),
                },
                NodeChoice {
                    text: "...esperar fuera.",
                    illustration: textures.bevy.clone(), // TODO: There was no texture...
                    additional_text: Arc::new(|context| if context.princesa_guerrera { "Cleodolinda, sabiendo que Sant Jordi no podría acabar con un dragón él solo, se quedó fuera esperando a que saliera despavorido en busca de ayuda." } else if context.fan_dragones { "Siendo consciente del peligro que tenía exponerse a un dragón real, Cleodolinda perefirió esperar a que el caballero cumpliera su cometido. Ya podría examinarlo bien después." } else if context.princesa_rechazada { "Cleodolinda esperó fuera de la cueva a que Sant Jordi finalizara su deber, si es que era tan machote como decía ser..." } else { unreachable!("Some flag should have been set.") }),
                    state_change: Arc::new(|context| context.salir_cueva = true),
                    next: Arc::new(|_| 17),
                }
            ],
        },
    );
    graph.add_node(
        13,
        Node::Simple {
            content: "Para sorpresa de Cleodolinda (aunque tampoco mucha siendo sinceros) lo único que encontró dentro de la cueva fue a Sant Jordi, solo, gritando y gruñendo, luchando contra su propia sombra...",
            extra: SimpleExtra {
                illustration: textures.sant_jordi_fighting_alone.clone(),
                additional_text: "Si bien lo que estaba haciendo Sant Jordi no estaba muy claro, sobre lo que no cabía duda era que allí dentro no había dragón alguno.",
            },
            next: Some(14),
        },
    );
    graph.add_node(
        14,
        Node::Simple {
            content: "Cleodolinda, se dio cuenta de que algo no cuadraba, sobretodo cuando en un rincón de la guarida vio lo que parecía ser un disfraz de dragón algo cutre tirado en el suelo... ¡Sant Jordi había sido el dragón todo ese tiempo!",
            extra: SimpleExtra {
                illustration: textures.bevy.clone(), // TODO: There was no texture...
                additional_text: "El caballero claramente le había estado tomando el pelo pero... ¿por qué?",
            },
            next: Some(15),
        },
    );
    graph.add_node(
        15,
        Node::Simple {
            content: "La princesa acusó a Sant Jordi, quien confesó que durante todo este tiempo había estado disfrazándose de dragón, para luego darse caza él mismo y llevarse la fama. Cleodolinda empezó a reñir severamente al caballero que, sorprendentemente, parecía aterrado...",
            extra: SimpleExtra {
                illustration: textures.dragon_returns_from_holidays.clone(),
                additional_text: "...aunque no precisamente de ella...",
            },
            next: Some(16),
        },
    );
    graph.add_node(
        16,
        Node::Simple {
            content: "Tras unas arduas negociaciones, al final dragón y princesa acordaron con Sant Jordi una resolución que definitivamente resolvería la situación...",
            extra: SimpleExtra {
                illustration: textures.princess_x_dragon.clone(),
                additional_text: "Y así, la villa de Montblanc regresó a la normalidad y tranquilidad que la caracterizaba... Al menos, hasta que apareciese el siguiente \"dragón\"...",
            },
            next: None,
        },
    );
    graph.add_node(
        17,
        Node::Simple {
            content: "Al poco tiempo, Sant Jordi, jadeante y sucio, salió de la cueva sujetando... ¡la cabeza del dragón!",
            extra: SimpleExtra {
                illustration: textures.sant_jordi_with_dragon_head.clone(),
                additional_text: "Así a la luz del día tampoco parecía gran cosa, pero bueno, a Sant Jordi se le veía orgulloso.",
            },
            next: Some(18),
        },
    );
    graph.add_node(
        18,
        Node::Simple {
            content:
                "Cleodolinda, algo escéptica, se dio cuenta de que algo no terminaba de encajar...", // TODO: Should use context here to differentiate a bit.
            extra: SimpleExtra {
                illustration: textures.bevy.clone(), // TODO: There was no texture...
                additional_text: "De hecho, se podían ver las marcas de costura en la \"cabeza del dragón\"... ¿Qué pretendía Sant Jordi con todo esto?",
            },
            next: Some(15),
        },
    );
    graph.add_node(
        19,
        Node::Simple {
            content: "Sant Jordi, notablemente incómodo, le contó a Cleodolinda que las rosas habían salido de la sangre del dragón cuando lo mató...",
            extra: SimpleExtra {
                illustration: textures.bevy.clone(), // TODO: No texture.
                additional_text: "... aunque quizás eso no fuera del todo cierto...", // TODO: Falta ilustración del dragón sensual saliendo de la cueva.
            },
            next: Some(20),
        },
    );
    graph
}

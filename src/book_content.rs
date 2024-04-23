use std::sync::Arc;

use bevy::prelude::*;

use crate::{
    arc,
    book::default_text_styles,
    graph::{ChoiceTrait, Graph, Node},
    loading::{FontAssets, Illustrations, UiTextures},
};

type WithContext<Result> = Arc<dyn Fn(&BookContext) -> Result + Sync + Send>;
type WithContextMut<Result> = Arc<dyn Fn(&mut BookContext) -> Result + Sync + Send>;

#[derive(Clone)]
pub struct TextStyles {
    pub normal: TextStyle,
    pub highlighted: TextStyle,
    pub first_letter: TextStyle,
}

pub struct SimpleContent {
    pub text: WithContext<&'static str>,
    pub text_styles: Option<TextStyles>,
    /// Decoration images that go below the text.
    pub decorations: Vec<Handle<Image>>,
}

impl Default for SimpleContent {
    fn default() -> Self {
        Self {
            text: arc!(""),
            text_styles: None,
            decorations: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! content {
    ($inner:expr, $fonts:expr, first_letter = $color:expr, too_many_options, $(decorations = $decoration:expr),*) => {
        SimpleContent {
            text: $inner,
            text_styles: Some(TextStyles {
                first_letter: TextStyle {
                    font: $fonts.first_letter.clone(),
                    font_size: 100.,
                    color: $color,
                },
                ..default_text_styles($fonts, true)
            }),
            decorations: vec![$($decoration),*],
        }
    };
    ($inner:expr, $fonts:expr, first_letter = $color:expr, too_many_options) => {
        SimpleContent {
            text: $inner,
            text_styles: Some(TextStyles {
                first_letter: TextStyle {
                    font: $fonts.first_letter.clone(),
                    font_size: 100.,
                    color: $color,
                },
                ..default_text_styles($fonts, true)
            }),
            ..default()
        }
    };
    ($inner:expr, $fonts:expr, first_letter = $color:expr) => {
        SimpleContent {
            text: $inner,
            text_styles: Some(TextStyles {
                first_letter: TextStyle {
                    font: $fonts.first_letter.clone(),
                    font_size: 100.,
                    color: $color,
                },
                ..default_text_styles($fonts, false)
            }),
            ..default()
        }
    };
    ($inner:expr, $fonts:expr, highlighted = $color:expr) => {
        SimpleContent {
            text: $inner,
            text_styles: Some(TextStyles {
                highlighted: TextStyle {
                    font: $fonts.normal.clone(),
                    font_size: 30.,
                    color: $color,
                },
                ..default_text_styles($fonts, false)
            }),
            ..default()
        }
    };
    ($inner:expr, $(decorations = $decoration:expr),*) => {
        SimpleContent {
            text: $inner,
            text_styles: None,
            decorations: vec![$($decoration),*],
        }
    };
    ($inner:expr) => {
        SimpleContent {
            text: $inner,
            text_styles: None,
            ..default()
        }
    };
}

pub type BookGraph = Graph<SimpleContent, SimpleExtra, NodeChoice, BookContext>;

pub struct SimpleExtra {
    pub illustration: Option<Handle<Image>>,
    pub additional_text: WithContext<&'static str>,
    pub decorations: Vec<Handle<Image>>,
}

impl Default for SimpleExtra {
    fn default() -> Self {
        Self {
            illustration: None,
            additional_text: arc!(""),
            decorations: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct NodeChoice {
    pub text: WithContext<&'static str>,
    pub illustration: Option<Handle<Image>>,
    pub additional_text: WithContext<&'static str>,
    pub state_change: WithContextMut<()>,
    pub next: WithContext<usize>,
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
}

pub fn get_book_content(
    textures: &Res<Illustrations>,
    fonts: &Res<FontAssets>,
    ui_textures: &Res<UiTextures>,
) -> BookGraph {
    let mut graph = BookGraph::new();
    graph.add_node(
        0,
        Node::Fork {
            content: content!(arc!("Erase una vez, un *terrible dragón* que atemorizaba la villa de Montblanc..."), decorations = ui_textures.rabbit_troubadour.clone()),
            choices: vec![
                NodeChoice {
                    text: arc!("Erase una vez, *un hombre claramente disfrazado de dragón* que, por algún motivo, atemorizaba la villa de Montblanc..."),
                    illustration: Some(textures.sant_jordi_disguised_as_dragon.clone()),
                    additional_text: arc!("Quizás fuera que se veían pocos dragones o que realmente tenían ganas de poder decir que habían visto uno, pero sea como fuere, la villa entera parecía convencida de ello."),
                    state_change: arc!(|context| context.santjordi_disfrazado = true),
                    next: arc!(1),
                },
                NodeChoice {
                    text: arc!("Erase una vez, *un dragón normalito*, con sus problemas y sus cosas, cuya presencia atemorizaba la villa de Montblanc..."),
                    illustration: Some(textures.normal_dragon.clone()),
                    additional_text: arc!("Realmente no les hacía nada, pero un dragón gigante escupefuego era el objetivo perfecto sobre el que proyectar sus problemas."),
                    state_change: arc!(|context| context.dragon_normal = true),
                    next: arc!(25),
                },
            ],
        },
    );
    graph.add_node(
        1,
        Node::Simple {
            content: content!(arc!("Todavía inmersos en sus delirios, los habitantes de la villa empezaron a mandarle reses y animales, esperando que eso saciara su \"ira\". Sin embargo, *no tuvo mucho efecto*..."), fonts, highlighted = Color::hex("282c83").unwrap()),
            extra: SimpleExtra {
                illustration: Some(textures.jordi_dragon_with_cow.clone()),
                additional_text: arc!("De hecho, algunas reses eran casi tan grandes como el \"dragón\"..."),
                ..default()
            },
            next: Some(2),
        },
    );
    graph.add_node(
        2,
        Node::Fork {
            content: content!(arc!("Con la villa desesperada, el rey no tuvo más alternativa que hacer un *sorteo* para ofrecerle a la bestia sacrificios humanos, ignorando que el destino, confuso y sibilino, se conjuraría en su contra con el sacrificio de su propia hija..."), fonts, first_letter = Color::hex("3d793a").unwrap(), too_many_options, decorations = ui_textures.green_fancy_underline.clone()),
            choices: vec![
                NodeChoice {
                    text: arc!("La princesa Cleodolinda, cansada de los inútiles intentos de la gente de la villa por calmar la situación, se ofreció para *matar al dragón*"),
                    illustration: Some(textures.princess_go_kill_dragon.clone()),
                    additional_text: arc!("Espada en mano y paso decididio, se dirigió a la cueva donde se escondía el dragón."),
                    state_change: arc!(|context| context.princesa_guerrera = true),
                    next: arc!(3),
                },
                NodeChoice {
                    text: arc!("La princesa Cleodolinda, *deseosa por conocer a un dragón de verdad*, se ofreció voluntaria para solventar la situación"),
                    illustration: Some(textures.princess_excited_to_be_picked.clone()),
                    additional_text: arc!("Con su enciclopedia favorita de dragones bajo el brazo, se dirigió a la cueva de la bestia sin ningún temor."),
                    state_change: arc!(|context| context.fan_dragones = true),
                    next: arc!(3),
                },
                NodeChoice {
                    text: arc!("Pero cuando vió que él mismo fue el escogido en el sorteo, les dijo a todos que la princesa Cleodolinda, su propia hija, fue la *desaventurada víctima de la fortuna...*"),
                    illustration: Some(textures.king_picks_princess.clone()),
                    additional_text: arc!("No fue una gran sorpresa para Cleodolinda, pero aún asi aceptó su destino y se encaminó hacia la guarida del dragón."),
                    state_change: arc!(|context| context.princesa_rechazada = true),
                    next: arc!(3),
                },
            ],
        },
    );
    graph.add_node(
        3,
        Node::Fork {
            content: content!(arc!("Cleodolinda aceptó su destino valientemente y tras un corto viaje llegó a la guarida del dragón, donde *se encontró...*")),
            choices: vec![
                NodeChoice {
                    text: arc!("Con un misterioso hombre disfrazado de dragón asando malvaviscos."),
                    illustration: Some(textures.sant_jordi_making_marshmallows.clone()),
                    additional_text: arc!("Es bien sabido que no hay nada que un dragón disfrute más que sentarse como un humano a asar dulces delante del fuego..."),
                    state_change: arc!(|context| context.encuentra_santjordi_disfrazado = true),
                    next: arc!(|context| if context.princesa_guerrera { 4 } else if context.fan_dragones { 8 } else if context.princesa_rechazada { 10 } else { unreachable!("Some flag must be true") }),
                },
                NodeChoice {
                    text: arc!("Con un apuesto caballero haciéndose pinturas tribales de guerra, alentándose a sí mismo..."),
                    illustration: Some(textures.sant_jordi_warrior.clone()),
                    additional_text: arc!("Su libro favorito era \"El arte de la guerra\" y especulaba con terrenos"),
                    state_change: arc!(|context| context.encuentra_santjordi_flipado = true),
                    next: arc!(12),
                },
                NodeChoice {
                    text: arc!("Con un apuesto caballero frente a la guarida de la temible bestia sosteniendo ferozmente un... ¿ramo de flores?"),
                    illustration: Some(textures.sant_jordi_roses.clone()),
                    additional_text: arc!("No todos los caballeros tienen que ser agresivos, seguramente Sant Jordi tendría sus \"métodos\"..."),
                    state_change: arc!(|context| context.encuentra_santjordi_enamorado = true),
                    next: arc!(19),
                }
            ],
        },
    );
    graph.add_node(
        4,
        Node::Simple {
            content: content!(arc!("Algo decepcionada ante el inocente y algo adorable dragonzuelo que se encontraba ante ella, Cleodolinda no se amilanó y le atizó a la bestia tremendo capón en la sesera...")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_punches_jordi_dragon.clone()),
                additional_text: arc!("Estaba claro que la princesa no había hecho todo este viaje para quedarse ahora de brazos cruzados."),
                ..default()
            },
            next: Some(5),
        },
    );
    graph.add_node(
        5,
        Node::Simple {
            content: content!(arc!("Tal fue la contundencia del mamporrazo que la cabeza del \"dragón\" salió volando, revelando al hombre que se había estado haciendo pasar por la bestia todo este tiempo: ¡Sant Jordi!")),
            extra: SimpleExtra {
                illustration: Some(textures.jordi_dragon_confesses.clone()),
                additional_text: arc!("La princesa, iracunda, exigió explicaciones a Sant Jordi, indignada ante semejante deshonra a la caballería."),
                ..default()
            },
            next: Some(6),
        }
    );
    graph.add_node(
        6,
        Node::Simple {
            content: content!(arc!("Sant Jordi confesó que durante todo este tiempo había estado disfrazándose de dragón, aprovechándose de la gente de Montblanc que, aterrada, no paraba de darle regalos y cosas gratis.")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_returns_from_holidays.clone()),
                additional_text: arc!("Pero antes de poder terminar sus explicaciones y justificarse, el dragón (que se ve que había estado de vacaciones) regresó, dejando helado al pobre caballero."),
                ..default()
            },
            next: Some(7),
        },
    );
    graph.add_node(
        7,
        Node::Simple {
            content: content!(arc!("Al final, el dragón y la princesa se vieron convertidos en unos improbables aliados ante la idiotez de Sant Jordi y el pueblo, por lo que acordaron una manera que, por fin, pondría un final a la disputa...")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_x_dragon.clone()),
                additional_text: arc!("Y así, la villa de Montblanc regresó a la normalidad y tranquilidad que la caracterizaba... Al menos, hasta que apareciese el siguiente \"dragón\"..."),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        8,
        Node::Simple {
            content: content!(arc!("Fascinada por el extraño ejemplar ante el que se econtraba, empezó a examinarlo exhaustivamente")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_analyzing_jordi_dragon.clone()),
                additional_text: arc!("Mientras que a cualquier otro habitante de la villa le temblarían las manos de pavor, a ella le temblaban de pura emoción."),
                ..default()
            },
            next: Some(9),
        },
    );
    graph.add_node(
        9,
        Node::Simple {
            content: content!(arc!("Tras su concienzudo análisis, para su decepción, vio claramente que se encontraba ante un disfraz. ¡Nunca hubo dragón! La princesa, triste, acusó a la bestia, quien resultó ser... ¿Sant Jordi?")),
            extra: SimpleExtra {
                illustration: Some(textures.jordi_dragon_confesses.clone()),
                additional_text: arc!("Oscilando entre el puchero y la ira, Cleodolinda exigió explicaciones al vil caballero."),
                ..default()
            },
            next: Some(6),
        },
    );
    graph.add_node(
        10,
        Node::Simple {
            content: content!(arc!("A diferencia del resto de la villa, Cleodolinda tenía alguna que otra luz en la sesera y reconoció rápidamente que lo que tenía delante de ella no era un dragón...")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_unmasks_jordi_dragon.clone()),
                additional_text: arc!("De hecho, fijándose bien, se podían ver claramente las marcas de costura en el traje de dragón."),
                ..default()
            },
            next: Some(11),
        },
    );
    graph.add_node(
        11,
        Node::Simple {
            content: content!(arc!("La princesa, cansada ya un poco de tanta tontería, le arrancó la máscara al falso dragón y reveló que detrás de toda esta farsa estaba... ¡Sant Jordi!")),
            extra: SimpleExtra {
                illustration: Some(textures.jordi_dragon_confesses.clone()),
                additional_text: arc!("Lejos de enfadarse o indignarse, Cleodolinda se vio inundada por una terrible ola de frustración."),
                ..default()
            },
            next: Some(6),
        },
    );
    graph.add_node(
        12,
        Node::Fork {
            content: content!(arc!("Sant Jordi, sacando pecho y guiñándole un ojo, le dijo a Cleodolinda: \"tranquila princesa, yo me encargo del dragoncito\". Y sin esperar respuesta, el caballero se adentró en las profundidades de la cueva de la bestia. Entonces, Cleodolinda decidió...")),
            choices: vec![
                NodeChoice {
                    text: arc!("Cleodolinda decidió entrar con Sant Jordi en la cueva."),
                    illustration: None,
                    additional_text: arc!(|context| if context.princesa_guerrera { "Cleodolinda, todavía con ganas de algo de acción, siguió al caballero procurando que no la viera." } else if context.fan_dragones { "Cleodolinda, demasiado ansiosa por la posibilidad de ver un dragón real, siguió de cerca a Sant Jordi y se adentró tras él en la guarida de la criatura." } else if context.princesa_rechazada { "No creyéndose del todo al quizás algo flipado caballero, Cleodolinda le siguió de cerca y se adentró en la guarida de la bestia." } else { unreachable!("Some flag should have been set.") }),
                    state_change: arc!(|context| context.entrar_cueva = true),
                    next: arc!(13),
                },
                NodeChoice {
                    text: arc!("Cleodolinda decidió esperar fuera."),
                    illustration: None,
                    additional_text: arc!(|context| if context.princesa_guerrera { "Cleodolinda, sabiendo que Sant Jordi no podría acabar con un dragón él solo, se quedó fuera esperando a que saliera despavorido en busca de ayuda." } else if context.fan_dragones { "Siendo consciente del peligro que tenía exponerse a un dragón real, Cleodolinda perefirió esperar a que el caballero cumpliera su cometido. Ya podría examinarlo bien después." } else if context.princesa_rechazada { "Cleodolinda esperó fuera de la cueva a que Sant Jordi finalizara su deber, si es que era tan machote como decía ser..." } else { unreachable!("Some flag should have been set.") }),
                    state_change: arc!(|context| context.salir_cueva = true),
                    next: arc!(17),
                }
            ],
        },
    );
    graph.add_node(
        13,
        Node::Simple {
            content: content!(arc!("Para sorpresa de Cleodolinda (aunque tampoco mucha siendo sinceros) lo único que encontró dentro de la cueva fue a Sant Jordi, solo, gritando y gruñendo, luchando contra su propia sombra...")),
            extra: SimpleExtra {
                illustration: Some(textures.sant_jordi_fighting_alone.clone()),
                additional_text: arc!("Si bien lo que estaba haciendo Sant Jordi no estaba muy claro, sobre lo que no cabía duda era que allí dentro no había dragón alguno."),
                ..default()
            },
            next: Some(14),
        },
    );
    graph.add_node(
        14,
        Node::Simple {
            content: content!(arc!("Cleodolinda, se dio cuenta de que algo no cuadraba, sobretodo cuando en un rincón de la guarida vio lo que parecía ser un disfraz de dragón algo cutre tirado en el suelo... ¡Sant Jordi había sido el dragón todo ese tiempo!")),
            extra: SimpleExtra {
                illustration: None,
                additional_text: arc!("El caballero claramente le había estado tomando el pelo pero... ¿por qué?"),
                decorations: vec![ui_textures.cat.clone()],
            },
            next: Some(15),
        },
    );
    graph.add_node(
        15,
        Node::Simple {
            content: content!(arc!("La princesa acusó a Sant Jordi, quien confesó que durante todo este tiempo había estado disfrazándose de dragón, para luego darse caza él mismo y llevarse la fama. Cleodolinda empezó a reñir severamente al caballero que, sorprendentemente, parecía aterrado...")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_returns_from_holidays.clone()),
                additional_text: arc!("Aunque no precisamente de ella..."),
                ..default()
            },
            next: Some(16),
        },
    );
    graph.add_node(
        16,
        Node::Simple {
            content: content!(arc!("Tras unas arduas negociaciones, al final dragón y princesa acordaron con Sant Jordi una resolución que definitivamente resolvería la situación...")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_x_dragon.clone()),
                additional_text: arc!("Y así, la villa de Montblanc regresó a la normalidad y tranquilidad que la caracterizaba... Al menos, hasta que apareciese el siguiente \"dragón\"..."),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        17,
        Node::Simple {
            content: content!(arc!("Al poco tiempo, Sant Jordi, jadeante y sucio, salió de la cueva sujetando... ¡la cabeza del dragón!")),
            extra: SimpleExtra {
                illustration: Some(textures.sant_jordi_with_dragon_head.clone()),
                additional_text: arc!("Así a la luz del día tampoco parecía gran cosa, pero bueno, a Sant Jordi se le veía orgulloso."),
                ..default()
            },
            next: Some(18),
        },
    );
    graph.add_node(
        18,
        Node::Simple {
            content:
                content!(arc!("Cleodolinda, algo escéptica, se dio cuenta de que algo no terminaba de encajar...")), // TODO: Should use context here to differentiate a bit.
            extra: SimpleExtra {
                illustration: None,
                additional_text: arc!("De hecho, se podían ver las marcas de costura en la \"cabeza del dragón\"... ¿Qué pretendía Sant Jordi con todo esto?"),
                decorations: vec![ui_textures.rabbit_troubadour.clone()],
            },
            next: Some(15),
        },
    );
    graph.add_node(
        19,
        Node::Simple {
            content: content!(arc!("Sant Jordi, notablemente incómodo, le contó a Cleodolinda que las rosas habían salido de la sangre del dragón cuando lo mató...")),
            extra: SimpleExtra {
                illustration: None,
                additional_text: arc!("Cleodolinda, algo escéptica ante el más que evidente nerviosismo del caballero, le pidió pruebas de la muerte del dragón."),
                decorations: vec![ui_textures.snail_boy.clone()],
            },
            next: Some(20),
        },
    );
    graph.add_node(
        20,
        Node::Simple {
            content: content!(arc!("Sant Jordi, viéndose obligado a improvisar, se metió dentro de la guarida del dragón y, pasado un buen rato, emergió de nuevo sosteniendo... ¡La cabeza del dragón!")),
            extra: SimpleExtra {
                illustration: Some(textures.sant_jordi_with_dragon_head.clone()),
                additional_text: arc!(|context| if context.princesa_rechazada || context.princesa_guerrera { "Cleodolinda, algo escéptica, se dio cuenta de que algo no terminaba de encajar..." } else if context.fan_dragones { "Cleodolinda, que de dragones sabía un rato, reconoció que claramente esa cabeza no era real..." } else { unreachable!("Some flag should've been set."); }),
                ..default()
            },
            next: Some(21),
        },
    );
    graph.add_node(
        21,
        Node::Simple {
            content: content!(arc!("Si cabía todavía alguna duda de que Sant Jordi no estaba siendo del todo sincero...")),
            extra: SimpleExtra {
                illustration: Some(textures.sensual_dragon_coming_out_of_cave.clone()),
                additional_text: arc!("Digamos que de pronto se esclareció todo..."),
                ..default()
            },
            next: Some(22),
        },
    );
    graph.add_node(
        22,
        Node::Fork {
            content: content!(arc!("La princesa, escandalizada, decidió...")),
            choices: vec![
                NodeChoice {
                    text: arc!("Chivarse del romance al resto de la villa."),
                    illustration: None, // TODO: No texture.
                    additional_text: arc!("Cotilla y morbosa, Cleodolinda corrió a la villa para compartir con el todo el mundo la aberrante y cómica relación contra natura que dragón y caballero estaban manteniendo."),
                    state_change: arc!({}),
                    next: arc!(23),
                },
                NodeChoice {
                    text: arc!("Guardar el secreto y contar en la villa una leyenda inventada para cubrirles."),
                    illustration: None, // TODO: No texture.
                    additional_text: arc!("Conmovida por semejante muestra de amor en contra de toda clase de prejuicios, la princesa decidió ayudarles y mantener su tórrido romance en secreto."),
                    state_change: arc!({}),
                    next: arc!(24),
                },
            ],
        },
    );
    graph.add_node(
        23,
        Node::Simple {
            content: content!(arc!("La villa se enteró del romance prohibido del dragón y Sant Jordi, lo que obligó a la pareja a vivir su luna de miel en Escocia")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_and_jordi_dragon_go_to_scotland.clone()),
                additional_text: arc!("Seguro que allí serían más tolerantes..."),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        24,
        Node::Simple {
            content: content!(arc!("Cleodolinda contó en la villa la gran hazaña del caballero Sant Jordi, quien venció al dragón y de cuya sangre brotaron rosas.")),
            extra: SimpleExtra {
                illustration: None, // TODO: No texture.
                additional_text: arc!("Y así, una vez más, el amor prevaleció por encima de todo."),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        25,
        Node::Simple {
            content: content!(arc!("Convencidos de que el dragón albergaba perversas intenciones, trataron de adelantarse a la desgracia ofreciéndole numerosas reses y animales, ignorando por completo que la bestia era vegana...")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_with_cow.clone()),
                additional_text: arc!("Al menos el dragón tendría compañía..."),
                ..default()
            },
            next: Some(26),
        },
    );
    graph.add_node(
        26,
        Node::Fork {
            content: content!(arc!("Con la villa desesperada, el rey no tuvo más alternativa que hacer un sorteo para ofrecerle a la bestia sacrificios humanos, ignorando que el destino, confuso y sibilino, se conjuraría en su contra con el sacrificio de su propia hija...")),
            choices: vec![
                NodeChoice {
                    text: arc!("La princesa Cleodolinda, cansada de los inútiles intentos de la gente de la villa por calmar la situación, se ofreció voluntaria para matar al dragón."),
                    illustration: Some(textures.princess_go_kill_dragon.clone()),
                    additional_text: arc!("Espada en mano y paso decidido, se dirigió a la cueva donde se escondía el dragón."),
                    state_change: arc!(|context| context.princesa_guerrera = true),
                    next: arc!(27),
                },
                NodeChoice {
                    text: arc!("La princesa Cleodolinda, deseosa por conocer a un dragón de verdad, se ofreció voluntaria para utilizar sus extensos conocimientos de dragones para solventar la situación."),
                    illustration: Some(textures.princess_excited_to_be_picked.clone()),
                    additional_text: arc!("Con su enciclopedia favorita de dragones bajo el brazo, se dirigió a la cueva de la bestia sin ningún temor."),
                    state_change: arc!(|context| context.fan_dragones = true),
                    next: arc!(27),
                },
                NodeChoice {
                    text: arc!("Pero cuando vió que él mismo fue el escogido en el sorteo, preso de su propia cobardía, les dijo a todos que la princesa Cleodolinda, su propia hija, fue la desaventurada víctima de la fortuna..."),
                    illustration: Some(textures.king_picks_princess.clone()),
                    additional_text: arc!("No fue una gran sorpresa para Cleodolinda, pero aún así aceptó su destino y se encaminó hacia la guarida del dragón."),
                    state_change: arc!(|context| context.princesa_rechazada = true),
                    next: arc!(27),
                },
            ],
        },
    );
    graph.add_node(
        27,
        Node::Fork {
            content: content!(arc!("Cleodolinda aceptó su destino valientemente y tras un corto viaje llegó a la guarida del dragón, donde se encontró...")),
            choices: vec![
                NodeChoice {
                    text: arc!("Con un misterioso hombre disfrazado de dragón asando malvaviscos."),
                    illustration: Some(textures.sant_jordi_making_marshmallows.clone()),
                    additional_text: arc!("Es bien sabido que no hay nada que un dragón disfrute más que sentarse como un humano a asar dulces delante del fuego..."),
                    state_change: arc!(|context| context.encuentra_santjordi_disfrazado = true),
                    next: arc!(28),
                },
                NodeChoice {
                    text: arc!("Con un apuesto caballero haciéndose pinturas tribales de guerra, alentándose a sí mismo..."),
                    illustration: Some(textures.sant_jordi_warrior.clone()),
                    additional_text: arc!("Su libro favorito era \"El arte de la guerra\" y especulaba con terrenos"),
                    state_change: arc!(|context| context.encuentra_santjordi_flipado = true),
                    next: arc!(34),
                },
                NodeChoice {
                    text: arc!("Con un apuesto caballero frente a la guarida de la temible bestia sosteniendo ferozmente un... ¿ramo de flores?"),
                    illustration: Some(textures.sant_jordi_roses.clone()),
                    additional_text: arc!("No todos los caballeros tienen que ser agresivos, seguramente Sant Jordi tendría sus \"métodos\"..."),
                    state_change: arc!(|context| context.encuentra_santjordi_enamorado = true),
                    next: arc!(41),
                },
            ],
        },
    );
    graph.add_node(
        28,
        Node::Fork {
            content: content!(arc!("Cleodolinda, intrigada por el extraño dragón, decidió acercarse para \"tantear\" el terreno, a lo que el dragón respondió...")),
            choices: vec![
                NodeChoice {
                    text: arc!("Dejándose llevar."),
                    illustration: Some(textures.jordi_dragon_accepts_princess.clone()),
                    additional_text: arc!("El amor funciona de manera misteriosa... ¿Quiénes somos nosotros para juzgar?"),
                    state_change: arc!({}),
                    next: arc!(29),
                },
                NodeChoice {
                    text: arc!("Rechazando, incómodo, el extraño acercamiento de Cleodolinda."),
                    illustration: Some(textures.jordi_dragon_rejects_princess.clone()),
                    additional_text: arc!("Se ve que no era su tipo... Ni su especie..."),
                    state_change: arc!({}),
                    next: arc!(31),
                },
            ],
        },
    );
    graph.add_node(
        29,
        Node::Simple {
            content: content!(arc!("Viendo que el extraño dragonzuelo respondía a su acercamiento, Cleodolinda decidió que era el momento de revelar su verdadera forma: ¡Ella era el dragón!")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_dragon.clone()),
                additional_text: arc!("¡Rayos y centellas! ¡Quién se lo hubiera imaginado! ¿La hija del Rey... el dragón?"),
                ..default()
            },
            next: Some(30),
        },
    );
    graph.add_node(
        30,
        Node::Simple {
            content: content!(arc!("Cleodolinda y el extraño dragón, que claramente era un humano disfrazado, fueron ambos víctimas de las flechas del caótico Cupido e iniciaron un romance que desafiaba a toda lógica y raciocinio.")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_x_jordi_dragon.clone()),
                additional_text: arc!("Y así, vivieron felices para siempre demostrando una vez más que el amor es ciego."),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        31,
        Node::Simple {
            content: content!(arc!("Cleodolinda no se amilanó y prosiguió con sus acercamientos, asumiendo que solo se estaba haciendo el difícil. Sin embargo el dragón, sintiéndose acosado por los continuos e inexplicables avances de la princesa, reveló su verdadera identidad. ¡Era Sant Jordi todo este tiempo!")),
            extra: SimpleExtra {
                illustration: Some(textures.jordi_dragon_confesses.clone()),
                additional_text: arc!("El reputado caballero disfrazado de dragón... ¿acaso ya no quedaba gente honrada?"),
                ..default()
            },
            next: Some(32),
        },
    );
    graph.add_node(
        32,
        Node::Simple {
            content: content!(arc!("Cleodolina, sintiéndose engañada y despechada, reveló su verdadera forma. ¡Ella era el dragón!")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_dragon.clone()),
                additional_text: arc!("Sant Jordi no lo sabía pero todo este tiempo había estado jugando con fuego..."),
                ..default()
            },
            next: Some(33),
        },
    );
    graph.add_node(
        33,
        Node::Simple {
            content: content!(arc!("Cleodolinda, como reprimenda, chamuscó un poco a Sant Jordi que huyó despavorido. La princesa, habiéndose ya quitado del medio al problemático falso dragón que le hacía un flaco favor a los suyos, celebró con el resto de la villa la vuelta a la normalidad.")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_chases_jordi_dragon.clone()),
                additional_text: arc!(""),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        34,
        Node::Fork {
            content: content!(arc!("Sant Jordi, sacando pecho y guiñándole un ojo, le dijo a Cleodolinda: \"tranquila princesa, yo me encargo del dragoncito\". Y sin esperar respuesta, el caballero se adentró en las profundidades de la cueva de la bestia. Mientras tanto, Cleodolinda decidió...")),
            choices: vec![
                NodeChoice {
                    text: arc!("entrar con Sant Jordi a la cueva."),
                    illustration: None,
                    additional_text: arc!(|context| if context.princesa_guerrera { "Cleodolinda, todavía con ganas de algo de acción, siguió al caballero procurando que no la viera." } else if context.fan_dragones { "Cleodolinda, demasiado ansiosa por la posibilidad de ver un dragón real, siguió de cerca a Sant Jordi y se adentró tras él en la guarida de la criatura." } else if context.princesa_rechazada { "No creyéndose del todo al quizás algo flipado caballero, Cleodolinda le siguió de cerca y se adentró en la guarida de la bestia." } else { unreachable!("Some flag should've been set.") }),
                    state_change: arc!({}),
                    next: arc!(35),
                },
                NodeChoice {
                    text: arc!("esperar fuera."),
                    illustration: None,
                    additional_text: arc!(|context| if context.princesa_guerrera { "Cleodolinda, sabiendo que Sant Jordi no podría acabar con un dragón él solo, se quedó fuera esperando a que saliera despavorido en busca de ayuda." } else if context.fan_dragones { "Siendo consciente del peligro que tenía exponerse a un dragón real, Cleodolinda perefirió esperar a que el caballero cumpliera su cometido. Ya podría examinarlo bien después." } else if context.princesa_rechazada { "Cleodolinda esperó fuera de la cueva a que Sant Jordi finalizara su deber, si es que era tan machote como decía ser..." } else { unreachable!("Some flag should've been set.") }),
                    state_change: arc!({}),
                    next: arc!(38),
                },
            ],
        },
    );
    graph.add_node(
        35,
        Node::Simple {
            content: content!(arc!("Para sorpresa de Cleodolinda (aunque tampoco mucha siendo sinceros) lo único que encontró dentro de la cueva fue a Sant Jordi, solo, gritando y gruñendo, luchando contra su propia sombra....")),
            extra: SimpleExtra {
                illustration: Some(textures.sant_jordi_fighting_alone.clone()),
                additional_text: arc!("Si bien lo que estaba haciendo Sant Jordi no estaba muy claro, sobre lo que no cabía duda era que allí dentro no había dragón alguno."),
                ..default()
            },
            next: Some(36),
        },
    );
    graph.add_node(
        36,
        Node::Simple {
            content: content!(arc!("Entonces Cleodolina, irritada, reveló su verdadera forma. ¡Ella era el dragón!")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_dragon.clone()),
                additional_text: arc!("Sant Jordi no lo sabía pero todo este tiempo había estado jugando con fuego..."),
                ..default()
            },
            next: Some(37),
        },
    );
    graph.add_node(
        37,
        Node::Simple {
            content: content!(arc!("Cleodolinda, como reprimenda, chamuscó un poco a Sant Jordi que huyó despavorido del reino. La Princesa volvió a la villa y todos celebraron su victoria.")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_chases_jordi_dragon.clone()),
                additional_text: arc!(""),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        38,
        Node::Simple {
            content: content!(arc!("Al poco tiempo, Sant Jordi, jadeante y sucio, salió de la cueva sujetando... ¡la cabeza del dragón! ")),
            extra: SimpleExtra {
                illustration: Some(textures.sant_jordi_with_dragon_head.clone()),
                additional_text: arc!("Así a la luz del día tampoco parecía gran cosa, pero bueno, a Sant Jordi se le veía orgulloso."),
                ..default()
            },
            next: Some(39),
        },
    );
    graph.add_node(
        39,
        Node::Simple {
            content: content!(arc!(
                "Entonces Cleodolina, irritada, reveló su verdadera forma. ¡Ella era el dragón!"
            )),
            extra: SimpleExtra {
                illustration: Some(textures.princess_dragon.clone()),
                additional_text: arc!("Sant Jordi no lo sabía pero todo este tiempo había estado jugando con fuego..."),
                ..default()
            },
            next: Some(40),
        },
    );
    graph.add_node(
        40,
        Node::Simple {
            content: content!(arc!("Cleodolinda, como reprimenda, chamuscó un poco a Sant Jordi que huyó despavorido del reino. La Princesa volvió a la villa y todos celebraron su victoria.")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_chases_jordi_dragon.clone()), // TODO: This doesn't seem right.
                additional_text: arc!(""),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        41,
        Node::Fork {
            content: content!(arc!("La princesa, escandalizada, decidió...")),
            choices: vec![
                NodeChoice {
                    text: arc!("Chivarse del romance al resto de la villa."),
                    illustration: Some(textures.princess_thinking.clone()),
                    additional_text: arc!("Cotilla y morbosa, Cleodolinda corrió a la villa para compartir con el todo el mundo la aberrante y cómica relación contra natura que dragón y caballero estaban manteniendo."),
                    state_change: arc!({}),
                    next: arc!(42),
                },
                NodeChoice {
                    text: arc!("Guardar el secreto."),
                    illustration: Some(textures.princess_thinking.clone()),
                    additional_text: arc!("Conmovida por semejante muestra de amor en contra de toda clase de prejuicios, la princesa decidió ayudarles a mantener su tórrido romance en secreto."),
                    state_change: arc!({}),
                    next: arc!(43),
                },
            ],
        },
    );
    graph.add_node(
        42,
        Node::Simple {
            content: content!(arc!("La villa se enteró del romance prohibido del dragón y Sant Jordi, lo que obligó a la pareja a vivir su luna de miel en Escocia")),
            extra: SimpleExtra {
                illustration: Some(textures.dragon_x_sant_jordi.clone()),
                additional_text: arc!("Seguro que allí serían más tolerantes... "),
                ..default()
            },
            next: None,
        },
    );
    graph.add_node(
        43,
        Node::Simple {
            content: content!(arc!("Cleodolinda contó en la villa la gran hazaña del caballero Sant Jordi, quien venció al dragón y de cuya sangre brotaron rosas.")),
            extra: SimpleExtra {
                illustration: Some(textures.princess_thinking.clone()),
                additional_text: arc!("Y así, una vez más, el amor prevaleció por encima de todo."),
                ..default()
            },
            next: None,
        },
    );
    graph
}

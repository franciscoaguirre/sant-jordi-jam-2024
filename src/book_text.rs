use bevy::prelude::*;
use bevy_talks::prelude::*;

use crate::resources::Illustrations;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct IllustrationArray(pub Vec<Handle<Image>>);

pub fn get_book_text(illustrations: &Res<Illustrations>) -> TalkBuilder {
    Talk::builder().say("Érase una vez...").empty_node().choose(vec![
        ("un dragón, bastante normal, probablemente con problemas de autoestima, que atemorizaba la villa de Montblanc.", Talk::builder()),
        ("un humano con un disfraz de dragón cutre, que atemorizaba la villa de Montblanc.", Talk::builder()),
    ]).with_component(IllustrationArray(vec![illustrations.0.get("normal-dragon").unwrap().clone(), illustrations.0.get("sant-jordi-disguised-as-dragon").unwrap().clone()]))
}

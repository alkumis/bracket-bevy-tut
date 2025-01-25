use crate::components::*;
use crate::AppSet;
use bevy::prelude::*;
use bracket_lib::bevy::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(BTermBuilder::simple_80x50())
        .add_systems(Update, render.in_set(AppSet::Update));
}

fn render(
    ctx: Res<BracketContext>,
    non_player: Query<(&Position, &Renderable), Without<Player>>,
    player: Query<(&Position, &Renderable), With<Player>>,
) {
    ctx.cls();

    // These shenanigans to separate the query and to chain
    // are so that the player gets set last and doesn't flicker
    non_player
        .iter()
        .chain(player.iter())
        .for_each(|(pos, render)| {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        });
}

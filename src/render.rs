use super::components::*;
use bevy::prelude::*;
use bracket_lib::bevy::*;

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

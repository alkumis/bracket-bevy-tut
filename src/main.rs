use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bracket_lib::bevy::*;
use std::cmp::{max, min};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(BTermBuilder::simple_80x50())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, render).chain())
        .run();
}

#[derive(Component)]
struct Renderable {
    glyph: u16,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Position { x: 0, y: 0 })
        .insert(Player);
}

fn move_player(kb_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    if let Some(kbi) = kb_input.get_pressed().next() {
        let (player_x, player_y) = match kbi {
            KeyCode::ArrowUp | KeyCode::KeyW => (0, -1),
            KeyCode::ArrowDown | KeyCode::KeyS => (0, 1),
            KeyCode::ArrowLeft | KeyCode::KeyA => (-1, 0),
            KeyCode::ArrowRight | KeyCode::KeyD => (1, 0),
            _ => (0, 0),
        };

        query.par_iter_mut().for_each(|mut pos| {
            pos.x = min(79, max(0, pos.x + player_x));
            pos.y = min(49, max(0, pos.y + player_y));
        });
    }
}

fn render(ctx: Res<BracketContext>, query: Query<(&Position, &Renderable)>) {
    ctx.cls();

    query.par_iter().for_each(|(pos, render)| {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    });
}

mod components;
mod map;
mod player;
mod render;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bracket_lib::bevy::*;
use bracket_lib::random::RandomNumberGenerator;

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
        .configure_sets(Update, (AppSet::RecordInput, AppSet::Update).chain())
        .insert_resource(Map::new())
        .add_systems(Startup, (init_player, draw_map))
        .add_systems(Update, (move_player, render).chain())
        .run();
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
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

#[derive(Component, PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Resource)]
struct Map {
    tiles: Vec<TileType>,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

impl Map {
    pub fn new() -> Self {
        let mut map = vec![TileType::Floor; 80 * 50];

        for x in 0..80 {
            map[xy_idx(x, 0)] = TileType::Wall;
            map[xy_idx(x, 49)] = TileType::Wall;
        }

        for y in 0..50 {
            map[xy_idx(0, y)] = TileType::Wall;
            map[xy_idx(79, y)] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = xy_idx(x, y);
            if idx != xy_idx(40, 25) {
                map[idx] = TileType::Wall;
            }
        }

        Map { tiles: map }
    }
}

fn init_player(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Position { x: 40, y: 25 })
        .insert(Player);
}

fn draw_map(mut commands: Commands, map: Res<Map>) {
    for y in 0..50 {
        for x in 0..80 {
            let idx = xy_idx(x, y);
            match map.tiles[idx] {
                TileType::Floor => {
                    commands
                        .spawn_empty()
                        .insert(Renderable {
                            glyph: to_cp437('.'),
                            fg: RGB::named(GRAY),
                            bg: RGB::named(BLACK),
                        })
                        .insert(Position { x, y });
                }
                TileType::Wall => {
                    commands
                        .spawn_empty()
                        .insert(Renderable {
                            glyph: to_cp437('#'),
                            fg: RGB::named(GREEN),
                            bg: RGB::named(BLACK),
                        })
                        .insert(Position { x, y });
                }
            }
        }
    }
}

fn move_player(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player_pos: Query<&mut Position, With<Player>>,
    map: Res<Map>,
) {
    if let Some(kbi) = kb_input.get_pressed().next() {
        let (player_x, player_y) = match kbi {
            KeyCode::ArrowUp | KeyCode::KeyW => (0, -1),
            KeyCode::ArrowDown | KeyCode::KeyS => (0, 1),
            KeyCode::ArrowLeft | KeyCode::KeyA => (-1, 0),
            KeyCode::ArrowRight | KeyCode::KeyD => (1, 0),
            _ => (0, 0),
        };

        if let Some(mut pos) = player_pos.single_mut().into() {
            let destination_idx = xy_idx(pos.x + player_x, pos.y + player_y);

            if map.tiles[destination_idx] != TileType::Wall {
                pos.x = (pos.x + player_x).clamp(0, 79);
                pos.y = (pos.y + player_y).clamp(0, 49);
            }
        }
    }
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

use crate::components::*;
use bevy::prelude::*;
use bracket_lib::bevy::*;
use bracket_lib::random::RandomNumberGenerator;

pub fn plugin(app: &mut App) {
    app.insert_resource(Map::new())
        .add_systems(Startup, spawn_map);
}

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
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

fn spawn_map(mut commands: Commands, map: Res<Map>) {
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

use crate::components::*;
use crate::rect::Rect;
use bevy::prelude::*;
use bracket_lib::bevy::*;
use bracket_lib::random::RandomNumberGenerator;
use std::cmp::{max, min};

pub fn plugin(app: &mut App) {
    app.insert_resource(Map::new_map_rooms_and_corridors())
        .add_systems(Startup, spawn_map);
}

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

impl Map {
    pub fn new_map_rooms_and_corridors() -> Self {
        let mut map = vec![TileType::Wall; 80 * 50];

        let mut rooms: Vec<Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - w - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false;
                }
            }
            if ok {
                apply_room_to_map(&new_room, &mut map);

                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room)
            }
        }

        Self { tiles: map, rooms }
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
                            fg: RGB::named(DARKOLIVEGREEN1),
                            bg: RGB::named(BLACK),
                        })
                        .insert(Position { x, y });
                }
            }
        }
    }
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx] = TileType::Floor;
        }
    }
}

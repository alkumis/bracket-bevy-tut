use bevy::prelude::*;
use bracket_lib::bevy::*;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

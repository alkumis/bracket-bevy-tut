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

#[derive(Component)]
pub struct Visible;

#[derive(Component, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
}

#[derive(Component, Debug)]
pub struct Monster;

#[derive(Component, Debug)]
pub struct EntityName {
    pub name: String,
}

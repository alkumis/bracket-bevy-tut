use crate::{components::*, map::Map};
use bevy::{
    app::{App, Startup},
    prelude::{Commands, Res},
};
use bracket_lib::{
    bevy::{to_cp437, BLACK, RED, RGB},
    random::RandomNumberGenerator,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_monsters);
}

fn spawn_monsters(mut commands: Commands, map: Res<Map>) {
    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let glyph = match rng.roll_dice(1, 2) {
            1 => to_cp437('g'),
            _ => to_cp437('o'),
        };

        commands
            .spawn_empty()
            .insert(Renderable {
                glyph,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .insert(Position { x, y })
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
            })
            .insert(Monster);
    }
}

use crate::{components::*, map::Map, AppSet, RunningState};
use bevy::prelude::*;
use bracket_lib::{
    bevy::{to_cp437, Point, BLACK, RED, RGB},
    random::RandomNumberGenerator,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_monsters).add_systems(
        Update,
        shout_insults
            .run_if(not(in_state(RunningState::Paused)))
            .in_set(AppSet::Update),
    );
}

fn spawn_monsters(mut commands: Commands, map: Res<Map>) {
    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let (glyph, name) = match rng.roll_dice(1, 2) {
            1 => (to_cp437('g'), "Goblin".to_string()),
            _ => (to_cp437('o'), "Orc".to_string()),
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
            .insert(Monster)
            .insert(EntityName {
                name: format!("{} #{}", &name, i + 1),
            });
    }
}

fn shout_insults(
    player_pos: Query<&Position, With<Player>>,
    mut monsters: Query<(&EntityName, &Viewshed), With<Monster>>,
) {
    let player_pos = player_pos.single();
    let player_pos = Point::new(player_pos.x, player_pos.y);
    for (name, viewshed) in monsters.iter_mut() {
        if viewshed.visible_tiles.contains(&player_pos) {
            info!["{:?} shouts insults", name.name];
        }
    }
}

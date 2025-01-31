use crate::components::*;
use crate::map::*;
use crate::AppSet;
use crate::RunningState;
use bevy::prelude::*;
use bracket_lib::bevy::*;

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, spawn_player)
        .add_systems(Update, move_player.in_set(AppSet::RecordInput));
}

fn spawn_player(mut commands: Commands, map: Res<Map>) {
    let (x, y) = map.rooms[0].center();
    commands
        .spawn_empty()
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Position { x, y })
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .insert(Visible)
        .insert(Player);
}

fn move_player(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut player_pos: Query<&mut Position, With<Player>>,
    map: Res<Map>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    match kb_input.get_just_pressed().next() {
        Some(kbi) => {
            let (player_x, player_y) = match kbi {
                KeyCode::ArrowLeft | KeyCode::KeyA | KeyCode::KeyH | KeyCode::Numpad4 => (-1, 0),
                KeyCode::ArrowRight | KeyCode::KeyD | KeyCode::KeyL | KeyCode::Numpad6 => (1, 0),
                KeyCode::ArrowUp | KeyCode::KeyW | KeyCode::KeyK | KeyCode::Numpad8 => (0, -1),
                KeyCode::ArrowDown | KeyCode::KeyS | KeyCode::KeyJ | KeyCode::Numpad2 => (0, 1),
                _ => (0, 0),
            };

            if (player_x, player_y) == (0, 0) {
                next_state.set(RunningState::Paused);
            } else {
                if let Some(mut pos) = player_pos.single_mut().into() {
                    let player_x = (pos.x + player_x).clamp(0, map.width - 1);
                    let player_y = (pos.y + player_y).clamp(0, map.height - 1);

                    let destination_idx = map.xy_idx(player_x, player_y);

                    if map.tiles[destination_idx] != TileType::Wall {
                        (pos.x, pos.y) = (player_x, player_y);
                    }
                }
                next_state.set(RunningState::Running);
            }
        }
        None => next_state.set(RunningState::Paused),
    }
}

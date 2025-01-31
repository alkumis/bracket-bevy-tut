use crate::{components::*, map::Map, AppSet, RunningState};
use bevy::prelude::*;
use bracket_lib::{
    bevy::{Point, DARKOLIVEGREEN, DARKOLIVEGREEN1, GRAY1, RGB},
    prelude::field_of_view,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            set_viewsheds,
            player_visibility,
            tile_visibility,
            monster_visibility,
        )
            .chain()
            .run_if(in_state(RunningState::Running).or(in_state(RunningState::Load)))
            .in_set(AppSet::Tick),
    );
}

fn set_viewsheds(mut viewsheds: Query<(&mut Viewshed, &Position)>, map: Res<Map>) {
    viewsheds.par_iter_mut().for_each(|(mut viewshed, pos)| {
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    });
}

fn player_visibility(player_viewshed: Query<&Viewshed, With<Player>>, mut map: ResMut<Map>) {
    map.visible_tiles.clear();
    for vis in player_viewshed.single().visible_tiles.iter() {
        let idx = map.xy_idx(vis.x, vis.y);
        map.revealed_tiles.insert(idx);
        map.visible_tiles.insert(idx);
    }
}

fn tile_visibility(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &mut Renderable), With<TileType>>,
    map: Res<Map>,
) {
    for idx in map.revealed_tiles.clone().iter() {
        let (x, y) = map.idx_xy(*idx);
        if let Some((entity, _, mut render)) = query
            .iter_mut()
            .find(|(_, pos, _)| pos.x == x && pos.y == y)
        {
            commands.entity(entity).insert_if_new(Visible);
            render.fg = if map.visible_tiles.contains(idx) {
                match map.tiles[*idx] {
                    TileType::Floor => RGB::named(DARKOLIVEGREEN),
                    TileType::Wall => RGB::named(DARKOLIVEGREEN1),
                }
            } else {
                RGB::named(GRAY1)
            }
        }
    }
}

fn monster_visibility(
    mut commands: Commands,
    mut query: Query<(Entity, &Position), With<Monster>>,
    map: Res<Map>,
) {
    for (entity, pos) in query.iter_mut() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles.contains(&idx) {
            commands.entity(entity).insert_if_new(Visible);
        } else {
            commands.entity(entity).remove::<Visible>();
        }
    }
}

use crate::{components::*, map::Map, AppSet};
use bevy::prelude::*;
use bracket_lib::{bevy::Point, prelude::field_of_view};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (set_viewsheds, player_visibility, set_visibility)
            .chain()
            .run_if(run_if_dirty)
            .in_set(AppSet::Tick),
    );
}

fn run_if_dirty(mut viewsheds: Query<&Viewshed>) -> bool {
    let mut is_dirty = false;

    if let Some(viewshed) = viewsheds.iter_mut().next() {
        is_dirty = viewshed.dirty;
    }

    is_dirty
}

fn set_viewsheds(mut viewsheds: Query<(&mut Viewshed, &Position)>, map: Res<Map>) {
    for (mut viewshed, pos) in viewsheds.iter_mut() {
        viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    }
}

fn player_visibility(player_viewshed: Query<&Viewshed, With<Player>>, mut map: ResMut<Map>) {
    for vis in player_viewshed.single().visible_tiles.iter() {
        let idx = map.xy_idx(vis.x, vis.y);
        map.revealed_tiles.push(idx);
    }
}

fn set_visibility(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Renderable), (Without<Player>, Without<Visible>)>,
    map: Res<Map>,
) {
    for idx in map.revealed_tiles.iter() {
        let (x, y) = map.idx_xy(*idx);
        if let Some((entity, _, _)) = query.iter().find(|(_, pos, _)| pos.x == x && pos.y == y) {
            commands.entity(entity).insert(Visible);
        }
    }
}

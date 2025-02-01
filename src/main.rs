mod components;
mod map;
mod monster;
mod player;
mod rect;
mod render;
mod visibility;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .init_state::<RunningState>()
        .configure_sets(
            Update,
            (
                AppSet::RecordInput,
                AppSet::Visibility,
                AppSet::Update,
                AppSet::Render,
            )
                .chain(),
        )
        .add_plugins((
            map::plugin,
            monster::plugin,
            player::plugin,
            visibility::plugin,
            render::plugin,
        ))
        .run();
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Record player input.
    RecordInput,
    /// Tick systems based on input.
    Visibility,
    /// Update - everything else goes here
    Update,
    /// Do everything else (consider splitting this into further variants).
    Render,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum RunningState {
    #[default]
    Load,
    Paused,
    Running,
}

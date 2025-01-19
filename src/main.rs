use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bracket_lib::bevy::*;
// use bracket_lib::prelude::FontCharType;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Wasm builds will check for meta files (that don't exist) if this isn't set.
            // This causes errors and even panics in web builds on itch.
            // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(BTermBuilder::simple_80x50())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Renderable {
    glyph: u16,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

fn setup(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Position { x: 0, y: 0 });
}

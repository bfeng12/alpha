use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, time::FixedTimestep};

mod animation;
mod player;

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player::handle_input)
                .with_system(player::update_controllable_velocities.after(player::handle_input))
                .with_system(
                    player::update_player_state_and_direction
                        .after(player::update_controllable_velocities),
                )
                .with_system(
                    player::handle_animation_state_update
                        .after(player::update_player_state_and_direction),
                )
                .with_system(player::move_entities.after(player::update_controllable_velocities))
                .with_system(
                    animation::animate_sprites.after(player::handle_animation_state_update),
                ),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/characters/player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        6,
        10,
        Some(Vec2::new(16.0, 16.0)),
        Some(Vec2::new(8.0, 16.0)),
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((player::PlayerBundle {
        name: player::PlayerName("Aldrazus".to_string()),
        animation: animation::Animation::new(0.1, 0, 5, animation::AnimationMode::Repeating, false),
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        ..default()
    },));
}

use bevy::{prelude::*, time::FixedTimestep};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_player.before(check_for_collisions))
                .with_system(animate_sprite)
                .with_system(update_player_direction.before(animate_sprite)),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction_x = 0.0;
    let mut direction_y = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        direction_y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction_y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction_x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::A) {
        direction_x -= 1.0;
    }

    let new_player_position =
        player_transform.translation + Vec3::new(direction_x, direction_y, 0.0) * 500.0 * TIME_STEP;

    player_transform.translation = new_player_position;
}

fn update_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Direction, With<Player>>,
) {
    let mut player_direction = query.single_mut();

    if keyboard_input.pressed(KeyCode::W) {
        *player_direction = Direction::Up
    }

    if keyboard_input.pressed(KeyCode::S) {
        *player_direction = Direction::Down
    }

    if keyboard_input.pressed(KeyCode::D) {
        *player_direction = Direction::Left
    }

    if keyboard_input.pressed(KeyCode::A) {
        *player_direction = Direction::Right
    }
}

fn check_for_collisions() {}

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
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Collider,
        Direction::Down,
    ));
}

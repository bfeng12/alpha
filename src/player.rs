use crate::animation;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct PlayerName(pub String);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: PlayerName,
    pub state: PlayerState,
    pub direction: Direction,
    pub controls: Controls,
    pub velocity: Velocity,

    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub animation: animation::Animation,
}

#[derive(Component, Default)]
pub struct Controls {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Component, Default, PartialEq)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Attacking,
    Dead,
}

#[derive(Component, Default)]
pub struct Velocity(Vec2);

const PLAYER_IDLE_UP_ROW_INDEX: usize = 2;
const PLAYER_IDLE_DOWN_ROW_INDEX: usize = 0;
const PLAYER_IDLE_HORIZONTAL_ROW_INDEX: usize = 1;
const PLAYER_WALK_UP_ROW_INDEX: usize = 5;
const PLAYER_WALK_DOWN_ROW_INDEX: usize = 3;
const PLAYER_WALK_HORIZONTAL_ROW_INDEX: usize = 4;

const PLAYER_ANIMATION_LENGTH: usize = 6;

pub fn handle_animation_state_update(
    mut query: Query<
        (
            &mut animation::Animation,
            &PlayerState,
            &Direction,
            &mut TextureAtlasSprite,
        ),
        Or<(Changed<PlayerState>, Changed<Direction>)>,
    >,
) {
    for (mut animation, state, direction, mut sprite) in query.iter_mut() {
        match (state, direction) {
            (PlayerState::Idle, Direction::Up) => {
                animation.start = PLAYER_IDLE_UP_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_IDLE_UP_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = false;
                sprite.flip_x = false;
                sprite.index = animation.start;
            }
            (PlayerState::Idle, Direction::Down) => {
                animation.start = PLAYER_IDLE_DOWN_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_IDLE_DOWN_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = false;
                sprite.flip_x = false;
                sprite.index = animation.start;
            }
            (PlayerState::Idle, Direction::Left) => {
                animation.start = PLAYER_IDLE_HORIZONTAL_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_IDLE_HORIZONTAL_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = true;
                sprite.flip_x = true;
                sprite.index = animation.start;
            }
            (PlayerState::Idle, Direction::Right) => {
                animation.start = PLAYER_IDLE_HORIZONTAL_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_IDLE_HORIZONTAL_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = false;
                sprite.flip_x = false;
                sprite.index = animation.start;
            }
            (PlayerState::Walking, Direction::Up) => {
                animation.start = PLAYER_WALK_UP_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_WALK_UP_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = false;
                sprite.flip_x = false;
                sprite.index = animation.start;
            }
            (PlayerState::Walking, Direction::Down) => {
                animation.start = PLAYER_WALK_DOWN_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_WALK_DOWN_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = false;
                sprite.flip_x = false;
                sprite.index = animation.start;
            }
            (PlayerState::Walking, Direction::Left) => {
                animation.start = PLAYER_WALK_HORIZONTAL_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_WALK_HORIZONTAL_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = true;
                sprite.flip_x = true;
                sprite.index = animation.start;
            }
            (PlayerState::Walking, Direction::Right) => {
                animation.start = PLAYER_WALK_HORIZONTAL_ROW_INDEX * PLAYER_ANIMATION_LENGTH;
                animation.end = (PLAYER_WALK_HORIZONTAL_ROW_INDEX + 1) * PLAYER_ANIMATION_LENGTH;
                animation.flip_x = false;
                sprite.flip_x = false;
                sprite.index = animation.start;
            }
            (PlayerState::Attacking, ..) => (),
            (PlayerState::Dead, ..) => (),
        }
    }
}

pub fn update_player_state_and_direction(
    mut query: Query<(&Velocity, &mut Direction, &mut PlayerState, &Controls)>,
) {
    let (Velocity(velocity), mut direction, mut player_state, controls) = query.single_mut();

    match *player_state {
        PlayerState::Idle => {
            if velocity.x.abs() > 0.0 || velocity.y.abs() > 0.0 {
                *player_state = PlayerState::Walking;
            }
        }
        PlayerState::Walking => {
            if *velocity == Vec2::new(0.0, 0.0) {
                *player_state = PlayerState::Idle;
            }
        }
        _ => (),
    }

    if *direction != Direction::Up
        && controls.up
        && !controls.down
        && !controls.left
        && !controls.right
    {
        *direction = Direction::Up;
    } else if *direction != Direction::Down
        && controls.down
        && !controls.up
        && !controls.left
        && !controls.right
    {
        *direction = Direction::Down;
    } else if *direction != Direction::Left
        && controls.left
        && !controls.up
        && !controls.down
        && !controls.right
    {
        *direction = Direction::Left;
    } else if *direction != Direction::Right
        && controls.right
        && !controls.up
        && !controls.down
        && !controls.left
    {
        *direction = Direction::Right;
    }
}

pub fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Controls>) {
    for mut controls in query.iter_mut() {
        controls.up = keyboard_input.pressed(KeyCode::W);
        controls.left = keyboard_input.pressed(KeyCode::A);
        controls.down = keyboard_input.pressed(KeyCode::S);
        controls.right = keyboard_input.pressed(KeyCode::D);
    }
}

pub fn update_controllable_velocities(mut query: Query<(&Controls, &mut Velocity)>) {
    for (controls, mut velocity) in query.iter_mut() {
        if controls.up && !controls.down {
            velocity.0.y = 1.0;
        } else if controls.down && !controls.up {
            velocity.0.y = -1.0;
        } else {
            velocity.0.y = 0.0;
        }

        if controls.left && !controls.right {
            velocity.0.x = -1.0;
        } else if controls.right && !controls.left {
            velocity.0.x = 1.0;
        } else {
            velocity.0.x = 0.0;
        }
    }
}

const TIME_STEP: f32 = 1.0 / 60.0;

pub fn move_entities(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, Velocity(velocity)) in query.iter_mut() {
        let (x, y) = (velocity.x, velocity.y);

        let new_position = transform.translation + Vec3::new(x, y, 0.0) * 500.0 * TIME_STEP;

        transform.translation = new_position;
    }
}

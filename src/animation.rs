use bevy::{
    prelude::*,
    sprite::TextureAtlasSprite,
    time::{Time, Timer},
};

#[derive(Default)]
pub enum AnimationMode {
    Once,
    #[default]
    Repeating,
}

#[derive(Default)]
pub enum AnimationState {
    #[default]
    Playing,
    Finished,
}

#[derive(Component, Default)]
pub struct Animation {
    timer: Timer,
    pub start: usize,
    pub end: usize,
    pub play_mode: AnimationMode,
    pub flip_x: bool,
    pub state: AnimationState,
    pub current_frame: usize,
}

impl Animation {
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn new(
        seconds_per_frame: f32,
        start: usize,
        end: usize,
        play_mode: AnimationMode,
        flip_x: bool,
    ) -> Self {
        Self {
            timer: Timer::from_seconds(seconds_per_frame, TimerMode::Repeating),
            start,
            end,
            play_mode,
            flip_x,
            state: AnimationState::Playing,
            current_frame: 0,
        }
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Animation)>,
) {
    let delta = time.delta();
    for (mut sprite, mut animation) in query.iter_mut() {
        match animation.state {
            AnimationState::Playing => {
                animation.timer.tick(delta);
                if animation.timer.just_finished() {
                    match animation.play_mode {
                        AnimationMode::Once => {
                            animation.current_frame += 1;
                            sprite.index = animation.start + animation.current_frame;
                            if sprite.index == animation.end - 1 {
                                animation.state = AnimationState::Finished;
                            }
                        }
                        AnimationMode::Repeating => {
                            animation.current_frame =
                                (animation.current_frame + 1) % animation.len();
                            sprite.index = animation.start + animation.current_frame;
                        }
                    }

                    sprite.flip_x = animation.flip_x;
                }
            }
            AnimationState::Finished => {}
        }
    }
}

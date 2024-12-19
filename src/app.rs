use std::time::Duration;

use crate::periodic_timer::PeriodicTimer;
use crate::state::State;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use macroquad_particles::{AtlasConfig, BlendMode, ColorCurve, Emitter, EmitterConfig};

pub struct App {
    state: State,
    position: Vec2,
    velocity: Vec2,
    rocket_texture: Texture2D,
    fire_emitter: Emitter,
    screen_size: Vec2,
    rocket_size: Vec2,
    timer_1_hz: PeriodicTimer,
    timer_10_hz: PeriodicTimer,
    last_play_time: Option<f64>,
    last_key_pressed_time: Option<f64>,
    fps: i32,
}

impl App {
    pub async fn new() -> Self {
        info!("created app instance");

        Self {
            state: State::Init,
            position: vec2(screen_width() / 2.0 - 50.0, screen_height() - 50.0),
            velocity: vec2(0.0, 0.0),
            rocket_texture: load_texture("assets/rocket.png").await.unwrap(),
            fire_emitter: Emitter::new(EmitterConfig {
                local_coords: false,
                emitting: false,
                texture: Some(load_texture("assets/smoke_fire.png").await.unwrap()),
                lifetime: 0.4,
                lifetime_randomness: 0.1,
                amount: 10,
                initial_direction_spread: 0.5,
                initial_velocity: -300.0,
                atlas: Some(AtlasConfig::new(4, 4, 8..)),
                size: 30.0,
                blend_mode: BlendMode::Additive,
                colors_curve: ColorCurve {
                    start: MAGENTA,
                    mid: ORANGE,
                    end: WHITE,
                },
                ..Default::default()
            }),
            screen_size: vec2(screen_width(), screen_height()),
            rocket_size: vec2(100.0, 200.0),
            timer_1_hz: PeriodicTimer::new(Duration::from_secs(1)),
            timer_10_hz: PeriodicTimer::new(Duration::from_secs_f64(0.1)),
            last_play_time: None,
            last_key_pressed_time: None,
            fps: get_fps(),
        }
    }

    pub async fn run(&mut self) -> ! {
        info!("run!");

        prevent_quit();

        loop {
            self.dispatch().await;
        }
    }

    async fn dispatch(&mut self) {
        self.screen_size = vec2(screen_width(), screen_height());

        if let Some(last_play_time) = self.last_play_time {
            self.timer_1_hz
                .update(Duration::from_secs_f64(get_time() - last_play_time));
            self.timer_10_hz
                .update(Duration::from_secs_f64(get_time() - last_play_time));
        }

        if self.timer_1_hz.triggered() {
            self.fps = get_fps();
        }

        match self.state {
            State::Init => {
                self.state = State::Idle;
            }
            State::Idle => {
                let size = vec2(250.0, 50.0);
                let position = self.screen_size / 2.0 - size / 2.0;

                root_ui().window(hash!(), position, size, |ui| {
                    ui.label(vec2(40.0, 15.0), "Press [space] to play");

                    if is_key_down(KeyCode::Space) {
                        self.state = State::Playing;
                        self.last_key_pressed_time = Some(get_time());
                    }
                });
            }
            State::Playing => {
                self.fire_emitter.config.emitting = true;

                if is_key_down(KeyCode::Enter) {
                    self.state = State::Paused;
                    self.last_key_pressed_time = Some(get_time());
                }

                if is_key_down(KeyCode::Tab) {
                    // TODO: display help message
                    self.last_key_pressed_time = Some(get_time());
                }

                // Apply gravity.
                self.velocity.y += 0.05;

                if is_key_down(KeyCode::Up) {
                    self.velocity.y -= 0.2;
                    self.last_key_pressed_time = Some(get_time());
                }
                if is_key_down(KeyCode::Down) {
                    self.velocity.y += 0.2;
                    self.last_key_pressed_time = Some(get_time());
                }
                if is_key_down(KeyCode::Left) {
                    self.velocity.x -= 0.2;
                    self.last_key_pressed_time = Some(get_time());
                }
                if is_key_down(KeyCode::Right) {
                    self.velocity.x += 0.2;
                    self.last_key_pressed_time = Some(get_time());
                }

                if let Some(last_key_pressed_time) = self.last_key_pressed_time {
                    if (get_time() - last_key_pressed_time) > 1.0 && self.timer_10_hz.triggered() {
                        if self.velocity.x < 0.0 {
                            self.velocity.x = (self.velocity.x + 0.2).min(0.0);
                        } else if self.velocity.x > 0.0 {
                            self.velocity.x = (self.velocity.x - 0.2).max(0.0);
                        }
                    }
                }

                // Update position
                self.position += self.velocity;

                // Screen bounds
                if self.position.x < 0.0 {
                    self.position.x = 0.0;
                    self.velocity.x = 0.0;
                }
                if self.position.x > screen_width() - self.rocket_size.x {
                    self.position.x = screen_width() - self.rocket_size.x;
                    self.velocity.x = 0.0;
                }
                if self.position.y < 0.0 + self.rocket_size.y {
                    self.position.y = 0.0 + self.rocket_size.y;
                    self.velocity.y = 0.0;
                }
                if self.position.y > screen_height() {
                    self.position.y = screen_height();
                    self.velocity.y = 0.0;
                }
            }
            State::Paused => {
                let size = vec2(250.0, 50.0);
                let position = self.screen_size / 2.0 - size / 2.0;

                root_ui().window(hash!(), position, size, |ui| {
                    ui.label(vec2(40.0, 15.0), "Press [space] to resume");

                    if is_key_down(KeyCode::Space) {
                        self.state = State::Playing;
                        self.last_key_pressed_time = Some(get_time());
                    }
                });
            }
        }

        self.last_play_time = Some(get_time());

        self.draw().await;
    }

    async fn draw(&mut self) {
        clear_background(BLUE);

        draw_text_ex(
            format!("uptime: {:.2}", get_time()).as_str(),
            10.0,
            20.0,
            TextParams {
                font_size: 20,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex(
            format!("FPS: {}", self.fps).as_str(),
            10.0,
            40.0,
            TextParams {
                font_size: 20,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex(
            "press [enter] to pause",
            10.0,
            60.0,
            TextParams {
                font_size: 20,
                color: BLACK,
                ..Default::default()
            },
        );

        draw_text_ex(
            format!("state: {}", self.state).as_str(),
            10.0,
            80.0,
            TextParams {
                font_size: 20,
                color: BLACK,
                ..Default::default()
            },
        );

        self.fire_emitter.draw(vec2(
            self.position.x + (self.rocket_size.x / 2.0),
            self.position.y - 10.0,
        ));

        draw_texture_ex(
            &self.rocket_texture,
            self.position.x,
            self.position.y - self.rocket_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.rocket_size),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}

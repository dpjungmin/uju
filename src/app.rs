use crate::state::State;

use macroquad::prelude::*;
use macroquad::ui::widgets::Window;
use macroquad::ui::{hash, root_ui};
use macroquad_particles::{AtlasConfig, BlendMode, ColorCurve, Emitter, EmitterConfig};

pub struct App {
    state: State,
    position: Vec2,
    velocity: Vec2,
    rocket_texture: Texture2D,
    fire_emitter: Emitter,
    draw_fire_emitter: bool,
    screen_size: Vec2,
    rocket_size: Vec2,
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
            draw_fire_emitter: false,
            screen_size: vec2(screen_width(), screen_height()),
            rocket_size: vec2(100.0, 200.0),
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

        match self.state {
            State::Init => {
                self.state = State::Idle;
            }
            State::Idle => {
                let size = vec2(250.0, 50.0);
                let position = self.screen_size / 2.0 - size / 2.0;

                Window::new(hash!(), position, size).ui(&mut root_ui(), |ui| {
                    ui.label(vec2(5.0, 5.0), "Press [space] to play!");

                    if is_key_down(KeyCode::Space) {
                        self.state = State::Playing;
                    }
                });
            }
            State::Playing => {
                self.draw_fire_emitter = true;

                if is_key_down(KeyCode::Enter) {
                    self.state = State::Paused;
                }

                if is_key_down(KeyCode::Up) {
                    self.velocity.y -= 0.2;
                }
                if is_key_down(KeyCode::Down) {
                    self.velocity.y += 0.2;
                }
                if is_key_down(KeyCode::Left) {
                    self.velocity.x -= 0.2;
                }
                if is_key_down(KeyCode::Right) {
                    self.velocity.x += 0.2;
                }

                // Apply gravity
                self.velocity.y += 0.05;

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

                Window::new(hash!(), position, size).ui(&mut root_ui(), |ui| {
                    ui.label(vec2(5.0, 5.0), "Press [space] to resume.");

                    if is_key_down(KeyCode::Space) {
                        self.state = State::Playing;
                    }
                });
            }
        }

        self.draw().await;
    }

    async fn draw(&mut self) {
        clear_background(BLUE);

        draw_text(self.state.into(), 10.0, 20.0, 15.0, WHITE);
        draw_text(&format!("{:.2}", get_time()), 10.0, 40.0, 15.0, WHITE);

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

        if self.draw_fire_emitter {
            self.fire_emitter.draw(vec2(
                self.position.x + (self.rocket_size.x / 2.0),
                self.position.y - 10.0,
            ));
        }

        next_frame().await;
    }
}

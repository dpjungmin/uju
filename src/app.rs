use crate::state::State;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::Window};

#[derive(Debug)]
pub struct App {
    state: State,
    position: Vec2,
    velocity: Vec2,
    rocket_texture: Texture2D,
    screen_size: Vec2,
    rocket_size: Vec2,
}

impl App {
    pub async fn new() -> Self {
        Self {
            state: State::Init,
            position: vec2(screen_width() / 2.0 - 50.0, screen_height() - 50.0),
            velocity: vec2(0.0, 0.0),
            rocket_texture: load_texture("assets/rocket.png").await.unwrap(),
            screen_size: vec2(screen_width(), screen_height()),
            rocket_size: vec2(100.0, 200.0),
        }
    }

    pub async fn run(&mut self) -> ! {
        prevent_quit();
        clear_background(BLUE);

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

    async fn draw(&self) {
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

        next_frame().await;
    }
}

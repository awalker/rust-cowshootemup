use macroquad::input;

// Game_data.rs
use crate::{prelude::*, state::State};

#[derive(Default, Debug)]
pub struct GameData {
    pub world: World,
    pub fps: i32,
    pub time: f32,
    pub state: State,
    pub show_gizmos: bool,
    pub show_editor: bool,
}

impl GameData {
    fn update_game(&mut self, delta_time: f32) {
        self.time += delta_time;
        self.handle_common_input(delta_time);
        self.world.update(delta_time);
        self.fps = get_fps();
    }

    pub fn is_editor(&self) -> bool {
        self.show_editor
    }

    fn update_paused(&mut self, delta_time: f32) {
        self.handle_common_input(delta_time);
    }

    fn handle_common_input(&mut self, _delta_time: f32) {
        self.step();
        if input::is_key_pressed(KeyCode::C) || input::is_key_pressed(KeyCode::F10) {
            self.show_gizmos = !self.show_gizmos;
        }
        if input::is_key_pressed(KeyCode::E) || input::is_key_pressed(KeyCode::F11) {
            self.show_editor = !self.show_editor;
        }
        if input::is_key_pressed(KeyCode::Escape) {
            self.press_escape();
        }
        if input::is_key_pressed(KeyCode::Space) {
            info!("mq input space");
            self.press_space();
        }
    }

    fn step(&mut self) {
        if matches!(self.state, State::Step | State::Paused) {
            if input::is_key_down(KeyCode::S) {
                self.state = State::Step;
            }
            if input::is_key_released(KeyCode::S) {
                self.state = State::Paused;
            }
            if input::is_key_pressed(KeyCode::G) {
                self.state = State::Playing
            }
        }
    }

    fn draw_game(&self) {
        clear_background(RED);

        self.world.draw();
    }

    fn draw_paused(&self) {
        draw_text("Paused", 34.0, 60.0, 10.0, WHITE);
    }

    fn draw_step(&self) {
        draw_text("Press s to Step", 20.0, 60.0, 10.0, WHITE);
    }

    fn press_escape(&mut self) {
        match self.state {
            State::Playing => self.state = State::Exit,
            State::Paused => self.state = State::Playing,
            State::Step => self.state = State::Exit,
            _ => {}
        }
    }

    fn press_space(&mut self) {
        if self.state.is_playing() {
            self.state = State::Paused;
        } else {
            self.state = State::Playing;
        }
    }
}

impl Updateable for GameData {
    fn update(&mut self, delta_time: f32) {
        match self.state {
            State::Init => self.state = State::Paused,
            State::Playing => self.update_game(delta_time),
            State::Paused => self.update_paused(delta_time),
            State::Step => {
                self.update_game(delta_time);
            }
            State::Exit => {}
        }
    }
}

impl Drawable for GameData {
    fn draw(&self) {
        match self.state {
            State::Init => {}
            State::Playing => self.draw_game(),
            State::Paused => {
                self.draw_game();
                self.draw_paused();
            }
            State::Step => {
                self.draw_game();
                self.draw_step();
            }
            State::Exit => self.draw_game(),
        }
    }

    fn draw_gizmos(&self) {
        if self.show_gizmos {
            self.world.draw_gizmos();
        }
    }
}

/* pub trait GameData {
} */

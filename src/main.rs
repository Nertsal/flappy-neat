use geng::prelude::*;

mod model;
mod renderer;

use model::*;
use renderer::*;

struct State {
    geng: Geng,
    model: Model,
    renderer: Renderer,
    model_delta_time: f32,
}

impl State {
    fn new(geng: &Geng) -> Self {
        let rules: Rules = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("rules.json").unwrap(),
        ))
        .unwrap();
        let config: neat::NeatConfig = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("config.json").unwrap(),
        ))
        .unwrap();
        Self {
            geng: geng.clone(),
            renderer: Renderer::new(geng),
            model: Model::new(rules, config),
            model_delta_time: 0.1,
        }
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        self.model.update(self.model_delta_time);
        self.renderer.update(delta_time as f32);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let minimal = self.geng.window().is_key_pressed(geng::Key::S);
        self.renderer.draw(&self.model, minimal, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.model.handle_event(&event);
        self.renderer.handle_event(&event);
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Flappy NEAT".to_string(),
        vsync: false,
        ..Default::default()
    });
    let state = State::new(&geng);
    geng::run(&geng, state);
}

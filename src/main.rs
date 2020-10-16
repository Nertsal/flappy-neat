use geng::prelude::*;

mod model;
mod renderer;

use model::*;
use renderer::*;

struct State {
    model: Model,
    renderer: Renderer,
}

impl State {
    fn new(geng: &Rc<Geng>) -> Self {
        let rules: Rules = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("rules.json").unwrap(),
        ))
        .unwrap();
        let config: neat::NeatConfig = serde_json::from_reader(std::io::BufReader::new(
            std::fs::File::open("config.json").unwrap(),
        ))
        .unwrap();
        Self {
            renderer: Renderer::new(geng, config.max_clients),
            model: Model::new(rules, config),
        }
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        self.model.update(delta_time as f32);
        self.renderer.update(delta_time as f32);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.renderer.draw(framebuffer, &self.model);
    }
    fn handle_event(&mut self, event: geng::Event) {
        self.model.handle_event(&event);
        self.renderer.handle_event(&event);
    }
}

fn main() {
    let geng = Rc::new(Geng::new(default()));
    let state = State::new(&geng);
    geng::run(geng, state);
}

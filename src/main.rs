use geng::prelude::ugli::Framebuffer;
use geng::prelude::*;
use neat::*;

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
        let rules = Rules {
            gravity: vec2(0.0, -9.8),
            bird_radius: 20.0,
        };
        Self {
            model: Model::new(geng, rules),
            renderer: Renderer::new(geng),
        }
    }
}

impl geng::State for State {
    fn update(&mut self, delta_time: f64) {
        self.model.update(delta_time as f32);
        self.renderer.update(delta_time as f32);
    }
    fn draw(&mut self, framebuffer: &mut Framebuffer) {
        self.renderer.draw(framebuffer, &self.model);
    }
    fn handle_event(&mut self, event: geng::Event) {
        self.model.handle_event(event);
    }
}

fn main() {
    let geng = Rc::new(Geng::new(default()));
    let state = State::new(&geng);
    geng::run(geng, state);
}

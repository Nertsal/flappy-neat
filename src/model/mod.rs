use super::*;
use neat::structure::Client;
use neat::{Neat, NeatConfig};

pub struct Rules {
    pub gravity: Vec2<f32>,
    pub bird_radius: f32,
    pub jump_speed: f32,
}

pub struct Model {
    geng: Rc<Geng>,
    neat: Rc<RefCell<Neat>>,
    rules: Rules,
    pub player: Bird,
    pub clients: HashMap<usize, Bird>,
}

pub struct Bird {
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub speed: Vec2<f32>,
    pub controller: Controller,
}

pub enum Controller {
    Player,
    Client(Rc<Client>),
}

impl Model {
    pub fn new(geng: &Rc<Geng>, rules: Rules) -> Self {
        let neat_config = NeatConfig {
            input_size: 1,
            output_size: 1,
            max_clients: 5,
            disjoint: 1.0,
            excess: 1.0,
            weight_diff: 1.0,
        };
        let neat = Neat::new(neat_config);
        let mut clients = HashMap::with_capacity(neat.borrow().clients.len());
        for (id, client) in neat.borrow().clients.iter().enumerate() {
            let bird = Bird {
                pos: vec2(0.0, 0.0),
                radius: rules.bird_radius,
                speed: vec2(1.0, 0.0),
                controller: Controller::Client(client.clone()),
            };
            clients.insert(id, bird);
        }
        let player = Bird {
            pos: vec2(0.0, 0.0),
            radius: rules.bird_radius,
            speed: vec2(1.0, 0.0),
            controller: Controller::Player,
        };
        Self {
            geng: geng.clone(),
            neat,
            rules,
            player,
            clients,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let gravity = self.rules.gravity;
        self.player.speed += gravity * delta_time;
        self.player.pos += self.player.speed * delta_time;
        for (_, bird) in &mut self.clients {
            bird.speed += gravity * delta_time;
            bird.pos += bird.speed * delta_time;

            match &bird.controller {
                Controller::Client(client) => {
                    let output = client.calculate([0.0].to_vec());
                    if *output.first().unwrap() >= 0.5 {
                        bird.speed.y = self.rules.jump_speed;
                    }
                }
                _ => (),
            }
        }
    }
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::W | geng::Key::Up | geng::Key::Space => {
                    self.player.speed.y = self.rules.jump_speed;
                }
                _ => (),
            },
            _ => (),
        }
    }
}

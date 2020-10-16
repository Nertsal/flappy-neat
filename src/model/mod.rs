use super::*;
use neat::structure::Client;
use neat::{Neat, NeatConfig};

pub struct Rules {
    pub gravity: Vec2<f32>,
    pub bird_radius: f32,
    pub jump_speed: f32,
}

pub struct Model {
    neat: Rc<RefCell<Neat>>,
    rules: Rules,
    pub player: Bird,
    pub clients: HashMap<usize, Bird>,
}

pub struct Bird {
    pub alive: bool,
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub speed: Vec2<f32>,
    pub controller: Controller,
}

pub enum Controller {
    Player,
    Client(Rc<RefCell<Client>>),
}

impl Model {
    pub fn new(rules: Rules, config: NeatConfig) -> Self {
        let neat = Neat::new(config);
        let mut clients = HashMap::with_capacity(neat.borrow().clients.len());
        for (id, client) in neat.borrow().clients.iter().enumerate() {
            let bird = Bird {
                alive: true,
                pos: vec2(0.0, 0.0),
                radius: rules.bird_radius,
                speed: vec2(1.0, 0.0),
                controller: Controller::Client(client.clone()),
            };
            clients.insert(id, bird);
        }
        let player = Bird {
            alive: true,
            pos: vec2(0.0, 0.0),
            radius: rules.bird_radius,
            speed: vec2(1.0, 0.0),
            controller: Controller::Player,
        };
        Self {
            neat,
            rules,
            player,
            clients,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let gravity = self.rules.gravity;

        if self.player.alive {
            self.player.speed += gravity * delta_time;
            self.player.pos += self.player.speed * delta_time;
            Self::check_bird(&mut self.player);
        }

        for (_, bird) in &mut self.clients {
            if bird.alive {
                bird.speed += gravity * delta_time;
                bird.pos += bird.speed * delta_time;

                match &bird.controller {
                    Controller::Client(client) => {
                        let output = client.borrow().calculate(Self::read_bird(bird));
                        if *output.first().unwrap() >= 0.5 {
                            bird.speed.y = self.rules.jump_speed;
                        }
                    }
                    _ => (),
                }

                Self::check_bird(bird);
            }
        }
    }
    pub fn handle_event(&mut self, event: &geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::W | geng::Key::Up | geng::Key::Space => {
                    if self.player.alive {
                        self.player.speed.y = self.rules.jump_speed;
                    }
                }
                geng::Key::R => {
                    self.reset();
                    self.neat.borrow_mut().evolve();
                }
                _ => (),
            },
            _ => (),
        }
    }
    fn reset(&mut self) {
        self.player.alive = true;
        self.player.pos = vec2(0.0, 0.0);
        self.player.speed = vec2(0.0, 0.0);
        for (_, bird) in &mut self.clients {
            bird.alive = true;
            bird.pos = vec2(0.0, 0.0);
            bird.speed = vec2(0.0, 0.0);
        }
    }
    fn read_bird(bird: &Bird) -> Vec<f32> {
        vec![bird.pos.y]
    }
    fn check_bird(bird: &mut Bird) {
        if bird.pos.y < -20.0 || bird.pos.y > 20.0 {
            bird.alive = false;
        }
    }
}

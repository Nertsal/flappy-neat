use super::*;
use neat::structure::Client;
use neat::{Neat, NeatConfig};

#[derive(Serialize, Deserialize)]
pub struct Rules {
    pub gravity: Vec2<f32>,
    pub bird_radius: f32,
    pub jump_speed: f32,
    pub obstacle_size: Vec2<f32>,
    pub obstacle_dist: f32,
}

pub struct Model {
    neat: Rc<RefCell<Neat>>,
    rules: Rules,
    pub player: Bird,
    pub clients: HashMap<usize, Bird>,
    pub obstacles: Vec<Obstacle>,
    next_obstacle: f32,
}

pub struct Bird {
    pub alive: bool,
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub speed: Vec2<f32>,
    pub controller: Controller,
}

impl Bird {
    fn check_pos(&mut self) {
        if self.pos.y < -20.0 || self.pos.y > 20.0 {
            self.alive = false;
        }
    }
    fn collide(&mut self, obstacle: &Obstacle) {
        let dist_left = obstacle.pos.x - obstacle.size.x / 2.0 - self.pos.x;
        let dist_right = self.pos.x - obstacle.pos.x - obstacle.size.x / 2.0;
        let dist_x = if dist_left.abs() < dist_right.abs() {
            dist_left
        } else {
            dist_right
        };
        if dist_x < self.radius {
            let dist_up = obstacle.pos.y + obstacle.size.y / 2.0 - self.pos.y;
            let dist_down = self.pos.y - obstacle.pos.y + obstacle.size.y / 2.0;

            if dist_up <= 0.0 || dist_down <= 0.0 {
                self.alive = false;
                return;
            }

            let dist_y = if dist_up.abs() < dist_down.abs() {
                dist_up
            } else {
                dist_down
            };
            if dist_x <= 0.0 {
                if dist_y < self.radius {
                    self.alive = false;
                }
            } else {
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if dist < self.radius {
                    self.alive = false;
                }
            }
        }
    }
}

pub enum Controller {
    Player,
    Client(Rc<RefCell<Client>>),
}

pub struct Obstacle {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
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
                speed: vec2(5.0, 0.0),
                controller: Controller::Client(client.clone()),
            };
            clients.insert(id, bird);
        }
        let player = Bird {
            alive: true,
            pos: vec2(0.0, 0.0),
            radius: rules.bird_radius,
            speed: vec2(5.0, 0.0),
            controller: Controller::Player,
        };
        Self {
            neat,
            rules,
            player,
            clients,
            obstacles: Vec::new(),
            next_obstacle: 20.0,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let gravity = self.rules.gravity;

        let mut max_x: f32 = 0.0;

        if self.player.alive {
            self.player.speed += gravity * delta_time;
            self.player.pos += self.player.speed * delta_time;
            max_x = max_x.max(self.player.pos.x);
            self.player.check_pos();
            for obstacle in &self.obstacles {
                self.player.collide(obstacle);
            }
        }

        for (_, bird) in &mut self.clients {
            if bird.alive {
                bird.speed += gravity * delta_time;
                bird.pos += bird.speed * delta_time;
                max_x = max_x.max(bird.pos.x);

                match &bird.controller {
                    Controller::Client(client) => {
                        let output = client
                            .borrow()
                            .calculate(Self::read_bird(bird, &self.obstacles));
                        if *output.first().unwrap() >= 0.5 {
                            bird.speed.y = self.rules.jump_speed;
                        }
                    }
                    _ => (),
                }

                bird.check_pos();
                for obstacle in &self.obstacles {
                    bird.collide(obstacle);
                }
            }
        }

        if max_x >= self.next_obstacle - self.rules.obstacle_dist {
            self.spawn_obstacle();
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
        self.player.speed.y = 0.0;
        for (_, bird) in &mut self.clients {
            if let Controller::Client(client) = &bird.controller {
                client.borrow_mut().score = bird.pos.x;
            }

            bird.alive = true;
            bird.pos = vec2(0.0, 0.0);
            bird.speed.y = 0.0;
        }
        self.obstacles.clear();
        self.next_obstacle = self.rules.obstacle_dist;
    }
    fn read_bird(bird: &Bird, obstacles: &Vec<Obstacle>) -> Vec<f32> {
        let mut input = Vec::with_capacity(4);
        input.push(bird.pos.y / 40.0 + 0.5);
        for obstacle in obstacles {
            let next = obstacle.pos.x + obstacle.size.x / 2.0;
            if bird.pos.x <= next {
                input.push((next - bird.pos.x) / 50.0);
                input.push((bird.pos.y - obstacle.pos.y - obstacle.size.y / 2.0) / 40.0 + 0.5);
                input.push((bird.pos.y - obstacle.pos.y + obstacle.size.y / 2.0) / 40.0 + 0.5);
                return input;
            }
        }
        for _ in 0..3 {
            input.push(0.0);
        }
        input
    }
    fn spawn_obstacle(&mut self) {
        let mut random = rand::thread_rng();
        let max_height = 20.0 - self.rules.obstacle_size.y;
        let height = random.gen_range(-max_height, max_height);
        let obstacle = Obstacle {
            pos: vec2(self.next_obstacle, height),
            size: self.rules.obstacle_size,
        };
        self.next_obstacle += self.rules.obstacle_dist;
        self.obstacles.push(obstacle);
    }
}

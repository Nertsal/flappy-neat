use super::*;

pub struct Rules {
    pub gravity: Vec2<f32>,
    pub bird_radius: f32,
}

pub struct Model {
    geng: Rc<Geng>,
    neat: Rc<RefCell<Neat>>,
    rules: Rules,
    pub birds: HashMap<usize, Bird>,
}

pub struct Bird {
    pub pos: Vec2<f32>,
    pub radius: f32,
    pub speed: Vec2<f32>,
    pub client: Rc<Client>,
}

impl Model {
    pub fn new(geng: &Rc<Geng>, rules: Rules) -> Self {
        let neat_config = NeatConfig {
            input_size: 5,
            output_size: 1,
            max_clients: 5,
            disjoint: 1.0,
            excess: 1.0,
            weight_diff: 1.0,
        };
        let neat = Neat::new(neat_config);
        let mut birds = HashMap::with_capacity(neat.borrow().clients.len());
        for (id, client) in neat.borrow().clients.iter().enumerate() {
            let bird = Bird {
                pos: vec2(0.0, 0.0),
                radius: rules.bird_radius,
                speed: vec2(0.0, 0.0),
                client: client.clone(),
            };
            birds.insert(id, bird);
        }
        Self {
            geng: geng.clone(),
            neat,
            rules,
            birds,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        let gravity = self.rules.gravity;
        for (id, bird) in &mut self.birds {
            bird.speed += gravity * delta_time;
            bird.pos += bird.speed * delta_time;
        }
    }
    pub fn handle_event(&mut self, event: geng::Event) {}
}

use super::*;

pub struct Renderer {
    geng: Rc<Geng>,
    scale: f32,
    focused: Option<usize>,
    max_focus: usize,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>, clients_count: usize) -> Self {
        Self {
            geng: geng.clone(),
            scale: 20.0,
            focused: None,
            max_focus: clients_count - 1,
        }
    }
    pub fn update(&mut self, _delta_time: f32) {}
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let mut offset = vec2(model.player.pos.x, 0.0);
        if let Some(focused) = self.focused {
            if let Some(bird) = model.clients.get(&focused) {
                offset.x = bird.pos.x;
                if let Controller::Client(client) = &bird.controller {
                    self.draw_brain(framebuffer, &client.borrow().genome);
                }
            }
        }

        let screen_center = framebuffer.size().map(|x| (x as f32) / 2.0);

        for (_, bird) in &model.clients {
            let color = if bird.alive { Color::RED } else { Color::GRAY };
            self.geng.draw_2d().circle(
                framebuffer,
                (bird.pos - offset) * self.scale + screen_center,
                bird.radius * self.scale,
                color,
            );
        }

        let color = if model.player.alive {
            Color::BLUE
        } else {
            Color::GRAY
        };
        self.geng.draw_2d().circle(
            framebuffer,
            (model.player.pos - offset) * self.scale + screen_center,
            model.player.radius * self.scale,
            color,
        );

        for obstacle in &model.obstacles {
            self.geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(
                    (obstacle.pos - obstacle.size / 2.0 - vec2(0.0, 30.0) - offset) * self.scale
                        + screen_center,
                    (obstacle.pos + vec2(obstacle.size.x, -obstacle.size.y) / 2.0 - offset)
                        * self.scale
                        + screen_center,
                ),
                Color::GRAY,
            );
            self.geng.draw_2d().quad(
                framebuffer,
                AABB::from_corners(
                    (obstacle.pos - vec2(obstacle.size.x, -obstacle.size.y) / 2.0
                        + vec2(0.0, 30.0)
                        - offset)
                        * self.scale
                        + screen_center,
                    (obstacle.pos + obstacle.size / 2.0 - offset) * self.scale + screen_center,
                ),
                Color::GRAY,
            );
        }
    }
    fn draw_brain(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        genome: &neat::structure::Genome,
    ) {
        let brain_scale = vec2(100.0, 500.0);
        let offset = vec2(50.0, 50.0);
        for node in genome.nodes() {
            self.geng.draw_2d().circle(
                framebuffer,
                vec2(node.x * brain_scale.x, node.y * brain_scale.y) + offset,
                5.0,
                Color::RED,
            );
        }
        for connection in &genome.connections {
            let vertices = [
                vec2(
                    connection.node_from.x * brain_scale.x,
                    connection.node_from.y * brain_scale.y,
                ) + offset,
                vec2(
                    connection.node_to.x * brain_scale.x,
                    connection.node_to.y * brain_scale.y,
                ) + offset,
            ];
            self.geng.draw_2d().draw(
                framebuffer,
                &vertices,
                Color::GREEN,
                ugli::DrawMode::Lines { line_width: 2.0 },
            );
        }
    }
    pub fn handle_event(&mut self, event: &geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Right | geng::Key::D => {
                    self.focused = match self.focused {
                        Some(focus) => {
                            if focus >= self.max_focus - 1 {
                                None
                            } else {
                                Some(focus + 1)
                            }
                        }
                        None => Some(0),
                    }
                }
                geng::Key::Left | geng::Key::A => {
                    self.focused = match self.focused {
                        Some(focus) => {
                            if focus <= 0 {
                                None
                            } else {
                                Some(focus - 1)
                            }
                        }
                        None => Some(self.max_focus - 1),
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}

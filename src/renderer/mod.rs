use super::*;

pub struct Renderer {
    geng: Rc<Geng>,
    scale: f32,
    ui_controller: geng::ui::Controller,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
            geng: geng.clone(),
            scale: 20.0,
            ui_controller: geng::ui::Controller::new(),
        }
    }
    pub fn update(&mut self, _delta_time: f32) {}
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let offset = if let Some((_, bird)) = model.clients.iter().find(|(_, bird)| bird.alive) {
            self.draw_brain(framebuffer, model, bird);
            vec2(bird.pos.x, 0.0)
        } else {
            vec2(model.player.pos.x, 0.0)
        };

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
    fn draw_brain(&mut self, framebuffer: &mut ugli::Framebuffer, model: &Model, bird: &Bird) {
        use geng::ui::*;
        let mut widgets: Vec<Box<dyn Widget>> = Vec::new();

        if let model::Controller::Client(client) = &bird.controller {
            let nodes_output = client
                .borrow()
                .genome
                .calculate_debug(bird.read(&model.obstacles));

            let brain_scale = vec2(300.0, 500.0);
            let offset = vec2(50.0, 50.0);
            for node in &client.borrow().genome.nodes() {
                let position = vec2(node.x * brain_scale.x, node.y * brain_scale.y) + offset;
                self.geng
                    .draw_2d()
                    .circle(framebuffer, position, 5.0, Color::RED);
                let value = nodes_output.get(&node.gene).unwrap();
                widgets.push(Box::new(
                    geng::ui::Text::new(
                        format!("{:.2}", value),
                        self.geng.default_font(),
                        0.015,
                        Color::WHITE,
                    )
                    .padding_left(position.x as f64)
                    .padding_bottom(position.y as f64),
                ));
            }
            for connection in &client.borrow().genome.connections {
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
                let position = (vertices[0] + vertices[1]) / 2.0;
                widgets.push(Box::new(
                    geng::ui::Text::new(
                        format!("{:.2}", connection.weight),
                        self.geng.default_font(),
                        0.015,
                        Color::GRAY,
                    )
                    .padding_left(position.x as f64)
                    .padding_bottom(position.y as f64),
                ));
            }
        }

        self.ui_controller
            .draw(&mut geng::ui::stack(widgets), framebuffer);
    }
    pub fn handle_event(&mut self, event: &geng::Event) {
        match event {
            _ => (),
        }
    }
}

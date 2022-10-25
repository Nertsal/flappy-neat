use super::*;

pub struct Renderer {
    geng: Geng,
    scale: f32,
}

impl Renderer {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            scale: 20.0,
        }
    }

    pub fn update(&mut self, _delta_time: f32) {}

    pub fn draw(&mut self, model: &Model, minimal: bool, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let camera = &geng::PixelPerfectCamera;

        let screen = AABB::ZERO.extend_positive(framebuffer.size().map(|x| x as f32));

        if !minimal {
            let offset = if let Some((_, bird)) = model.clients.iter().find(|(_, bird)| bird.alive)
            {
                self.draw_brain(bird, model, framebuffer);
                vec2(bird.pos.x, 0.0)
            } else {
                vec2(model.player.pos.x, 0.0)
            };

            for bird in model.clients.values() {
                let color = if bird.alive { Color::RED } else { Color::GRAY };
                self.geng.draw_2d(
                    framebuffer,
                    camera,
                    &draw_2d::Ellipse::circle(
                        (bird.pos - offset) * self.scale + screen.center(),
                        bird.radius * self.scale,
                        color,
                    ),
                );
            }

            let color = if model.player.alive {
                Color::BLUE
            } else {
                Color::GRAY
            };
            self.geng.draw_2d(
                framebuffer,
                camera,
                &draw_2d::Ellipse::circle(
                    (model.player.pos - offset) * self.scale + screen.center(),
                    model.player.radius * self.scale,
                    color,
                ),
            );

            for obstacle in &model.obstacles {
                self.geng.draw_2d(
                    framebuffer,
                    camera,
                    &draw_2d::Quad::new(
                        AABB::from_corners(
                            (obstacle.pos - obstacle.size / 2.0 - vec2(0.0, 30.0) - offset)
                                * self.scale
                                + screen.center(),
                            (obstacle.pos + vec2(obstacle.size.x, -obstacle.size.y) / 2.0 - offset)
                                * self.scale
                                + screen.center(),
                        ),
                        Color::GRAY,
                    ),
                );
                self.geng.draw_2d(
                    framebuffer,
                    camera,
                    &draw_2d::Quad::new(
                        AABB::from_corners(
                            (obstacle.pos - vec2(obstacle.size.x, -obstacle.size.y) / 2.0
                                + vec2(0.0, 30.0)
                                - offset)
                                * self.scale
                                + screen.center(),
                            (obstacle.pos + obstacle.size / 2.0 - offset) * self.scale
                                + screen.center(),
                        ),
                        Color::GRAY,
                    ),
                );
            }
        }

        // Generation number
        let text = draw_2d::Text::unit(
            &**self.geng.default_font(),
            format!("Generation {}", model.generation),
            Color::WHITE,
        )
        .scale_uniform(10.0)
        .translate(screen.top_left() + vec2(100.0, -30.0));
        self.geng.draw_2d(framebuffer, camera, &text);

        // Max score
        let text = draw_2d::Text::unit(
            &**self.geng.default_font(),
            format!("Best score: {}", model.max_score),
            Color::WHITE,
        )
        .scale_uniform(10.0)
        .translate(screen.top_left() + vec2(100.0, -60.0));
        self.geng.draw_2d(framebuffer, camera, &text);
    }

    fn draw_brain(&mut self, bird: &Bird, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let camera = &geng::PixelPerfectCamera;

        if let model::Controller::Client(client) = &bird.controller {
            let brain_scale = vec2(300.0, 500.0);
            let offset = vec2(50.0, 50.0);
            let client = model.neat.clients.get(client).unwrap();
            for node in &client.genome.nodes() {
                let position = vec2(node.x * brain_scale.x, node.y * brain_scale.y) + offset;
                self.geng.draw_2d(
                    framebuffer,
                    camera,
                    &draw_2d::Ellipse::circle(position, 5.0, Color::RED),
                );
            }
            for connection in &client.genome.connections {
                let from = vec2(
                    connection.node_from.x * brain_scale.x,
                    connection.node_from.y * brain_scale.y,
                ) + offset;
                let to = vec2(
                    connection.node_to.x * brain_scale.x,
                    connection.node_to.y * brain_scale.y,
                ) + offset;
                self.geng.draw_2d(
                    framebuffer,
                    camera,
                    &draw_2d::Segment::new(Segment::new(from, to), 2.0, Color::GREEN),
                );
            }
        }
    }

    pub fn handle_event(&mut self, event: &geng::Event) {}
}

use super::*;

pub struct Renderer {
    geng: Rc<Geng>,
    scale: f32,
    focused: Option<usize>,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self {
            geng: geng.clone(),
            scale: 20.0,
            focused: Some(1),
        }
    }
    pub fn update(&mut self, delta_time: f32) {}
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let mut offset = vec2(model.player.pos.x, 0.0);
        if let Some(focused) = self.focused {
            if let Some(bird) = model.clients.get(&focused) {
                offset.x = bird.pos.x;
                if let Controller::Client(client) = &bird.controller {
                    self.draw_brain(framebuffer, client);
                }
            }
        }

        let screen_center = framebuffer.size().map(|x| (x as f32) / 2.0);

        self.geng.draw_2d().circle(
            framebuffer,
            (model.player.pos - offset) * self.scale + screen_center,
            model.player.radius * self.scale,
            Color::BLUE,
        );
        for (_, bird) in &model.clients {
            self.geng.draw_2d().circle(
                framebuffer,
                (bird.pos - offset) * self.scale + screen_center,
                bird.radius * self.scale,
                Color::RED,
            );
        }
    }
    fn draw_brain(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        client: &neat::structure::Client,
    ) {
        let brain_scale = 100.0;
        let offset = vec2(50.0, 50.0);
        for node in &client.genome.nodes {
            self.geng.draw_2d().circle(
                framebuffer,
                vec2(node.x, node.y) * brain_scale + offset,
                5.0,
                Color::RED,
            );
        }
        for connection in &client.genome.connections {
            let vertices = [
                vec2(connection.node_from.x, connection.node_from.y) * brain_scale + offset,
                vec2(connection.node_to.x, connection.node_to.y) * brain_scale + offset,
            ];
            self.geng
                .draw_2d()
                .draw(framebuffer, &vertices, Color::GREEN, ugli::DrawMode::Points);
        }
    }
}

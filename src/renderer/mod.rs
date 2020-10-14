use super::*;

pub struct Renderer {
    geng: Rc<Geng>,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self { geng: geng.clone() }
    }
    pub fn update(&mut self, delta_time: f32) {}
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let center_coords = framebuffer.size().map(|x| (x as f32) / 2.0);

        for (_, bird) in &model.birds {
            self.geng.draw_2d().circle(
                framebuffer,
                bird.pos + center_coords,
                bird.radius,
                Color::RED,
            );
        }
    }
}

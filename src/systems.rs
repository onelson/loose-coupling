use specs;
use rand;
use std::sync::Arc;
use radiant_rs::{Color, Layer};
use game;
use components;

#[derive(Clone)]
pub struct Spinner {
    factor: f32
}


impl specs::System<game::Delta> for Spinner
{
    fn run(&mut self, arg: specs::RunArg, _: game::Delta) {
        use specs::Join;

        let mut body = arg.fetch(|w| {
            w.write::<components::Body>()
        });

        // update entities
        for b in (&mut body).iter() {
            b.rotation += self.factor * rand::random::<f32>();
        }
    }
}


#[derive(Clone)]
pub struct Renderer {
    pub layer: Arc<Layer>
}


impl specs::System<game::Delta> for Renderer
{
    fn run(&mut self, arg: specs::RunArg, _: game::Delta) {
        use specs::Join;
        let (body, sprited) = arg.fetch(|w| {
            (w.read::<components::Body>(), w.read::<components::Sprited>())
        });

        // update entities
        for (b, s) in (&body, &sprited).iter() {
            let frame_id = 0;
            s.sprite.draw_transformed(self.layer.clone(), frame_id, b.x, b.y, Color::transparent(), b.rotation, b.scale_x, b.scale_y);
        }
    }
}

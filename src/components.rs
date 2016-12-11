use radiant_rs::Sprite;
use specs;
use std::sync::Arc;


#[derive(Default, Clone, Debug)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}

#[derive(Default, Clone, Debug)]
pub struct Sprited<'game> {
    sprite: Arc<Sprite<'game>>
}

impl<'game> specs::Component for Sprited<'game> {
    type Storage = specs::VecStorage<Sprited<'game>>;
}

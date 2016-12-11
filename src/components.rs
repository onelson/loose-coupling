use radiant_rs::Sprite;
use specs;
use std::sync::Arc;


#[derive(Clone, Debug)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}

#[derive(Clone, Debug)]
pub struct Sprited {
    id: u8
}

impl specs::Component for Sprited {
    type Storage = specs::VecStorage<Sprited>;
}

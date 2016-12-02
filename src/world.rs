use specs;
use sys;
use uuid;

#[derive(Default, Clone, Debug)]
pub struct Body {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
}

impl specs::Component for Body {
    type Storage = specs::VecStorage<Body>;
}

#[derive(Debug)]
pub struct Sprited {
    id: uuid::Uuid
}

impl specs::Component for Sprited {
    type Storage = specs::VecStorage<Sprited>;
}

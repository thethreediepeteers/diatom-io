use std::collections::HashMap;

#[derive(Clone)]
pub struct XY {
    pub x: f32,
    pub y: f32,
}

impl XY {
    pub fn add(&mut self, vec: &Self) {
        self.x += vec.x;
        self.y += vec.y;
    }
}

pub struct Entity {
    pub id: i32,
    pub pos: XY,
    pub vel: XY,
    pub size: f32,
    pub keys: HashMap<char, bool>,
}

impl Entity {
    pub fn clone(&self) -> Self {
        Self {
            id: self.id,
            pos: self.pos.clone(),
            vel: self.vel.clone(),
            size: self.size,
            keys: HashMap::new(),
        }
    }
}

use crate::util::lerp;

pub struct XY {
    pub x: f32,
    pub y: f32,
}

pub struct Entity {
    pub id: i32,
    pub pos: XY,
    server_pos: XY,
    pub size: f32,
}

impl Entity {
    pub fn new(id: i32, x: f32, y: f32, size: f32) -> Self {
        Self {
            id,
            pos: XY { x, y },
            server_pos: XY { x, y },
            size,
        }
    }

    pub fn set_predict(&mut self, x: f32, y: f32) {
        self.server_pos.x = x;
        self.server_pos.y = y;
    }

    pub fn predict(&mut self) {
        self.pos.x = lerp(self.pos.x, self.server_pos.x);
        self.pos.y = lerp(self.pos.y, self.server_pos.y);
    }
}

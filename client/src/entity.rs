use crate::util::lerp;

pub struct XY {
    pub x: f32,
    pub y: f32,
}

pub struct Entity {
    pub id: i32,
    pub pos: XY,
    pub size: f32,
    server_data: (f32, f32, f32),
}

impl Entity {
    pub fn new(id: i32, x: f32, y: f32, size: f32) -> Self {
        Self {
            id,
            pos: XY { x, y },
            size,
            server_data: (x, y, size),
        }
    }

    pub fn set_predict(&mut self, x: f32, y: f32, size: f32) {
        self.server_data = (x, y, size);
    }

    pub fn predict(&mut self) {
        self.pos.x = lerp(self.pos.x, self.server_data.0);
        self.pos.y = lerp(self.pos.y, self.server_data.1);
        self.size = lerp(self.size, self.server_data.2);
    }
}

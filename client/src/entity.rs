use crate::util::lerp;

pub struct XY {
    pub x: f64,
    pub y: f64,
}

pub struct Entity {
    pub id: i32,
    pub pos: XY,
    pub size: f64,
    server_data: (f64, f64, f64),
}

impl Entity {
    pub fn new(id: i32, x: f64, y: f64, size: f64) -> Self {
        Self {
            id,
            pos: XY { x, y },
            size,
            server_data: (x, y, size),
        }
    }

    pub fn set_predict(&mut self, x: f64, y: f64, size: f64) {
        self.server_data = (x, y, size);
    }

    pub fn predict(&mut self) {
        self.pos.x = lerp(self.pos.x, self.server_data.0);
        self.pos.y = lerp(self.pos.y, self.server_data.1);
        self.size = lerp(self.size, self.server_data.2);
    }
}

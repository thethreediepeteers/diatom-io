use crate::util::lerp;

pub struct XY {
    pub x: f64,
    pub y: f64,
}

pub struct Entity {
    pub id: i32,
    pub mockup_id: i32,
    pub pos: XY,
    pub size: f64,
    pub angle: f64,
    server_data: (f64, f64, f64, f64),
}

impl Entity {
    pub fn new(id: i32, x: f64, y: f64, size: f64, mockup_id: i32) -> Self {
        Self {
            id,
            mockup_id,
            pos: XY { x, y },
            size,
            angle: 0.0,
            server_data: (x, y, size, 0.0),
        }
    }

    pub fn set_predict(&mut self, x: f64, y: f64, size: f64, angle: f64) {
        self.server_data = (x, y, size, angle);
    }

    pub fn predict(&mut self) {
        self.pos.x = lerp(self.pos.x, self.server_data.0, 0.05);
        self.pos.y = lerp(self.pos.y, self.server_data.1, 0.05);
        self.size = lerp(self.size, self.server_data.2, 0.1);
        self.angle = lerp(self.angle, self.server_data.3, 0.1);
    }
}

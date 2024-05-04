use crate::{game::get_game, util::{lerp, lerp_angle}};

pub struct XY {
    pub x: f64,
    pub y: f64
}

pub struct Entity {
    pub id: u16,
    pub mockup_id: u16,
    pub pos: XY,
    pub size: f64,
    pub angle: f64,
    server_data: (f64, f64, f64, f64),
    pub is_player: bool
}

impl Entity {
    pub fn new(id: u16, x: f64, y: f64, size: f64, mockup_id: u16, is_player: bool) -> Self {
        Self {
            id,
            mockup_id,
            pos: XY { x, y },
            size,
            angle: 0.0,
            server_data: (x, y, size, 0.0),
            is_player
        }
    }

    pub fn set_predict(&mut self, x: f64, y: f64, size: f64, angle: f64) {
        self.server_data = (x, y, size, angle);
    }

    pub fn predict(&mut self) {
        self.pos.x = lerp(self.pos.x, self.server_data.0, 0.05);
        self.pos.y = lerp(self.pos.y, self.server_data.1, 0.05);
        self.size = lerp(self.size, self.server_data.2, 0.1);
        if self.is_player {
            let game =  get_game();
            self.angle = game.mouse_angle;
        } else {
            self.angle = lerp_angle(self.angle, self.server_data.3, 0.15);
        }
    }
}
use super::hashgrid::XY;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Entity {
    pub id: i32,
    pub pos: XY,
    pub vel: XY,
    pub size: f32,
    pub keys: HashMap<char, bool>,
}

impl Entity {
    pub fn update_pos(&mut self) {
        if self.keys[&'w'] {
            self.vel.y -= 1.0;
        } else if self.keys[&'s'] {
            self.vel.y += 1.0;
        }

        if self.keys[&'a'] {
            self.vel.x -= 1.0;
        } else if self.keys[&'d'] {
            self.vel.x += 1.0;
        }

        self.pos += self.vel;
        self.vel.x *= 0.8;
        self.vel.y *= 0.8;
    }

    pub fn stay_in_bounds(&mut self, width: f32, height: f32) {
        if self.pos.x < 0.0 {
            self.vel.x += -self.pos.x / 10.0;
        } else if self.pos.x > width {
            self.vel.x -= (self.pos.x - width) / 10.0;
        }
        if self.pos.y < 0.0 {
            self.vel.y += -self.pos.y / 10.0;
        } else if self.pos.y > height {
            self.vel.y -= (self.pos.y - height) / 10.0;
        }
    }
}

use super::{game::Game, rect::Rectangle};
use std::collections::HashMap;

pub trait Entity {
    fn id(&self) -> u16;
    fn mockup_id(&self) -> u16;
    fn bounds(&self) -> Rectangle;
    fn angle(&self) -> f64;
    fn kill(&mut self, game: &mut Game);
}

pub trait Movement {
    fn update_pos(&mut self);
    fn stay_in_bounds(&mut self, width: f64, height: f64);
}

#[derive(Clone)]

pub struct Player {
    index: u16,
    mockup_index: u16,
    bounds: Rectangle,
    pub vel: (f64, f64),
    pub keys: HashMap<char, bool>,
    pub angle: f64,
    pub shooting: bool,
}

impl Player {
    pub fn new(
        index: u16,
        mockup_index: u16,
        bounds: Rectangle,
        vel: (f64, f64),
        keys: HashMap<char, bool>,
        angle: f64,
        shooting: bool,
    ) -> Self {
        Self {
            index,
            mockup_index,
            bounds,
            vel,
            keys,
            angle,
            shooting,
        }
    }

    pub fn shoot(&self) {
        if self.shooting {
            println!("player {} is shooting", self.index);
        }
    }
}

impl Entity for Player {
    fn id(&self) -> u16 {
        self.index
    }
    fn mockup_id(&self) -> u16 {
        self.mockup_index
    }
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
    fn angle(&self) -> f64 {
        self.angle
    }
    fn kill(&mut self, game: &mut Game) {
        game.remove_player(self.index);
    }
}
impl Movement for Player {
    fn update_pos(&mut self) {
        let (vx, vy) = self.vel;
        let (x, y) = self.bounds.get_center();
        let size = self.bounds.get_width();
        self.bounds = Rectangle::center_rect(x + vx, y + vy, size, size);

        if self.keys[&'w'] {
            self.vel.1 -= 1.0;
        } else if self.keys[&'s'] {
            self.vel.1 += 1.0;
        }

        if self.keys[&'a'] {
            self.vel.0 -= 1.0;
        } else if self.keys[&'d'] {
            self.vel.0 += 1.0;
        }

        self.vel.0 *= 0.8;
        self.vel.1 *= 0.8;
    }

    fn stay_in_bounds(&mut self, width: f64, height: f64) {
        let (mut x, mut y) = self.bounds.get_center();
        let size = self.bounds.get_width();
        let half_size = size * 0.5;

        if x - half_size < 0.0 {
            x = half_size;
        } else if x + half_size > width {
            x = width - half_size;
        }

        if y - half_size < 0.0 {
            y = half_size;
        } else if y + half_size > height {
            y = height - half_size;
        }

        self.bounds = Rectangle::center_rect(x, y, size, size);
    }
}

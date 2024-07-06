use crate::CLIENT_COUNTER;

use super::{game::Game, rect::Rectangle};
use std::collections::HashMap;

pub struct EntityState {
    pub id: u16,
    pub mockup_id: u16,
    pub bounds: Rectangle,
    pub angle: f64,
}

#[allow(dead_code)]
pub trait Entity: CloneEntity {
    fn id(&self) -> u16;
    fn mockup_id(&self) -> u16;
    fn bounds(&self) -> Rectangle;
    fn set_vel(&mut self, vx: f64, vy: f64);
    fn angle(&self) -> f64;
    fn remove(&mut self, game: &mut Game);
    fn get_state(&self) -> EntityState;
    fn update_pos(&mut self);
    fn stay_in_bounds(&mut self, width: f64, height: f64) {}
    fn set_bounds(&mut self, bounds: Rectangle) {}
    fn set_keys(&mut self, keys: &HashMap<char, bool>) {}
    fn is_player(&self) -> bool {
        false
    }
}

pub trait CloneEntity {
    fn clone_foo<'a>(&self) -> Box<dyn Entity>;
}

impl<T> CloneEntity for T
where
    T: Entity + Clone + 'static,
{
    fn clone_foo(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Self {
        self.clone_foo()
    }
}

#[derive(Clone)]

pub struct Player {
    pub index: u16,
    pub mockup_index: u16,
    pub bounds: Rectangle,
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

    pub fn shoot(&mut self, game: &mut Game) {
        if self.shooting {
            println!("player {} is shooting", self.index);
            game.id += 1;
            unsafe {
                CLIENT_COUNTER += 1;
            }
            game.spawn_entity(Box::new(Bullet::new(
                game.id,
                0,
                self.bounds.clone(),
                (0.0, 0.0),
                self.angle,
            )));
            self.shooting = false;
        }
    }

    pub fn input_key(&mut self, key: char, value: bool) {
        self.keys.insert(key, value);
    }

    pub fn input_angle(&mut self, angle: f64) {
        self.angle = angle
    }

    pub fn input_click(&mut self, click: bool) {
        self.shooting = click
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

    fn set_vel(&mut self, vx: f64, vy: f64) {
        self.vel = (vx, vy);
    }

    fn angle(&self) -> f64 {
        self.angle
    }

    fn remove(&mut self, game: &mut Game) {
        game.remove_entity_at_id(self.index);
    }

    fn get_state(&self) -> EntityState {
        EntityState {
            id: self.index,
            mockup_id: self.mockup_index,
            bounds: self.bounds,
            angle: self.angle,
        }
    }

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

    fn is_player(&self) -> bool {
        true
    }
    fn set_bounds(&mut self, bounds: Rectangle) {
        self.bounds = bounds
    }

    fn set_keys(&mut self, keys: &HashMap<char, bool>) {
        self.keys = keys.clone()
    }
}

#[derive(Clone)]
struct Bullet {
    id: u16,
    mockup_id: u16,
    bounds: Rectangle,
    vel: (f64, f64),
    angle: f64,
}

impl Bullet {
    pub fn new(id: u16, mockup_id: u16, bounds: Rectangle, vel: (f64, f64), angle: f64) -> Self {
        Self {
            id,
            mockup_id,
            bounds,
            vel,
            angle,
        }
    }
}

impl Entity for Bullet {
    fn id(&self) -> u16 {
        self.id
    }

    fn mockup_id(&self) -> u16 {
        self.mockup_id
    }

    fn bounds(&self) -> Rectangle {
        self.bounds
    }

    fn set_vel(&mut self, vx: f64, vy: f64) {
        self.vel = (vx, vy);
    }

    fn angle(&self) -> f64 {
        self.angle
    }

    fn remove(&mut self, game: &mut Game) {
        game.remove_entity_at_id(self.id);
    }

    fn get_state(&self) -> EntityState {
        EntityState {
            id: self.id,
            mockup_id: self.mockup_id,
            bounds: self.bounds,
            angle: self.angle,
        }
    }

    fn update_pos(&mut self) {
        let (vx, vy) = self.vel;
        let (x, y) = self.bounds.get_center();
        let size = self.bounds.get_width();
        self.bounds = Rectangle::center_rect(x + vx, y + vy, size, size);
    }
    fn set_bounds(&mut self, bounds: Rectangle) {
        self.bounds = bounds
    }
}

use rand::random;

use super::{
    entity::Entity,
    hashgrid::{Box, HashGrid, XY},
};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Map {
    pub width: f32,
    pub height: f32,
}

pub struct GameState {
    pub entities: Vec<Entity>,
    pub map: Map,
}

pub struct Game {
    players: HashMap<i32, Entity>,
    pub map: Map,
    grid: HashGrid,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            map: Map {
                width: 40.0 * 32.0,
                height: 40.0 * 32.0,
            },
            grid: HashGrid::new(32),
        }
    }

    pub fn add_player(&mut self, id: i32) {
        self.players.insert(
            id,
            Entity {
                id,
                pos: XY {
                    x: random::<f32>() * self.map.width,
                    y: random::<f32>() * self.map.height,
                },
                vel: XY { x: 0.0, y: 0.0 },
                size: 65.0,
                keys: HashMap::from([('w', false), ('a', false), ('s', false), ('d', false)]),
            },
        );
    }

    pub fn remove_player(&mut self, id: i32) {
        self.players.remove(&id);
    }

    pub fn set_input(&mut self, id: i32, key: u8, value: bool) {
        if let Some(entity) = self.players.get_mut(&id) {
            let char = match key {
                0 => 'w',
                1 => 'a',
                2 => 's',
                3 => 'd',
                _ => return,
            };

            entity.keys.insert(char, value);
        }
    }

    pub fn update(&mut self) {
        for entity in self.players.values_mut() {
            let mut bounding_box = Box::new(entity.id, entity.pos.x, entity.pos.y, entity.size);

            self.grid.remove(bounding_box);

            entity.update_pos();
            entity.stay_in_bounds(self.map.width, self.map.height);

            bounding_box = Box::new(entity.id, entity.pos.x, entity.pos.y, entity.size);

            self.grid.insert(bounding_box);

            for other in self.grid.query(bounding_box) {
                entity.vel.x -= (other.min.x +- entity.pos.x) * 0.01;
                entity.vel.y -= (other.min.y - entity.pos.y) * 0.01;
            }
        }
    }

    pub fn get_state(&self) -> GameState {
        let mut state = GameState {
            entities: Vec::new(),
            map: self.map,
        };
        for entity in self.players.values() {
            state.entities.push(entity.clone());
        }

        state
    }
}

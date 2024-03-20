use super::entity::{Entity, XY};
use std::collections::HashMap;

pub struct GameState {
    pub entities: Vec<Entity>,
}

pub struct Game {
    players: HashMap<i32, Entity>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, id: i32) {
        self.players.insert(
            id,
            Entity {
                id,
                pos: XY { x: 0.0, y: 0.0 },
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
                _ => 'w',
            };

            entity.keys.insert(char, value);
        }
    }

    pub fn update(&mut self) {
        for entity in self.players.values_mut() {
            if entity.keys[&'w'] {
                entity.vel.y -= 1.0;
            }
            if entity.keys[&'a'] {
                entity.vel.x -= 1.0;
            }
            if entity.keys[&'s'] {
                entity.vel.y += 1.0;
            }
            if entity.keys[&'d'] {
                entity.vel.x += 1.0;
            }

            entity.pos.add(&entity.vel);
            entity.vel.x *= 0.8;
            entity.vel.y *= 0.8;
        }
    }

    pub fn get_state(&self) -> GameState {
        let mut state = GameState {
            entities: Vec::new(),
        };
        for entity in self.players.values() {
            state.entities.push(entity.clone());
        }

        state
    }
}

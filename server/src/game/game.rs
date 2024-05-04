use super::{entity::Entity, rect::Rectangle};
use crate::game::up_search_quadtree::UpSearchQuadTree;
use rand::random;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Map {
    pub width: f64,
    pub height: f64,
}

pub struct GameState {
    pub entities: Vec<Entity>,
    pub map: Map
}

pub struct Game {
    players: HashMap<u16, Entity>,
    pub map: Map,
    quadtree: UpSearchQuadTree<u16, 8>
}

impl Game {
    pub fn new() -> Self {
        let map = Map {
            width: 40.0 * 32.0,
            height: 40.0 * 32.0,
        };
        Self {
            players: HashMap::new(),
            map,
            quadtree: UpSearchQuadTree::new(Rectangle::new(0.0, 0.0, map.width, map.height))
        }
    }

    pub fn add_player(&mut self, id: u16) {
        let size = 65.0;
        let bounds = Rectangle::center_rect(
            random::<f64>() * self.map.width,
            random::<f64>() * self.map.height,
            size,
            size,
        );
        let entity = Entity {
            id,
            mockup_id: 0,
            bounds: bounds.clone(),
            vel: (0.0, 0.0),
            keys: HashMap::from([('w', false), ('a', false), ('s', false), ('d', false)]),
            angle: 0.0
        };
        self.players.insert(id, entity);
        self.quadtree.insert(bounds, id);
    }

    pub fn remove_player(&mut self, id: u16) {
        if let Some(_entity) = self.players.remove(&id) {
            self.quadtree.remove(id);
        }
    }

    pub fn set_input(&mut self, id: u16, key: u8, value: bool) {
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

    pub fn set_mouse(&mut self, id: u16, rad: f64) {
        if let Some(entity) = self.players.get_mut(&id) {
            entity.angle = rad;
        }
    }

    pub fn update(&mut self) {
        let mut players_immut = HashMap::new();
        players_immut.clone_from(&self.players);

        let ids = self.players.keys().cloned().collect::<Vec<u16>>();

        for id in ids {
            let entity = self.players.get_mut(&id).unwrap();

            entity.update_pos();
            entity.stay_in_bounds(self.map.width, self.map.height);

            let mut candidates: Vec<u16> = Vec::new();

            self.quadtree.search(&entity.bounds, |id: u16| {
                candidates.push(id);
            });

            for candidate in candidates {
                if id == candidate {
                    continue;
                }

                let other_entity = players_immut.get_mut(&candidate);
                if let Some(other_entity) = other_entity {
                    let (x, y) = other_entity.bounds.get_center();
                    let (ex, ey) = entity.bounds.get_center();

                    entity.vel.0 -= (x - ex) / 100.0;
                    entity.vel.1 -= (y - ey) / 100.0;
                }
            }

            self.quadtree.update(entity.bounds, id);
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

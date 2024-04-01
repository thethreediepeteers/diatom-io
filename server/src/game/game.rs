use crate::game::up_search_quadtree::UpSearchQuadTree;
use rand::random;
use super::{
    entity::Entity,
    rect::Rectangle,
};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Map {
    pub width: f64,
    pub height: f64,
}

pub struct GameState {
    pub entities: Vec<Entity>,
    pub map: Map,
}

pub struct Game {
    players: HashMap<i32, Entity>,
    pub map: Map,
    quadtree: UpSearchQuadTree<i32, 8>,
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
            quadtree: UpSearchQuadTree::new(Rectangle::new(0.0, 0.0, map.width, map.height)),
        }
    }

    pub fn add_player(&mut self, id: i32) {
        let size = 65.0;
        let bounds = Rectangle::center_rect(
            random::<f64>() * self.map.width,
            random::<f64>() * self.map.height,
            size,
            size,
        );
        let entity = Entity {
            id,
            bounds: bounds.clone(), 
            vel: (0.0, 0.0),
            keys: HashMap::from([('w', false), ('a', false), ('s', false), ('d', false)]),
        };
        self.players.insert(id, entity);
        self.quadtree.insert(bounds, id);
    }

    pub fn remove_player(&mut self, id: i32) {
        if let Some(_entity) = self.players.remove(&id) {
            self.quadtree.remove(id);
        }
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
        let mut players_temp = std::mem::take(&mut self.players);
    
        for (id, entity) in players_temp.iter_mut() {
            entity.update_pos();
            entity.stay_in_bounds(self.map.width, self.map.height);

            self.quadtree.search(&entity.bounds, |other_id| {
                    if self.check_collision(*id, other_id) {
                        println!("collision");
                        let other_entity = self.players.get(&other_id);
                        // log entity.bounds.get_center();
                        // println!("{:?}", entity.bounds.get_center());
                        if let Some(other_entity) = other_entity {
                            let (x, y) = other_entity.bounds.get_center();
                            let (ex, ey) = entity.bounds.get_center();
                            entity.vel.0 -= (x - ex) * 0.01;
                            entity.vel.1 -= (y - ey) * 0.01;
                        }
                    }
                
            });

            self.quadtree.update(entity.bounds.clone(), *id);
        }
        self.players = players_temp;
    }

    fn check_collision(&self, entity_id: i32, candidate_id: i32) -> bool {
        if entity_id == candidate_id {
            return false;
        }
        true
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
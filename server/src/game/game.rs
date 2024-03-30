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
        if let Some(entity) = self.players.remove(&id) {
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
    
        let player_bounds: Vec<(i32, Rectangle)> = players_temp.iter().map(|(&id, entity)| (id, entity.bounds.clone())).collect();
    
        for (id, entity) in players_temp.iter_mut() {
            self.quadtree.remove(*id);
            entity.update_pos();
            entity.stay_in_bounds(self.map.width, self.map.height);
            self.quadtree.insert(entity.bounds.clone(), *id);
    
            for (other_id, other_bounds) in &player_bounds {
                if *id != *other_id {
                    let (x, y) = other_bounds.get_center();
                    let (ex, ey) = entity.bounds.get_center();
                    entity.vel.0 -= (x - ex) * 0.01;
                    entity.vel.1 -= (y - ey) * 0.01;
                }
            }
        }
    
        self.players = players_temp;
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
use super::{
    entity::{Entity, EntityState, Player},
    rect::Rectangle,
};
use crate::game::up_search_quadtree::UpSearchQuadTree;
use rand::random;
use std::collections::HashMap;

// todo: entity id to u32 (or usize) because 65565 is not so big

#[derive(Copy, Clone)]
pub struct Map {
    pub width: f64,
    pub height: f64,
}

pub struct GameState {
    pub entities: Vec<EntityState>,
    pub map: Map,
}

pub struct Game {
    pub id: u16,
    entities: HashMap<u16, Box<dyn Entity>>,
    players: HashMap<u16, Player>,
    pub map: Map,
    quadtree: UpSearchQuadTree<u16, 8>,
}

impl Game {
    pub fn new() -> Self {
        let map = Map {
            width: 40.0 * 32.0,
            height: 40.0 * 32.0,
        };
        Self {
            id: 0,
            entities: HashMap::new(),
            players: HashMap::new(),
            map,
            quadtree: UpSearchQuadTree::new(Rectangle::new(0.0, 0.0, map.width, map.height)),
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
        let entity = Box::new(Player::new(
            id,
            0,
            bounds.clone(),
            (0.0, 0.0),
            HashMap::from([('w', false), ('a', false), ('s', false), ('d', false)]),
            0.0,
            false,
        ));
        self.id += 1;
        self.players.insert(id, *entity.clone());
        self.spawn_entity(entity);
    }

    pub fn spawn_entity(&mut self, entity: Box<dyn Entity>) {
        let id = entity.id();
        self.quadtree.insert(entity.bounds(), id);
        self.entities.insert(id, entity);
    }

    pub fn remove_entity_at_id(&mut self, id: u16) {
        if let Some(_entity) = self.entities.remove(&id) {
            self.quadtree.remove(id);
        }
        self.players.remove(&id);
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
            entity.input_key(char, value);
            self.entities.get_mut(&id).unwrap().set_keys(&entity.keys);
        }
    }

    pub fn set_mouse(&mut self, id: u16, rad: f64) {
        if let Some(entity) = self.players.get_mut(&id) {
            entity.input_angle(rad);
        }
    }

    pub fn set_mouse_click(&mut self, id: u16, b: bool) {
        if let Some(entity) = self.players.clone().get_mut(&id) {
            entity.input_click(b);
            entity.shoot(self);
        }
    }

    pub fn update(&mut self) {
        let mut players_immut = HashMap::new();
        players_immut.clone_from(&self.entities);

        let ids = self.entities.keys().cloned().collect::<Vec<u16>>();

        for id in ids {
            let entity = self.entities.get_mut(&id).unwrap();

            entity.update_pos();
            entity.stay_in_bounds(self.map.width, self.map.height);

            let mut candidates: Vec<u16> = Vec::new();

            self.quadtree.search(&entity.bounds(), |id: u16| {
                candidates.push(id);
            });

            for candidate in candidates {
                if id == candidate {
                    continue;
                }

                let other_entity = players_immut.get_mut(&candidate);
                if let Some(other_entity) = other_entity {
                    let (x, y) = other_entity.bounds().get_center();
                    let (ex, ey) = entity.bounds().get_center();

                    entity.set_vel((x - ex) / 100.0, (y - ey) / 100.0);
                }
            }

            self.quadtree.update(entity.bounds(), id);
        }
    }

    pub fn get_state(&self) -> GameState {
        let mut state = GameState {
            entities: Vec::new(),
            map: self.map,
        };
        for entity in self.entities.values() {
            state.entities.push(entity.get_state());
        }
        state
    }
}

use crate::{
    game::entity::{Entity, XY},
    GameInput, GameState, Message,
};

use std::collections::HashMap;

pub trait WriteMessage {
    fn encode(&self) -> Message;
}

pub trait ReadMessage {
    fn from_vec(vec: Vec<Message>) -> Option<Self>
    where
        Self: Sized;
}

impl WriteMessage for XY {
    fn encode(&self) -> Message {
        Message::Array(vec![Message::Float32(self.x), Message::Float32(self.y)])
    }
}

impl WriteMessage for Entity {
    fn encode(&self) -> Message {
        Message::Array(vec![
            Message::Int32(self.id),
            self.pos.encode(),
            Message::Float32(self.size),
        ])
    }
}

impl WriteMessage for GameState {
    fn encode(&self) -> Message {
        let mut message = vec![];
        for entity in &self.entities {
            message.push(entity.encode());
        }
        Message::Array(message)
    }
}

impl ReadMessage for GameInput {
    fn from_vec(vec: Vec<Message>) -> Option<Self> {
        if let [Message::Bool(w), Message::Bool(a), Message::Bool(s), Message::Bool(d)] =
            vec.as_slice()
        {
            let mut input = Self {
                keys: HashMap::new(),
            };

            input.keys.insert('w', *w);
            input.keys.insert('a', *a);
            input.keys.insert('s', *s);
            input.keys.insert('d', *d);

            Some(input)
        } else {
            None
        }
    }
}


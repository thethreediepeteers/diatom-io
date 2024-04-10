use crate::{
    game::{entity::Entity, rect::Rectangle},
    GameState, Message,
};

pub trait WriteMessage {
    fn encode(&self) -> Message;
}

pub trait ReadMessage {
    fn from_vec(vec: Vec<Message>) -> Option<Self>
    where
        Self: Sized;
}

impl WriteMessage for Rectangle {
    fn encode(&self) -> Message {
        Message::Array(vec![
            Message::Float64(self.min_x),
            Message::Float64(self.min_y),
            Message::Float64(self.max_x),
            Message::Float64(self.max_y),
        ])
    }
}

impl WriteMessage for Entity {
    fn encode(&self) -> Message {
        Message::Array(vec![
            Message::Int32(self.id),
            Message::Int32(self.mockup_id),
            Message::Float64(self.angle),
            self.bounds.encode(),
        ])
    }
}

impl WriteMessage for GameState {
    fn encode(&self) -> Message {
        let mut message = vec![];
        for entity in &self.entities {
            message.push(entity.encode());
        }
        message.push(Message::Array(vec![
            Message::Float64(self.map.width),
            Message::Float64(self.map.height),
        ]));
        Message::Array(message)
    }
}
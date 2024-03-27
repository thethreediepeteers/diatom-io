use crate::{
    game::{entity::Entity, hashgrid::XY},
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

        message.push(Message::Array(vec![
            Message::Float32(self.map.width),
            Message::Float32(self.map.height),
        ]));

        Message::Array(message)
    }
}

use crate::{Connection, GameState};

pub enum GameEvent {
    Join(Connection),
    Quit(i32),
    Input(i32, u8, bool),
}

pub enum BroadcastEvent {
    Join(Connection),
    Quit(i32),
    SendState(GameState),
}

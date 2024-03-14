use crate::{Connection, GameInput, GameState};

pub enum GameEvent {
    Join(Connection),
    Quit(i32),
    Input(i32, GameInput),
}

pub enum BroadcastEvent {
    Join(Connection),
    Quit(i32),
    SendState(GameState),
}

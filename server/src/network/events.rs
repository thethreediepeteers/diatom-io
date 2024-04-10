use crate::{Connection, GameState};

pub enum GameEvent {
    Join(Connection),
    Quit(i32),
    Input(i32, Input),
}

pub enum Input {
    Keys(u8, bool),
    Mouse(f64),
}

pub enum BroadcastEvent {
    Join(Connection),
    Quit(i32),
    SendState(GameState),
}

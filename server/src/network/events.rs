use crate::{Connection, GameState};

pub enum GameEvent {
    Join(Connection),
    Quit(u16),
    Input(u16, Input),
}

pub enum Input {
    Keys(u8, bool),
    Mouse(f64),
    MouseClick(bool),
}

pub enum BroadcastEvent {
    Join(Connection),
    Quit(u16),
    SendState(GameState),
}
use yew::prelude::*;

pub enum WordleMsg {
    KeyPress(KeyboardEvent),
    WinGame,
    LoseGame,
    StartNew,
}

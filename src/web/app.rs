use crate::board::Board;
use crate::board::Keyboard;
use yew::prelude::*;

pub struct App {
    board: Board,
    keyboard: Keyboard,
    attempts: usize,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            board: Board::new(),
            keyboard: Keyboard::new(),
            attempts: 0,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "WORDLE" }</h1>
                // <div class="board">
                    { self.board.view() }
                // </div>
                // <div class="keyboard">
                    { self.keyboard.view() }
                // </div>
            </>
        }
    }
}

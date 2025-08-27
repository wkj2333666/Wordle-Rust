use crate::board::Board;
use crate::board::Keyboard;
use crate::buffer::GuessInputBuffer;
use crate::msg::WordleMsg;
use wordle_lib::Wordle;
use wordle_lib::game::WORD_LENGTH;
use yew::prelude::*;

#[derive(Debug)]
pub struct App {
    board: Board,
    keyboard: Keyboard,
    attempts: usize,
    buffer: GuessInputBuffer,

    game: Wordle,

    ans: String,
    guess_results: wordle_lib::game::Guess,
}

impl Component for App {
    type Message = WordleMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            board: Board::new(),
            keyboard: Keyboard::new(),
            attempts: 0,
            buffer: GuessInputBuffer::new(),
            game: wordle_lib::Wordle::new(),
            ans: String::new(),
            guess_results: wordle_lib::game::Guess::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_keydown_callback = link.callback(|event: KeyboardEvent| WordleMsg::KeyPress(event));

        html! {
            <>
                <h1>{ "WORDLE" }</h1>
                <div onkeydown={on_keydown_callback} tabindex="0">
                    { self.board.view() }
                    { self.keyboard.view() }
                </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WordleMsg::KeyPress(key) => self.get_one_key_input(&key),
            // WordleMsg::GuessInput(guess) => self.handle_one_guess(),
        };
        dbg!(self);
        true
    }
}

impl App {
    fn get_one_key_input(&mut self, key: &KeyboardEvent) {
        // let link = ctx.link();
        match key.key().as_str() {
            // "Enter" => link.send_message(WordleMsg::GuessInput(self.buffer.content.clone())),
            "Enter" => self.input_one_guess(),
            "Backspace" => self.buffer_pop(),
            _ if key.key().len() == 1 && key.key().chars().next().unwrap().is_alphabetic() => {
                self.buffer_push(key.key().chars().next().unwrap());
            }
            _ => {}
        }
    }

    fn input_one_guess(&mut self) {
        if !(self.buffer.is_valid() && self.game.acceptable_words.contains(&self.buffer.content)) {
            return;
        }

        let guess = self.buffer.content.clone();

        let game_win = self
            .game
            .handle_one_guess(&mut self.guess_results, &self.ans, &guess);

        self.attempts += 1;

        if game_win || self.attempts == 6 {
            self.game.game_recorder.add_game(game_win, self.attempts);
            self.start_new_game();
        }
    }

    fn start_new_game(&mut self) {
        // re-init game
        self.ans = self.game.gen_answer(&self.game.final_words);
        self.guess_results.clear();

        self.buffer.clear();
        self.attempts = 0;

        self.board.clear();
        self.keyboard.clear();
    }

    fn buffer_push(&mut self, c: char) {
        if self.buffer.len() < WORD_LENGTH {
            self.board
                .update_char(self.attempts, self.buffer.len(), Some(c));
            self.buffer.push(c);
        }
    }

    fn buffer_pop(&mut self) {
        if self.buffer.pop().is_some() {
            self.board
                .update_char(self.attempts, self.buffer.len(), None);
        }
    }
}

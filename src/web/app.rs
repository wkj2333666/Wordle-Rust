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
    win: Option<bool>,
    guess_results: wordle_lib::game::Guess,
}

impl Component for App {
    type Message = WordleMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut new_self = Self {
            board: Board::new(),
            keyboard: Keyboard::new(),
            attempts: 0,
            buffer: GuessInputBuffer::new(),
            game: wordle_lib::Wordle::new(),
            ans: String::new(),
            win: None,
            guess_results: wordle_lib::game::Guess::new(),
        };

        new_self.ans = new_self.game.gen_answer(&new_self.game.final_words);
        new_self.game.day_increment();
        log::info!("Answer of this game: {}", &new_self.ans);
        new_self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_keydown_callback = link.callback(|event: KeyboardEvent| WordleMsg::KeyPress(event));

        html! {
            <>
                <h1>{ "WORDLE" }</h1>
                <div onkeydown={on_keydown_callback} tabindex="0">
                    { self.board.view() }
                    {
                        if self.win == Some(true) {
                            html! {
                                <div class="btn-container">
                                    <button class="game-button" onclick={link.callback(|_| WordleMsg::StartNew)}>
                                        { "You win! Click to guess next word!" }
                                    </button>
                                </div>
                            }
                        } else if self.win == Some(false) {
                            html! {
                                <div class="btn-container">
                                    <button class="game-button" onclick={link.callback(|_| WordleMsg::StartNew)}>
                                        { format!("You lose! The answer was \"{}\". Click to guess next word!", self.ans) }
                                    </button>
                                </div>
                            }
                        } else {
                            html! {<></>}
                        }
                    }
                    { self.keyboard.view() }
                </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WordleMsg::KeyPress(key) if self.win == None => self.get_one_key_input(&key, _ctx),
            WordleMsg::WinGame => self.win = Some(true),
            WordleMsg::LoseGame => self.win = Some(false),
            WordleMsg::StartNew => self.start_new_game(),
            _ => (),
        };
        true
    }
}

impl App {
    fn get_one_key_input(&mut self, key: &KeyboardEvent, _ctx: &Context<Self>) {
        if key.key() == "Enter" {
            log::debug!("Enter pressed");
            self.input_one_guess(_ctx);
        } else if key.key() == "Backspace" {
            log::debug!("Backspace pressed");
            self.buffer_pop();
        } else if key.key().len() == 1 && key.key().chars().next().unwrap().is_alphabetic() {
            log::debug!("{} pressed", key.key());
            self.buffer_push(key.key().chars().next().unwrap());
        }
    }

    fn input_one_guess(&mut self, _ctx: &Context<Self>) {
        let link = _ctx.link();

        log::debug!("Handling input...");
        if !(self.buffer.is_valid() && self.game.acceptable_words.contains(&self.buffer.content)) {
            log::debug!("Invalid input!");
            return;
        }

        let guess = self.buffer.content.clone();

        let game_win = self
            .game
            .handle_one_guess(&mut self.guess_results, &self.ans, &guess);

        // log the guess result
        log::info!(
            "Guess result: {}",
            self.guess_results.history.last().unwrap().to_string()
        );

        // update render
        self.board
            .update_from_cli(&self.guess_results, self.attempts);
        self.keyboard.update_from_cli(&self.guess_results);

        // update attempts
        self.attempts += 1;
        log::debug!("Updated attempts: {}", self.attempts);

        if game_win {
            self.game.game_recorder.add_game(game_win, self.attempts);
            // self.start_new_game();
            link.send_message(WordleMsg::WinGame);
        } else if self.attempts == 6 {
            self.game.game_recorder.add_game(game_win, self.attempts);
            // self.start_new_game();
            link.send_message(WordleMsg::LoseGame);
        }

        self.buffer.clear();
    }

    fn start_new_game(&mut self) {
        log::info!("Starting new game!");
        // re-init game
        self.ans = self.game.gen_answer(&self.game.final_words);
        log::info!("Answer of this game: {}", &self.ans);
        self.game.day_increment();
        self.guess_results.clear();
        self.win = None;

        self.buffer.clear();
        self.attempts = 0;

        self.board.clear();
        self.keyboard.clear();
    }

    fn buffer_push(&mut self, c: char) {
        if self.buffer.len() < WORD_LENGTH {
            log::debug!(
                "board push at ({}, {}) with character {}.",
                self.attempts,
                self.buffer.len(),
                c
            );
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

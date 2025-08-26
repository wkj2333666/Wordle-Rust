use std::collections::BTreeMap;

use wordle_lib::game::{CharStatus, Guess, MAX_ATTEMPTS, WORD_LENGTH};
use yew::prelude::*;

pub struct Board {
    lines: [Line<WORD_LENGTH>; MAX_ATTEMPTS],
}

impl Board {
    pub fn new() -> Self {
        Self {
            lines: [Line::<WORD_LENGTH>::new(); MAX_ATTEMPTS],
        }
    }

    pub fn view(&self) -> Html {
        html! {
            <div class="board">
                { for self.lines.iter().map(|line| line.view()) }
            </div>
        }
    }

    pub fn update_from_cli(&mut self, cli_guess_results: &Guess, attempts_not_updated: usize) {
        self.lines[attempts_not_updated] = cli_guess_results
            .history
            .last()
            .unwrap()
            .status
            .iter()
            .zip(cli_guess_results.history.last().unwrap().content.chars())
            .map(|(status, letter)| (Some(letter), *status))
            .collect::<Vec<_>>()
            .into();
    }
}

pub struct Keyboard {
    line1: Line<10>,
    line2: Line<9>,
    line3: Line<7>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            line1: Line::<10>::new_with_chars(['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p']),
            line2: Line::<9>::new_with_chars(['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l']),
            line3: Line::<7>::new_with_chars(['z', 'x', 'c', 'v', 'b', 'n', 'm']),
        }
    }

    pub fn view(&self) -> Html {
        html! {
            <div class="keyboard">
                { self.line1.view() }
                { self.line2.view() }
                { self.line3.view() }
            </div>
        }
    }

    pub fn update_from_cli(&mut self, cli_guess_results: &Guess) {
        self.line1
            .update_from_map(&cli_guess_results.history.last().unwrap().keyboard);
        self.line2
            .update_from_map(&cli_guess_results.history.last().unwrap().keyboard);
        self.line3
            .update_from_map(&cli_guess_results.history.last().unwrap().keyboard);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Line<const LENGTH: usize> {
    cells: [(Option<char>, CharStatus); LENGTH],
}

impl<const LENGTH: usize> Line<LENGTH> {
    fn new() -> Self {
        Self {
            cells: [(None, CharStatus::Unknown); LENGTH],
        }
    }

    fn new_with_chars(chars: [char; LENGTH]) -> Self {
        Self {
            cells: chars
                .into_iter()
                .map(|c| (Some(c), CharStatus::Unknown))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="line">
            {
                self.cells.iter().map(|(c, s)| {
                    match (c, s) {
                        (Some(c), CharStatus::Correct) => html! { <span class="tile correct">{ c.to_string().to_uppercase() }</span> },
                        (Some(c), CharStatus::Misplaced) => html! { <span class="tile misplaced">{ c.to_string().to_uppercase() }</span> },
                        (Some(c), CharStatus::Wrong) => html! { <span class="tile wrong">{ c.to_string().to_uppercase() }</span> },
                        (Some(c), CharStatus::Unknown) => html! { <span class="tile unknown">{ c.to_string().to_uppercase() }</span> },
                        (None, _) => html! { <span class="tile unknown"> </span> },
                    }
                })
                .collect::<Html>()
            }
            </div>
        }
    }

    fn update_from_map(&mut self, map: &BTreeMap<char, CharStatus>) {
        self.cells
            .iter_mut()
            .for_each(|cell| cell.1 = map.get(&cell.0.unwrap()).unwrap().clone());
    }
}

impl<const LENGTH: usize> From<Vec<(Option<char>, CharStatus)>> for Line<LENGTH> {
    fn from(vec: Vec<(Option<char>, CharStatus)>) -> Self {
        Self {
            cells: vec.try_into().unwrap(),
        }
    }
}

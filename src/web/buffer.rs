use wordle_lib::game::WORD_LENGTH;
use yew::prelude::*;

// #[derive(Properties, PartialEq, Default)]
#[derive(Debug)]
pub struct GuessInputBuffer {
    pub content: String,
}

impl GuessInputBuffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, new_char: char) {
        self.content.push(new_char.to_ascii_lowercase());
    }

    pub fn pop(&mut self) -> Option<char> {
        self.content.pop()
    }

    pub fn is_valid(&self) -> bool {
        self.content.len() == WORD_LENGTH
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}

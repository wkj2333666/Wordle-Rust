use yew::prelude::*;

pub struct Board {
    lines: [Line<5>; 6],
}

impl Board {
    pub fn new() -> Self {
        Self {
            lines: [Line::<5>::new(); 6],
        }
    }

    pub fn view(&self) -> Html {
        html! {
            <div class="row">
                { for self.lines.iter().map(|line| line.view()) }
            </div>
        }
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
            line1: Line::<10>::new(),
            line2: Line::<9>::new(),
            line3: Line::<7>::new(),
        }
    }

    pub fn view(&self) -> Html {
        html! {
            <div class="row">
                { self.line1.view() }
                { self.line2.view() }
                { self.line3.view() }
            </div>
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Line<const LENGTH: usize> {
    cells: [(Option<char>, Status); LENGTH],
}

impl<const LENGTH: usize> Line<LENGTH> {
    fn new() -> Self {
        Self {
            cells: [(None, Status::Unknown); LENGTH],
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
            {
                self.cells.iter().map(|(c, s)| {
                    match (c, s) {
                        (Some(c), Status::Correct) => html! { <span class="correct">{ c.to_string().to_uppercase() }</span> },
                        (Some(c), Status::Misplaced) => html! { <span class="misplaced">{ c.to_string().to_uppercase() }</span> },
                        (Some(c), Status::Wrong) => html! { <span class="wrong">{ c.to_string().to_uppercase() }</span> },
                        (Some(c), Status::Unknown) => html! { <span class="unknown">{ c.to_string().to_uppercase() }</span> },
                        (None, _) => html! { <span class="unknown"> </span> },
                    }
                })
                .collect::<Html>()
            }
            </div>
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Status {
    Unknown,
    Correct,
    Misplaced,
    Wrong,
}

use app::App;
// use yew::prelude;

mod app;
mod board;

fn main() {
    yew::Renderer::<App>::new().render();
}

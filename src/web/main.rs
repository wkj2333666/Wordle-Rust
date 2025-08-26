// #[cfg(target_arch = "wasm32")]
mod app;
// #[cfg(target_arch = "wasm32")]
mod board;

fn main() {
    // #[cfg(target_arch = "wasm32")]
    yew::Renderer::<app::App>::new().render();
}

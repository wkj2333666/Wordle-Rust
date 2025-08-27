#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod board;
#[cfg(target_arch = "wasm32")]
mod buffer;
#[cfg(target_arch = "wasm32")]
mod msg;

fn main() {
    #[cfg(target_arch = "wasm32")]
    yew::Renderer::<app::App>::new().render();
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::default());
}

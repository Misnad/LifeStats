mod app;
mod components;
mod models;
mod state;

fn main() {
    yew::Renderer::<app::App>::new().render();
}

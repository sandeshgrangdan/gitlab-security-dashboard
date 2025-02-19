mod api;
mod app;
mod atoms;
mod components;
mod container;
mod context;
mod molecules;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

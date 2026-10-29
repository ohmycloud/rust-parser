#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod scrollbars;

#[cfg(target_arch = "wasm32")]
fn main() {
    yew::Renderer::<app::App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("recipe-web runs in a browser. Start it with: trunk serve --open");
}

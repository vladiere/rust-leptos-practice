mod app;
mod components;
mod layouts;
mod pages;

fn main() {
    use crate::app::App;

    leptos::mount_to_body(App)
}

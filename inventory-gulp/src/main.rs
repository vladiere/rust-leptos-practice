mod app;

fn main() {
    use inventory_gulp::app::*;

    dotenv::dotenv().ok();
    leptos::mount_to_body(App)
}

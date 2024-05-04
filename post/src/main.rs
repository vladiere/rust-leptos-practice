use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::js_sys::JsString;
use web_sys::console;

fn main() {
    leptos::mount_to_body(App)
}

#[derive(Clone, Deserialize, Debug, Serialize)]
struct PostBody {
    #[serde(rename = "userId")]
    user_id: i64,
    id: Option<i64>,
    title: String,
    body: String,
}

#[component]
fn App() -> impl IntoView {
    let client = reqwest::Client::new();
    wasm_bindgen_futures::spawn_local(async move {
        let post_url = "https://jsonplaceholder.typicode.com/posts";
        let fetched_post: Vec<PostBody> = client
            .get(post_url)
            .send()
            .await
            .json()
            .await
            .expect("Request Error");

        console::log_1(&JsString::from(format!("{:?}", fetched_post)))
    });
    view! {
        <h1>"Hello world"</h1>
    }
}

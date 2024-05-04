use crate::components::LinksComponent;
use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <h1>"Home page"</h1>
            <LinksComponent />
        </div>
    }
}

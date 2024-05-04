use crate::pages::HomePage;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <div class="w-screen flex flex-row gap-5">
            <HomePage />
            <Outlet />
        </div>
    }
}

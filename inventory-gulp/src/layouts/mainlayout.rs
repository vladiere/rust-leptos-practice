use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}

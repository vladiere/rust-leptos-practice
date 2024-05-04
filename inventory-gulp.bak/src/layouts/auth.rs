use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn AuthLayout() -> impl IntoView {
    view! {
        <Outlet />
    }
}

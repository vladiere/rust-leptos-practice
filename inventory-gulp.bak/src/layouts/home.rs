use leptos::*;
use leptos_router::{Outlet, A};

#[component]
pub fn HomeLayout() -> impl IntoView {
    view! {
        <div class="flex flex-col dark:text-white">
            <nav class="flex flex-row items-center justify-center gap-5 dark:bg-gray-900 bg-gray-300 text-5md py-3 font-medium">
                <A href="/" class="hover:text-blue-500">"Home"</A>
                <A href="/" class="hover:text-blue-500">"Add new product"</A>
                <A href="/auth/login" class="hover:text-blue-500">"Logout"</A>
            </nav>
            <Outlet />
        </div>
    }
}

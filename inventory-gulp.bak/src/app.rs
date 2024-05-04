use crate::{AuthLayout, HomeLayout, HomePage, LoginPage, RegisterPage};
use leptos::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/auth" view=AuthLayout>
                    <Route path="login" view=LoginPage />
                    <Route path="register" view=RegisterPage />
                </Route>
                <Route path="" view=HomeLayout>
                    <Route path="" view=HomePage />
                </Route>
                <Route path="/*any" view=NotfoundPage />
            </Routes>
        </Router>
    }
}

#[component]
fn NotfoundPage() -> impl IntoView {
    view! {
        <div class="dark:bg-gray-900 dark:text-white h-screen">
            <div class="w-full h-full flex flex-row items-center text-4xl font-medium justify-center">
                "404 | Not Found"
            </div>
        </div>
    }
}

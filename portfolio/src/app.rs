use crate::{layouts::MainLayout, pages::*};
use leptos::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=MainLayout >
                    <Route path="/" view=AboutPage />
                    <Route path="/about" view=AboutPage />
                    <Route path="/skills" view=SkillsPage />
                </Route>
                <Route path="*any" view=NotFound />
            </Routes>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="h-screen bg-gray-200 dark:bg-gray-800 dark:text-white">
            <div class="h-full flex flex-col items-center justify-center gap-10">
                <h1 class="text-8xl">"404"</h1>
                <h1 class="text-4xl">"Page not Found"</h1>
            </div>
        </div>
    }
}

use crate::{
    layouts::{homelayout::HomeLayout, mainlayout::MainLayout, productlayout::ProductLayout},
    pages::{
        home::HomePage,
        login::LoginPage,
        product::{AddProduct, EditProduct},
        register::RegisterPage,
    },
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="" view=MainLayout>
                        <Route path="" view=HomeLayout >
                            <Route path="" view=HomePage />
                            <Route path="product" view=ProductLayout >
                                <Route path="" view=AddProduct />
                                <Route path="edit" view=EditProduct />
                            </Route>
                        </Route>
                        <Route path="login" view=LoginPage />
                        <Route path="register" view=RegisterPage />
                    </Route>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <section class="h-screen">
            <div class="flex flex-row items-center justify-center h-full gap-5 dark:bg-gray-900">
                <h1 class="text-4xl font-medium dark:text-white">"404"</h1>
                <h1 class="text-2xl font-medium dark:text-white">"|"</h1>
                <h1 class="text-2xl font-medium dark:text-white">"Not Found"</h1>
            </div>
        </section>
    }
}

#[component]
fn DashboardRoute() -> impl IntoView {
    view! {}
}

use leptos::*;
use leptos_router::A;
use std::error::Error;
use web_sys::{console::log_1, js_sys::JsString};

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email_address, set_email_address) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (is_email, set_is_email) = create_signal(String::new());
    let (is_password, set_is_password) = create_signal(String::new());
    let (token, set_token) = create_signal(String::new());
    let (err_res, set_err_res) = create_signal(String::new());
    let (is_disabled, set_disabled) = create_signal(false);

    let handle_submit = move |_| log_1(&JsString::from("Clicked"));

    view! {
        { token }
        { err_res }
        <section class="h-screen flex items-center dark:bg-gray-700">
            <form on:submit=move |ev| ev.prevent_default() class="w-5/6 md:w-1/3 md:p-0 p-3 mx-auto" >
                <h1 class="text-2xl mb-5 font-bold text-left dark:text-white">"Sign in to your account"</h1>
                <div class="mb-5">
                    <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Your email"</label>
                    <input type="email" name="email_address" id="email" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="name@email.com" required on:input=move |ev| set_email_address.set(event_target_value(&ev))/>
                    <span class="block my-2 text-sm font-medium text-red-400">{ is_email }</span>
                </div>
                <div class="mb-5">
                    <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Your password"</label>
                    <input type="password" name="password" id="password" class="bg-gray-50 border border-gray-300 outline-none text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="**********" required on:input=move |ev| set_password.set(event_target_value(&ev))/>
                    <span class="block my-2 text-sm font-medium text-red-400">{ is_password }</span>
                </div>
                <button on:click=handle_submit class="text-white mb-5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">"Sign in"</button>
                <p class="flex flex-col md:flex-row gap-4 font-medium dark:text-white">
                    "Don't have an account?"
                    <A href="/register" class="font-medium text-2md hover:text-blue-600 hover:underline hover:decoration-solid dark:text-white">"Sign up now"</A>
                </p>
            </form>
        </section>
    }
}

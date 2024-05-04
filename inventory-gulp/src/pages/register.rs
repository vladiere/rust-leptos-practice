use leptos::*;
use leptos_router::A;
use serde::{Deserialize, Serialize};
use web_sys::{console::log_1, js_sys::JsString};

#[derive(Deserialize, Serialize, Debug)]
struct Register {
    firstname: String,
    lastname: String,
    email_address: String,
    password: String,
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirmpass, set_confirmpass) = create_signal(String::new());
    let (firstname, set_firstname) = create_signal(String::new());
    let (lastname, set_lastname) = create_signal(String::new());
    let (msg, set_msg) = create_signal(String::new());

    let click_btn = move |_| {
        log_1(&JsString::from(format!(
            "{} {} {} {} {}",
            firstname.get(),
            lastname.get(),
            email.get(),
            password.get(),
            msg.get()
        )))
    };

    create_effect(move |_| {
        if password.get() != confirmpass.get() {
            set_msg.set(String::from("Confirm password does not match"));
        } else {
            set_msg.set("".to_string());
        }
    });

    view! {
        <section class="h-screen flex items-center dark:bg-gray-700">
            <div  class="w-5/6 md:w-1/3 md:p-0 p-3 mx-auto">
                <>
                    <h1 class="text-2xl mb-5 font-bold text-left dark:text-white">"Sign up your account"</h1>
                    <div class="mb-5">
                        <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Email address"</label>
                        <input type="email" name="email" id="email" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="name@email.com" required on:input=move |ev| set_email.set(event_target_value(&ev)) />
                    </div>
                    <div class="mb-5">
                        <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Password"</label>
                        <input type="password" name="password" id="password" class="bg-gray-50 border border-gray-300 outline-none text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="**********" required on:input=move |ev| set_password.set(event_target_value(&ev))/>
                        <span class="block mb-2 text-sm font-medium text-red-600">{ msg }</span>
                    </div>
                    <div class="mb-5">
                        <label for="password2" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Confirm password"</label>
                        <input type="password" name="confirmpass" id="password2" class="bg-gray-50 border border-gray-300 outline-none text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="**********" required on:input=move |ev| set_confirmpass.set(event_target_value(&ev))/>
                        <span class="block mb-2 text-sm font-medium text-red-600">{ msg }</span>
                    </div>
                    <div class="grid md:grid-cols-2 md:gap-4">
                        <div class="w-full group mb-5">
                            <label for="firstname" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Firstname"</label>
                            <input type="text" name="firstname" id="firstname" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="john" required on:input=move |ev| set_firstname.set(event_target_value(&ev))/>
                        </div>
                        <div class="w-full group mb-5">
                            <label for="lastname" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Lastname"</label>
                            <input type="text" name="lastname" id="lastname" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="doe" required on:input=move |ev| set_lastname.set(event_target_value(&ev))/>
                        </div>
                    </div>
                    <button type="button" on:click=click_btn class="text-white mb-5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">"Sign up"</button>
                    <p class="flex flex-col md:flex-row gap-4 font-medium dark:text-white">
                        "Already have an account?"
                        <A href="/login" class="font-medium text-2md hover:text-blue-600 hover:underline hover:decoration-solid dark:text-white">"Sign in now"</A>
                    </p>
                </>
            </div>
        </section>
    }
}

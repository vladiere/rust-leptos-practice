use leptos::*;
use leptos_router::A;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="h-screen bg-gray-200 dark:bg-gray-900 dark:text-white">
            <div class="flex flex-row items-center justify-center w-full h-full">
                <div class="w-[30%] flex flex-col gap-4">
                    <div>
                        <label for="email_address" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Email address"</label>
                        <input type="email" id="email_address" name="email_address" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="name@email.com" required />
                    </div>
                    <div>
                        <label for="first_name" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Password"</label>
                        <input type="password" id="first_name" class="bg-gray-50 border outline-none border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="********" required />
                    </div>
                    <div class="flex flex-row justify-between">
                        <button type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">"Submit"</button>
                        <A href="/auth/register" class="text-6md font-medium hover:text-blue-800">"Register"</A>
                    </div>
                </div>
            </div>
        </div>
    }
}

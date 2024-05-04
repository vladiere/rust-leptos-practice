use crate::components::search::SearchComponent;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="relative overflow-x-auto shadow-md sm:rounded-lg p-3">
            <SearchComponent />

            <table class="w-full text-sm text-left rtl:text-right text-gray-500 dark:text-gray-400">
                <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                    <tr>
                        <th scope="col" class="px-6 py-3">
                            "Product name"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Type"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Category"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Price"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            "Quantity"
                        </th>
                        <th scope="col" class="px-6 py-3">
                            <span class="sr-only">"Edit"</span>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Apple MacBook Pro 17"
                        </th>
                        <td class="px-6 py-4">
                            "Silver"
                        </td>
                        <td class="px-6 py-4">
                            "Laptop"
                        </td>
                        <td class="px-6 py-4">
                            "$2999"
                        </td>
                        <td class="px-6 py-4">
                            "100"
                        </td>
                        <td class="px-6 py-4 text-right">
                            <a href="/product/edit" class="font-medium mr-3 text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                            <a href="#" class="font-medium text-red-600 dark:text-red-500 hover:underline">"Remove"</a>
                        </td>
                    </tr>
                    <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Microsoft Surface Pro"
                        </th>
                        <td class="px-6 py-4">
                            "White"
                        </td>
                        <td class="px-6 py-4">
                            "Laptop PC"
                        </td>
                        <td class="px-6 py-4">
                            "$1999"
                        </td>
                        <td class="px-6 py-4">
                            "100"
                        </td>
                        <td class="px-6 py-4 text-right">
                            <a href="/product/edit" class="font-medium mr-3 text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                            <a href="#" class="font-medium text-red-600 dark:text-red-500 hover:underline">"Remove"</a>
                        </td>
                    </tr>
                    <tr class="bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-600">
                        <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            "Magic Mouse 2"
                        </th>
                        <td class="px-6 py-4">
                            "Black"
                        </td>
                        <td class="px-6 py-4">
                            "Accessories"
                        </td>
                        <td class="px-6 py-4">
                            "$99"
                        </td>
                        <td class="px-6 py-4">
                            "100"
                        </td>
                        <td class="px-6 py-4 text-right">
                            <a href="/product/edit" class="font-medium mr-3 text-blue-600 dark:text-blue-500 hover:underline">"Edit"</a>
                            <a href="#" class="font-medium text-red-600 dark:text-red-500 hover:underline">"Remove"</a>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

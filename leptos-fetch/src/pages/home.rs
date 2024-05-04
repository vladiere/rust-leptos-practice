use crate::components::counter_btn::Button;
use leptos::{error::Result, *};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                        alt="Leptos Logo"
                        height="200"
                        width="400"
                    />
                </picture>

                <h1>"Welcome to Leptos"</h1>

                <div class="buttons">
                    <Button/>
                    <Button increment=5/>
                </div>
                <FetchExample />
            </div>
        </ErrorBoundary>
    }
}

#[component]
pub fn FetchExample() -> impl IntoView {
    let (user_id, set_user_id) = create_signal(0);

    let users = create_local_resource(move || Some(user_id.get()), fetch_users);

    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };

        view! {
            <div class="error">
                <h1>"Error"</h1>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let users_view = move || {
        users.and_then(|data| {
            data.iter()
                .map(|s| {
                    view! {
                        <li>ID: {s.id.clone()}</li>
                        <li>NAME: {s.name.clone()}</li>
                        <li>USERNAME: {s.username.clone()}</li>
                        <li>EMAIL: {s.email.clone()}</li>
                        <li>STREET: {s.address.street.clone()}</li>
                        <li>SUITE: {s.address.suite.clone()}</li>
                        <li>CITY: {s.address.city.clone()}</li>
                        <li>ZIPCODE: {s.address.zipcode.clone()}</li>
                        <li>PHONE: {s.phone.clone()}</li>
                        <li>WEBSITE: {s.website.clone()}</li>
                        <li>COMPANY NAME: {s.company.name.clone()}</li>
                        <li>COMPANY CATCH PHRASE: {s.company.catch_phrase.clone()}</li>
                        <li>COMPANY BS: {s.company.bs.clone()}</li>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <div>
            <label>
                "Search user id"
                <input
                    min="0"
                    type="number"
                    prop:value=move || user_id.get().to_string()
                    on:input=move |ev| {
                        let val = event_target_value(&ev).parse().unwrap_or(0);
                        set_user_id.set(val);
                    }
                />
            </label>
            <Transition fallback=move || {
                view! { <div>"Loading users..."</div> }
            }>
                <ErrorBoundary fallback>
                    <div>
                        <ul>
                            {users_view}
                        </ul>
                    </div>
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[derive(Error, Clone, Debug)]
pub enum UserError {
    #[error("Request more than zero users.")]
    NonZeroUser,
}

async fn fetch_users(id: Option<i64>) -> Result<Vec<Users>> {
    if let Some(id) = id {
        let res = reqwasm::http::Request::get(&format!(
            "https://jsonplaceholder.typicode.com/users/{id}"
        ))
        .send()
        .await?
        .json::<Vec<Users>>()
        .await?
        .into_iter()
        .collect::<Vec<_>>();
        Ok(res)
    } else {
        Err(UserError::NonZeroUser.into())
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Geo {
    lat: String,
    lng: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Company {
    name: String,
    catch_phrase: String,
    bs: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Users {
    id: i64,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

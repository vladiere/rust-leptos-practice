use crate::services::test_route::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let add_people = create_server_multi_action::<AddPeople>();
    let submissions = add_people.submissions();
    let remove_people = create_server_action::<RemovePeople>();

    let peoples = create_resource(move || add_people.version().get(), move |_| get_people());

    view! {
        <div>
            <MultiActionForm
                on:submit=move |ev| {
                    let data = AddPeople::from_event(&ev).expect("to parse from data");
                    if data.firstname != "" && data.lastname != "" && data.middlename != "" {
                        ev.prevent_default();
                    }
                }
                action=add_people
            >
                <label> "Add new people" </label>
                <input type="text" name="lastname" placeholder="Lastname" />
                <input type="text" name="firstname" placeholder="Firstname" />
                <input type="text" name="middlename" placeholder="Middlename" />
                <button type="submit" >"Submit"</button>
            </MultiActionForm>
            <Transition fallback=move || view!{ <p>"Fetching..."</p>}>
                {move || {
                             let existing_people = {
                                 move || {
                                     peoples.get()
                                         .map(move |peoples| match peoples {
                                             Err(e) => {
                                                 view!{<pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                             }
                                             Ok(peoples) => {
                                                 if peoples.is_empty() {
                                                     view! {<p>"No list of people found"</p>}.into_view()
                                                 } else {
                                                     peoples
                                                         .into_iter()
                                                         .map(move |ppl| {
                                                             view!{
                                                                <li>
                                                                    {ppl.lastname}" "{ppl.firstname}","{ppl.middlename}
                                                                    <ActionForm action=remove_people>
                                                                        <input type="hidden" name="id" value={ppl.id} />
                                                                        <input type="submit" value="Remove" />
                                                                    </ActionForm>
                                                                </li>
                                                             }
                                                         })
                                                        .collect_view()
                                                 }
                                             }
                                         })
                                        .unwrap_or_default()
                                 }
                             };

                             let pending_peoples = move || {
                                 submissions
                                     .get()
                                     .into_iter()
                                     .filter(|submission| submission.pending().get())
                                     .map(|submission| {
                                         view! {
                                             <li>
                                                {move || submission.input.get().map(|data| data.lastname)}" "
                                                {move || submission.input.get().map(|data| data.firstname)}","
                                                {move || submission.input.get().map(|data| data.middlename)}
                                             </li>
                                         }
                                     })
                                    .collect_view()
                             };

                             view!{
                                 <ul>
                                    {existing_people}
                                    {pending_peoples}
                                 </ul>
                             }
                         }
                }
            </Transition>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

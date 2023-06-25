use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let always_errors = create_server_action::<AlwaysErrors>(cx);
    let error = create_rw_signal(cx, None);

    view! { cx,
            <ActionForm action=always_errors error=error>
                <button type="submit">
                    "Click Me To Trigger Server-Side Error"
                </button>
            </ActionForm>
            {move || {
                error
                    .with(|e| {
                        if let Some(err) = e {
                            view! { cx, <p>{format!("ERROR: {err}")}</p> }
                                .into_view(cx)
                        } else {
                            view! { cx, <div></div> }
                                .into_view(cx)
                        }
                    })
            }}
    }

}

#[server(AlwaysErrors, "/api")]
pub async fn always_errors(cx: Scope) -> Result<(),ServerFnError> {
    Err(ServerFnError::ServerError("I will always error".to_string()))

}


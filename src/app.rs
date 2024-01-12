use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::home::Home;
use crate::pages::admin::Admin;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="SyntaxMakers - Rustlang Blog"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home />
                    <Route path="/admin" view=Admin />
                    <Route path="/*" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

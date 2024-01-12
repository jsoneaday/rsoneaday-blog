use leptos::*;
use leptos_meta::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="SyntaxMakers - Rustlang Blog"/>

        {children()}
    }
}
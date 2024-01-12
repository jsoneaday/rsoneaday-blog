use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Eq)]
struct PostSelector {
    id: usize
}

#[component]
pub fn Post() -> impl IntoView {
    let post_selector = use_params::<PostSelector>();
    let id = move || {
        post_selector.with(|params| {
            params.as_ref()
                .map(|params| params.id)
                .unwrap_or_default()
        })
    };
    //logging::log!("{:?}", id());

    view! {
        <p>{move || id()}</p>
    }
}
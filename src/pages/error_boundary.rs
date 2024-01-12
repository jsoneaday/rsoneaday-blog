use leptos::*;

#[component]
pub fn GeneralErrorsDisplay(children: Children) -> impl IntoView {
    view! {
        <ErrorBoundary
            fallback=|errors| view! {
                <ul>
                    {
                        move || errors
                            .get()
                            .into_iter()
                            .map(|(_, err)| view! {
                                <li>
                                    { err.to_string()}
                                </li>
                            })
                            .collect_view()
                    }
                </ul>
            }
        >
            {children()}
        </ErrorBoundary>
    }
}
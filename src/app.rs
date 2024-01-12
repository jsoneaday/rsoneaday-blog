use leptos::*;
use leptos_router::*;
use crate::pages::home::Home;
use crate::pages::admin::Admin;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    view! {
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

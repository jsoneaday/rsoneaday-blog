use leptos::*;
use leptos_router::*;
use crate::pages::home::Home;
use crate::pages::admin::Admin;
use crate::pages::post::Post;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="layout-container">
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/post/:id" view=Post />
                    <Route path="/admin" view=Admin />
                    <Route path="/*" view=NotFound />
                </Routes>
            </main>
        </Router>    
    }
}

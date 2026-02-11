use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::pages::{confirmation::ConfirmationPage, menu::MenuPage, order::OrderPage};
use crate::state::cart::provide_cart_state;

#[component]
pub fn App() -> impl IntoView {
    // Provide cart state to entire app
    provide_cart_state();

    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { "Page not found" }>
                    <Route path=path!("") view=MenuPage />
                    <Route path=path!("order") view=OrderPage />
                    <Route path=path!("confirmation/:id") view=ConfirmationPage />
                </Routes>
            </main>
        </Router>
    }
}

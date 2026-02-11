use leptos::prelude::*;

use super::cart_summary::CartSummary;

#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <div class="page-layout">
            <header class="page-header">
                <div class="header-content">
                    <h1 class="logo">"🍕 Royal Pizza"</h1>
                    <CartSummary />
                </div>
            </header>
            <main class="page-main">
                {children()}
            </main>
            <footer class="page-footer">
                <p>"© 2026 Royal Pizza. All rights reserved."</p>
            </footer>
        </div>
    }
}

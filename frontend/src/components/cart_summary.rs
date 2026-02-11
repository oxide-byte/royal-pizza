use leptos::prelude::*;
use leptos_router::hooks::*;

use crate::state::cart::use_cart;
use crate::utils::format::format_currency;

#[component]
pub fn CartSummary() -> impl IntoView {
    let cart = use_cart();
    let navigate = use_navigate();

    let item_count = create_memo(move |_| cart.item_count());
    let total = create_memo(move |_| cart.total());
    let is_empty = create_memo(move |_| cart.is_empty());

    let proceed_to_order = move |_| {
        navigate("/order", Default::default());
    };

    view! {
        <div class="cart-summary">
            <div class="cart-info">
                <span class="cart-icon">"🛒"</span>
                <span class="cart-count">{move || item_count.get()}</span>
                <span class="cart-total">{move || format_currency(total.get())}</span>
            </div>
            <button
                class="proceed-button"
                disabled=move || is_empty.get()
                on:click=proceed_to_order
            >
                "Proceed to Order"
            </button>
        </div>
    }
}

use leptos::prelude::*;
use shared::models::{Pizza, PizzaSize};

use crate::utils::format::format_currency;

#[component]
pub fn PizzaCard(
    pizza: Pizza,
    #[prop(into)] on_add_to_cart: Callback<(String, PizzaSize, u32), ()>,
) -> impl IntoView {
    let (selected_size, set_selected_size) = signal(PizzaSize::Medium);
    let (quantity, set_quantity) = signal(1u32);
    let (show_added_feedback, set_show_added_feedback) = signal(false);

    let price = pizza.price.clone();
    let current_price = Memo::new(move |_| {
        selected_size.get().get_price(&price)
    });

    let add_to_cart = move |_| {
        on_add_to_cart.run((
            pizza.id.clone(),
            selected_size.get(),
            quantity.get(),
        ));

        // Show feedback
        set_show_added_feedback.set(true);
        set_timeout(
            move || set_show_added_feedback.set(false),
            std::time::Duration::from_secs(2),
        );
    };

    view! {
        <div class="pizza-card">
            <div class="pizza-image">
                {match pizza.image_url.as_ref() {
                    Some(url) => view! { <img src={url.clone()} alt={pizza.name.clone()} /> }.into_any(),
                    None => view! { <div class="placeholder-image">"🍕"</div> }.into_any()
                }}
            </div>

            <div class="pizza-info">
                <h3 class="pizza-name">{pizza.name.clone()}</h3>
                <p class="pizza-description">{pizza.description.clone()}</p>

                <div class="pizza-ingredients">
                    <strong>"Ingredients: "</strong>
                    <span>{pizza.ingredients.join(", ")}</span>
                </div>

                <div class="pizza-prices">
                    <div class="price-options">
                        <button
                            class="size-button"
                            class:selected=move || selected_size.get() == PizzaSize::Small
                            on:click=move |_| set_selected_size.set(PizzaSize::Small)
                        >
                            <span class="size-name">"Small"</span>
                            <span class="size-price">{format_currency(pizza.price.small)}</span>
                        </button>
                        <button
                            class="size-button"
                            class:selected=move || selected_size.get() == PizzaSize::Medium
                            on:click=move |_| set_selected_size.set(PizzaSize::Medium)
                        >
                            <span class="size-name">"Medium"</span>
                            <span class="size-price">{format_currency(pizza.price.medium)}</span>
                        </button>
                        <button
                            class="size-button"
                            class:selected=move || selected_size.get() == PizzaSize::Large
                            on:click=move |_| set_selected_size.set(PizzaSize::Large)
                        >
                            <span class="size-name">"Large"</span>
                            <span class="size-price">{format_currency(pizza.price.large)}</span>
                        </button>
                    </div>
                </div>

                <div class="pizza-actions">
                    <div class="quantity-selector">
                        <label>"Quantity: "</label>
                        <input
                            type="number"
                            min="1"
                            max="10"
                            prop:value=move || quantity.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<u32>() {
                                    set_quantity.set(val.max(1).min(10));
                                }
                            }
                        />
                    </div>

                    <button
                        class="add-to-cart-button"
                        class:added=move || show_added_feedback.get()
                        on:click=add_to_cart
                        disabled=move || !pizza.is_available
                    >
                        {move || if show_added_feedback.get() {
                            "✓ Added!"
                        } else if !pizza.is_available {
                            "Unavailable"
                        } else {
                            "Add to Cart"
                        }}
                    </button>
                </div>

                <div class="selected-price">
                    <strong>"Price: "</strong>
                    {move || format_currency(current_price.get() * quantity.get() as f64)}
                </div>
            </div>
        </div>
    }
}

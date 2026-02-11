use leptos::prelude::*;
use shared::models::{CustomPizza, PizzaSize};

use crate::utils::format::format_currency;

const MAX_INSTRUCTIONS_LENGTH: usize = 500;
const MIN_INSTRUCTIONS_LENGTH: usize = 10;

#[component]
pub fn CustomPizzaCard(
    #[prop(into)] on_add_to_cart: Callback<(CustomPizza, u32), ()>,
) -> impl IntoView {
    let (instructions, set_instructions) = signal(String::new());
    let (selected_size, set_selected_size) = signal(PizzaSize::Medium);
    let (quantity, set_quantity) = signal(1u32);
    let (show_added_feedback, set_show_added_feedback) = signal(false);
    let (validation_error, set_validation_error) = signal(None::<String>);

    let char_count = Memo::new(move |_| instructions.get().len());
    let remaining_chars = Memo::new(move |_| MAX_INSTRUCTIONS_LENGTH - char_count.get());

    let current_price = Memo::new(move |_| {
        let custom = CustomPizza {
            instructions: String::new(),
            size: selected_size.get(),
        };
        custom.get_price()
    });

    let is_valid = Memo::new(move |_| {
        let len = char_count.get();
        len >= MIN_INSTRUCTIONS_LENGTH && len <= MAX_INSTRUCTIONS_LENGTH
    });

    let add_to_cart = move |_| {
        let inst = instructions.get();

        // Validate
        if inst.trim().is_empty() {
            set_validation_error.set(Some("Please provide instructions for your custom pizza.".to_string()));
            return;
        }

        if inst.len() < MIN_INSTRUCTIONS_LENGTH {
            set_validation_error.set(Some(format!(
                "Instructions must be at least {} characters.",
                MIN_INSTRUCTIONS_LENGTH
            )));
            return;
        }

        if inst.len() > MAX_INSTRUCTIONS_LENGTH {
            set_validation_error.set(Some(format!(
                "Instructions cannot exceed {} characters.",
                MAX_INSTRUCTIONS_LENGTH
            )));
            return;
        }

        // Clear validation error
        set_validation_error.set(None);

        // Create custom pizza
        let custom = CustomPizza {
            instructions: inst,
            size: selected_size.get(),
        };

        on_add_to_cart.run((custom, quantity.get()));

        // Show feedback
        set_show_added_feedback.set(true);
        set_timeout(
            move || {
                set_show_added_feedback.set(false);
                set_instructions.set(String::new()); // Clear form
            },
            std::time::Duration::from_secs(2),
        );
    };

    view! {
        <div class="custom-pizza-card">
            <div class="custom-pizza-header">
                <h3>"🎨 Create Your Own Pizza"</h3>
                <p class="custom-pizza-subtitle">
                    "Tell us what you'd like on your pizza!"
                </p>
            </div>

            <div class="custom-pizza-form">
                <div class="instructions-section">
                    <label for="instructions">
                        <strong>"Your Instructions"</strong>
                        <span class="char-counter" class:warning=move || remaining_chars.get() < 50>
                            {move || format!("{} / {}", char_count.get(), MAX_INSTRUCTIONS_LENGTH)}
                        </span>
                    </label>
                    <textarea
                        id="instructions"
                        class="instructions-input"
                        placeholder="Example: Extra cheese, pepperoni, mushrooms, and olives. Light on the sauce."
                        maxlength=MAX_INSTRUCTIONS_LENGTH
                        rows="4"
                        prop:value=move || instructions.get()
                        on:input=move |ev| {
                            set_instructions.set(event_target_value(&ev));
                            set_validation_error.set(None);
                        }
                    />
                    <p class="instructions-hint">
                        {format!("Minimum {} characters. Be specific about your toppings and preferences!", MIN_INSTRUCTIONS_LENGTH)}
                    </p>
                </div>

                {move || validation_error.get().map(|err| {
                    view! {
                        <div class="validation-error">
                            {err}
                        </div>
                    }
                })}

                <div class="size-section">
                    <label><strong>"Size"</strong></label>
                    <div class="size-options">
                        <button
                            class="size-button"
                            class:selected=move || selected_size.get() == PizzaSize::Small
                            on:click=move |_| set_selected_size.set(PizzaSize::Small)
                        >
                            <span class="size-name">"Small"</span>
                            <span class="size-price">"$10.99"</span>
                        </button>
                        <button
                            class="size-button"
                            class:selected=move || selected_size.get() == PizzaSize::Medium
                            on:click=move |_| set_selected_size.set(PizzaSize::Medium)
                        >
                            <span class="size-name">"Medium"</span>
                            <span class="size-price">"$14.99"</span>
                        </button>
                        <button
                            class="size-button"
                            class:selected=move || selected_size.get() == PizzaSize::Large
                            on:click=move |_| set_selected_size.set(PizzaSize::Large)
                        >
                            <span class="size-name">"Large"</span>
                            <span class="size-price">"$17.99"</span>
                        </button>
                    </div>
                </div>

                <div class="custom-actions">
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
                        disabled=move || !is_valid.get()
                    >
                        {move || if show_added_feedback.get() {
                            "✓ Added!"
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

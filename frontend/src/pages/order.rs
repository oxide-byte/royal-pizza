use chrono::{DateTime, Duration, Utc};
use leptos::{ev, prelude::*, task::spawn_local};
use leptos_router::hooks::use_navigate;
use shared::dto::{CreateOrderRequest, OrderItemRequest};
use shared::models::{CustomerInfo, OrderItemType};

use crate::api::client::create_order;
use crate::components::{error_display::ErrorDisplay, layout::PageLayout};
use crate::state::cart::{use_cart, CartItemType};
use crate::utils::format::format_currency;

#[component]
pub fn OrderPage() -> impl IntoView {
    let cart = use_cart();
    let navigate = use_navigate();

    // Redirect to menu if cart is empty
    Effect::new(move |_| {
        if cart.is_empty() {
            navigate("/", Default::default());
        }
    });

    // Form state
    let (customer_name, set_customer_name) = signal(String::new());
    let (customer_phone, set_customer_phone) = signal(String::new());
    let (pickup_date, set_pickup_date) = signal(String::new());
    let (pickup_time, set_pickup_time) = signal(String::new());
    let (validation_errors, set_validation_errors) = signal(Vec::<String>::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (api_error, set_api_error) = signal(None::<String>);

    // Get current date/time for defaults
    let now = Utc::now();
    let default_date = now.format("%Y-%m-%d").to_string();
    let default_time = (now + Duration::hours(1)).format("%H:%M").to_string();

    // Set defaults on mount
    Effect::new(move |_| {
        if pickup_date.get().is_empty() {
            set_pickup_date.set(default_date.clone());
        }
        if pickup_time.get().is_empty() {
            set_pickup_time.set(default_time.clone());
        }
    });

    let cart_total = Memo::new(move |_| cart.total());
    let cart_items = Memo::new(move |_| cart.items());

    // Validation function
    let validate_form = move || -> Vec<String> {
        use shared::validation::{
            validate_customer_name, validate_phone_number, validate_pickup_time,
        };

        let mut errors = Vec::new();

        // Validate customer name
        if let Err(e) = validate_customer_name(&customer_name.get()) {
            errors.push(e);
        }

        // Validate phone
        if let Err(e) = validate_phone_number(&customer_phone.get()) {
            errors.push(e);
        }

        // Validate pickup time
        let date = pickup_date.get();
        let time = pickup_time.get();
        if date.is_empty() || time.is_empty() {
            errors.push("Pickup date and time are required.".to_string());
        } else {
            if let Ok(pickup_datetime) =
                format!("{}T{}:00Z", date, time).parse::<DateTime<Utc>>()
            {
                if let Err(e) = validate_pickup_time(pickup_datetime) {
                    errors.push(e);
                }
            } else {
                errors.push("Invalid date or time format.".to_string());
            }
        }

        // Validate cart
        if cart.is_empty() {
            errors.push("Cart is empty. Please add items before ordering.".to_string());
        }

        errors
    };

    // Submit handler
    let submit_order = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        // Clear previous errors
        set_validation_errors.set(Vec::new());
        set_api_error.set(None);

        // Validate
        let errors = validate_form();
        if !errors.is_empty() {
            set_validation_errors.set(errors);
            return;
        }

        set_is_submitting.set(true);

        // Build request
        let customer = CustomerInfo {
            name: customer_name.get().trim().to_string(),
            phone: customer_phone.get().trim().to_string(),
        };

        let items: Vec<OrderItemRequest> = cart_items
            .get()
            .into_iter()
            .map(|cart_item| {
                let item_type = match cart_item.cart_item_type {
                    CartItemType::StandardPizza { pizza_id, size, .. } => {
                        OrderItemType::StandardPizza { pizza_id, size }
                    }
                    CartItemType::CustomPizza { custom } => OrderItemType::CustomPizza { custom },
                };
                OrderItemRequest {
                    item_type,
                    quantity: cart_item.quantity,
                }
            })
            .collect();

        let pickup_datetime_str = format!("{}T{}:00Z", pickup_date.get(), pickup_time.get());
        let pickup_datetime = match pickup_datetime_str.parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            Err(_) => {
                set_validation_errors.set(vec!["Invalid date or time format.".to_string()]);
                set_is_submitting.set(false);
                return;
            }
        };

        let request = CreateOrderRequest {
            customer,
            items,
            pickup_time: pickup_datetime,
        };

        // Call API
        spawn_local(async move {
            match create_order(request).await {
                Ok(response) => {
                    // Clear cart and navigate to confirmation
                    cart.clear();
                    let path = format!("/confirmation/{}", response.order_id);
                    leptos_router::hooks::use_navigate()(&path, Default::default());
                }
                Err(err) => {
                    set_api_error.set(Some(err.user_message()));
                    set_is_submitting.set(false);
                }
            }
        });
    };

    let remove_item = move |item_id: String| {
        cart.remove_item(&item_id);
    };

    let update_item_quantity = move |(item_id, quantity): (String, u32)| {
        cart.update_quantity(&item_id, quantity);
    };

    view! {
        <PageLayout>
            <div class="order-page">
                <div class="order-header">
                    <h2>"Complete Your Order"</h2>
                </div>

                <div class="order-content">
                    <div class="cart-review">
                        <h3>"Your Order"</h3>
                        <div class="cart-items-list">
                            {move || {
                                cart_items
                                    .get()
                                    .into_iter()
                                    .map(|item| {
                                        let item_id_for_remove = item.id.clone();
                                        let item_id_for_update = item.id.clone();
                                        let item_quantity = item.quantity;
                                        view! {
                                            <div class="cart-item">
                                                <div class="item-info">
                                                    <div class="item-name">
                                                        {item.cart_item_type.display_name()}
                                                    </div>
                                                    <div class="item-price">
                                                        {format_currency(item.unit_price)}
                                                        " x "
                                                        {item.quantity}
                                                    </div>
                                                </div>
                                                <div class="item-actions">
                                                    <input
                                                        type="number"
                                                        min="1"
                                                        max="10"
                                                        class="quantity-input"
                                                        prop:value=item_quantity
                                                        on:input=move |ev| {
                                                            if let Ok(val) = event_target_value(&ev).parse::<u32>()
                                                            {
                                                                update_item_quantity(
                                                                    (item_id_for_update.clone(), val.max(1).min(10)),
                                                                );
                                                            }
                                                        }
                                                    />

                                                    <div class="item-subtotal">
                                                        {format_currency(item.subtotal())}
                                                    </div>
                                                    <button
                                                        class="remove-button"
                                                        on:click=move |_| {
                                                            remove_item(item_id_for_remove.clone())
                                                        }
                                                    >

                                                        "×"
                                                    </button>
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </div>
                        <div class="cart-total">
                            <strong>"Total: "</strong>
                            {move || format_currency(cart_total.get())}
                        </div>
                    </div>

                    <div class="customer-form-section">
                        <h3>"Customer Information"</h3>

                        <Show when=move || !validation_errors.get().is_empty()>
                            <div class="validation-errors">
                                {move || {
                                    validation_errors
                                        .get()
                                        .into_iter()
                                        .map(|err| view! { <div class="error-item">{err}</div> })
                                        .collect::<Vec<_>>()
                                }}
                            </div>
                        </Show>

                        <ErrorDisplay error=Signal::derive(move || api_error.get()) />

                        <form class="customer-form" on:submit=submit_order>
                            <div class="form-group">
                                <label for="customer-name">"Name *"</label>
                                <input
                                    type="text"
                                    id="customer-name"
                                    placeholder="Enter your name"
                                    prop:value=move || customer_name.get()
                                    on:input=move |ev| set_customer_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-group">
                                <label for="customer-phone">"Phone *"</label>
                                <input
                                    type="tel"
                                    id="customer-phone"
                                    placeholder="(555) 123-4567"
                                    prop:value=move || customer_phone.get()
                                    on:input=move |ev| set_customer_phone.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-group">
                                <label for="pickup-date">"Pickup Date *"</label>
                                <input
                                    type="date"
                                    id="pickup-date"
                                    prop:value=move || pickup_date.get()
                                    on:input=move |ev| set_pickup_date.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-group">
                                <label for="pickup-time">"Pickup Time *"</label>
                                <input
                                    type="time"
                                    id="pickup-time"
                                    prop:value=move || pickup_time.get()
                                    on:input=move |ev| set_pickup_time.set(event_target_value(&ev))
                                />
                                <p class="form-hint">
                                    "Orders must be placed at least 30 minutes in advance."
                                </p>
                            </div>

                            <button
                                type="submit"
                                class="submit-order-button"
                                disabled=move || is_submitting.get()
                            >
                                {move || if is_submitting.get() { "Submitting..." } else { "Place Order" }}
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </PageLayout>
    }
}

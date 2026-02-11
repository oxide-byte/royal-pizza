use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};

use crate::api::client::fetch_order_by_id;
use crate::components::{error_display::ErrorDisplay, layout::PageLayout, loading::LoadingSpinner};
use crate::state::cart::use_cart;
use crate::utils::format::{format_currency, format_datetime};

#[derive(Params, PartialEq, Clone)]
struct ConfirmationParams {
    id: Option<String>,
}

#[component]
pub fn ConfirmationPage() -> impl IntoView {
    let navigate = use_navigate();
    let cart = use_cart();
    let params = use_params::<ConfirmationParams>();

    let order_id = move || {
        params.get().ok().and_then(|p| p.id).unwrap_or_default()
    };

    // Fetch order details
    let order = Resource::new(order_id, |id| async move {
        fetch_order_by_id(&id).await
    });

    let error_message = create_rw_signal(None::<String>);

    let order_another = move |_| {
        cart.clear();
        navigate("/", Default::default());
    };

    view! {
        <PageLayout>
            <div class="confirmation-page">
                <Suspense fallback=move || view! { <LoadingSpinner /> }>
                    {move || {
                        order
                            .get()
                            .map(|result| match result {
                                Ok(order_data) => {
                                    view! {
                                        <div class="confirmation-content">
                                            <div class="success-header">
                                                <div class="success-icon">"✓"</div>
                                                <h2>"Order Confirmed!"</h2>
                                                <p class="success-message">
                                                    "Thank you for your order. We're preparing your delicious pizzas!"
                                                </p>
                                            </div>

                                            <div class="order-details">
                                                <div class="detail-section">
                                                    <h3>"Order Number"</h3>
                                                    <p class="order-number">{&order_data.order_number}</p>
                                                </div>

                                                <div class="detail-section">
                                                    <h3>"Customer Information"</h3>
                                                    <p>
                                                        <strong>"Name: "</strong>
                                                        {&order_data.customer.name}
                                                    </p>
                                                    <p>
                                                        <strong>"Phone: "</strong>
                                                        {&order_data.customer.phone}
                                                    </p>
                                                </div>

                                                <div class="detail-section">
                                                    <h3>"Order Items"</h3>
                                                    <div class="order-items-list">
                                                        {order_data
                                                            .items
                                                            .iter()
                                                            .map(|item| {
                                                                let item_name = match &item.item_type {
                                                                    shared::models::OrderItemType::StandardPizza {
                                                                        pizza_id,
                                                                        size,
                                                                    } => {
                                                                        format!("Pizza {} - {:?}", pizza_id, size)
                                                                    }
                                                                    shared::models::OrderItemType::CustomPizza { custom } => {
                                                                        format!("Custom Pizza - {:?}", custom.size)
                                                                    }
                                                                };
                                                                view! {
                                                                    <div class="order-item-row">
                                                                        <span class="item-name">{item_name}</span>
                                                                        <span class="item-quantity">
                                                                            "x " {item.quantity}
                                                                        </span>
                                                                        <span class="item-price">
                                                                            {format_currency(item.subtotal)}
                                                                        </span>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()}
                                                    </div>
                                                </div>

                                                <div class="detail-section">
                                                    <h3>"Pickup Time"</h3>
                                                    <p class="pickup-time">
                                                        {format_datetime(&order_data.pickup_time)}
                                                    </p>
                                                </div>

                                                <div class="detail-section total-section">
                                                    <h3>"Total Amount"</h3>
                                                    <p class="total-amount">
                                                        {format_currency(order_data.total_amount)}
                                                    </p>
                                                </div>
                                            </div>

                                            <div class="confirmation-actions">
                                                <button class="order-another-button" on:click=order_another>
                                                    "Order Another Pizza"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                        .into_view()
                                }
                                Err(err) => {
                                    error_message.set(Some(err.user_message()));
                                    view! {
                                        <div class="confirmation-error">
                                            <ErrorDisplay error=Signal::derive(move || error_message.get()) />
                                            <button class="back-to-menu-button" on:click=order_another>
                                                "Back to Menu"
                                            </button>
                                        </div>
                                    }
                                        .into_view()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </PageLayout>
    }
}

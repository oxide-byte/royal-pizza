use leptos::prelude::*;
use shared::models::{CustomPizza, PizzaSize};

use crate::api::client::fetch_pizzas;
use crate::components::{
    custom_pizza_card::CustomPizzaCard, error_display::ErrorDisplay, layout::PageLayout,
    loading::LoadingSpinner, pizza_card::PizzaCard,
};
use crate::state::cart::{use_cart, CartItemType};

#[component]
pub fn MenuPage() -> impl IntoView {
    let cart = use_cart();

    // Fetch pizzas from API
    let pizzas = LocalResource::new(|| async move { fetch_pizzas().await });

    // Error state for API failures
    let error_message = RwSignal::new(None::<String>);

    // Handle adding standard pizza to cart
    let add_standard_pizza = move |(pizza_id, size, quantity): (String, PizzaSize, u32)| {
        pizzas.with(|result| {
            if let Some(Ok(pizza_list)) = result {
                if let Some(pizza) = pizza_list.iter().find(|p| p.id == pizza_id) {
                    let unit_price = size.get_price(&pizza.price);
                    let cart_item_type = CartItemType::StandardPizza {
                        pizza_id: pizza.id.clone(),
                        pizza_name: pizza.name.clone(),
                        size,
                    };
                    cart.add_item(cart_item_type, quantity, unit_price);
                }
            }
        });
    };

    // Handle adding custom pizza to cart
    let add_custom_pizza = move |(custom, quantity): (CustomPizza, u32)| {
        let unit_price = custom.get_price();
        let cart_item_type = CartItemType::CustomPizza { custom };
        cart.add_item(cart_item_type, quantity, unit_price);
    };

    view! {
        <PageLayout>
            <div class="menu-page">
                <div class="menu-header">
                    <h2>"Our Menu"</h2>
                    <p>"Choose from our delicious selection of pizzas or create your own!"</p>
                </div>

                <Suspense fallback=move || view! { <LoadingSpinner /> }>
                    {move || {
                        pizzas
                            .get()
                            .map(|result| match result {
                                Ok(pizza_list) => {
                                    if pizza_list.is_empty() {
                                        view! {
                                            <div class="menu-content">
                                                <div class="empty-state">
                                                    <h3>"No pizzas available at the moment"</h3>
                                                    <p>"We're working on restocking our menu. In the meantime, you can create a custom pizza!"</p>
                                                </div>
                                                <div class="custom-pizza-section">
                                                    <CustomPizzaCard on_add_to_cart=add_custom_pizza />
                                                </div>
                                            </div>
                                        }
                                        .into_any()
                                    } else {
                                        view! {
                                            <div class="menu-content">
                                                <div class="pizza-grid">
                                                    {pizza_list
                                                        .into_iter()
                                                        .map(|pizza| {
                                                            view! {
                                                                <PizzaCard
                                                                    pizza=pizza
                                                                    on_add_to_cart=add_standard_pizza
                                                                />
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </div>

                                                <div class="custom-pizza-section">
                                                    <CustomPizzaCard on_add_to_cart=add_custom_pizza />
                                                </div>
                                            </div>
                                        }
                                            .into_any()
                                    }
                                }
                                Err(err) => {
                                    error_message.set(Some(err.user_message()));
                                    view! {
                                        <ErrorDisplay error=Signal::derive(move || error_message.get()) />
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </PageLayout>
    }
}

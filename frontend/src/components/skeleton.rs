use leptos::prelude::*;

/// Skeleton loader for pizza cards
#[component]
pub fn PizzaCardSkeleton() -> impl IntoView {
    view! {
        <div class="pizza-card skeleton-card">
            <div class="skeleton skeleton-image"></div>
            <div class="pizza-info">
                <div class="skeleton skeleton-title"></div>
                <div class="skeleton skeleton-text"></div>
                <div class="skeleton skeleton-text skeleton-text-short"></div>
                <div class="skeleton skeleton-ingredients"></div>
                <div class="skeleton-prices">
                    <div class="skeleton skeleton-price"></div>
                    <div class="skeleton skeleton-price"></div>
                    <div class="skeleton skeleton-price"></div>
                </div>
                <div class="skeleton skeleton-button"></div>
            </div>
        </div>
    }
}

/// Grid of skeleton pizza cards for loading state
#[component]
pub fn PizzaGridSkeleton(#[prop(optional, default = 9)] count: usize) -> impl IntoView {
    view! {
        <div class="pizza-grid">
            {(0..count)
                .map(|_| view! { <PizzaCardSkeleton /> })
                .collect::<Vec<_>>()}
        </div>
    }
}

/// Generic skeleton component for custom layouts
#[component]
pub fn Skeleton(
    #[prop(optional)] width: Option<String>,
    #[prop(optional)] height: Option<String>,
) -> impl IntoView {
    let style = format!(
        "{}{}",
        width.map(|w| format!("width: {};", w)).unwrap_or_default(),
        height.map(|h| format!("height: {};", h)).unwrap_or_default()
    );

    view! {
        <div class="skeleton" style=style></div>
    }
}
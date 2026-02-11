use leptos::prelude::*;

#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="loading-spinner-container">
            <div class="loading-spinner"></div>
            <p>"Loading..."</p>
        </div>
    }
}

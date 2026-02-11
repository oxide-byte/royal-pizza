use leptos::prelude::*;

#[component]
pub fn ErrorDisplay(
    #[prop(into)] error: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <Show
            when=move || error.get().is_some()
            fallback=|| view! { <></> }
        >
            <div class="error-display">
                <div class="error-icon">"⚠️"</div>
                <div class="error-message">
                    {move || error.get().unwrap_or_default()}
                </div>
            </div>
        </Show>
    }
}

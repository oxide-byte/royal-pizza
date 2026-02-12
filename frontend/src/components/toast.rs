use leptos::prelude::*;
use leptos::task::spawn_local;

#[derive(Clone, Debug, PartialEq)]
pub enum ToastType {
    Success,
    Info,
    Warning,
    Error,
}

#[component]
pub fn Toast(
    message: String,
    #[prop(optional, default = ToastType::Info)] toast_type: ToastType,
    #[prop(optional, default = true)] auto_dismiss: bool,
) -> impl IntoView {
    let visible = RwSignal::new(true);

    // Auto-dismiss after 3 seconds if enabled
    if auto_dismiss {
        let visible_clone = visible;
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(3000).await;
            visible_clone.set(false);
        });
    }

    let (icon, class_name) = match toast_type {
        ToastType::Success => ("✓", "toast-success"),
        ToastType::Info => ("ℹ", "toast-info"),
        ToastType::Warning => ("⚠", "toast-warning"),
        ToastType::Error => ("✕", "toast-error"),
    };

    // Store values as owned for use in view
    let icon_owned = icon.to_string();
    let class_full = format!("toast {}", class_name);

    view! {
        <Show when=move || visible.get()>
            <div class=class_full.clone()>
                <span class="toast-icon">{icon_owned.clone()}</span>
                <span class="toast-message">{message.clone()}</span>
                <button
                    class="toast-close"
                    on:click=move |_| visible.set(false)
                    aria-label="Close notification"
                >
                    "×"
                </button>
            </div>
        </Show>
    }
}

/// Container for displaying multiple toasts
#[component]
pub fn ToastContainer(children: Children) -> impl IntoView {
    view! {
        <div class="toast-container">
            {children()}
        </div>
    }
}
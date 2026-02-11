// Royal Pizza Frontend - Leptos Application

mod api;
mod app;
mod components;
mod pages;
mod state;
mod utils;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use app::App;

/// Main entry point for the WASM application
#[wasm_bindgen(start)]
pub fn main() {
    // Enable console_error_panic_hook for better error messages in the browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Mount the app to the body
    mount_to_body(|| view! { <App /> });
}

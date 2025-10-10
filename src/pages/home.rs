use leptos::prelude::*;
use singlestage::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Button class="bg-red-400 mr-4" on:click=on_click>"Click Me: "{count}</Button>
        <Button>"Test Button"</Button>
    }
}
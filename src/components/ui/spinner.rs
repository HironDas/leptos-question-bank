use leptos::prelude::*;

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <span class="animate-spin h-4 w-4 border-2 border-blue-400 rounded-full border-t-transparent" >
        </span>
    }
}

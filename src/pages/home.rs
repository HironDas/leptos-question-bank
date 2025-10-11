use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
// use singlestage::*;


/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        
            <div class="p-4 sm:p-8">
                <h1 class="text-2xl font-bold mb-4">"Welcome to the Question Bank!"</h1>
                <p class="mb-4">
                    "This is a simple application to manage and organize your questions."
                </p>
                <A href="/">
                    "Go to Login Page"
                </A>
            </div>
            <Outlet/>
    }
}

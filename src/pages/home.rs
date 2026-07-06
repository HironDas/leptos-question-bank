use leptos::prelude::*;
use leptos_router::components::{Outlet, A};

use crate::server_function::home::home;
// use singlestage::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Blocking: home() performs the page's auth check and calls redirect()
    // on failure, which must land before the response headers are sent —
    // a non-blocking resource doesn't guarantee that.
    let auth_check = Resource::new_blocking(|| (), |_| home());
    view! {
        <Suspense fallback=|| ()>
            {move || {
                auth_check.get();
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
            }}
        </Suspense>
    }
}

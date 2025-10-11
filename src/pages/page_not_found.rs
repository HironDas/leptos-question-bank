use leptos::prelude::*;

/// Documentation for [`NotFound`].
#[component]
pub fn NotFound() -> impl IntoView {
    

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen px-4">
            <h1 class="text-9xl font-bold mb-4">"Oops!"</h1>
            <h1 class="text-2xl font-bold mb-4">"404 - Page Not Found"</h1>
            <p class="mb-4">"The page you are looking for does not exist."</p>
            <a href="/" class="text-blue-500 underline-offset-4 hover:underline">"Go back to LOGIN"</a>    
        </div>
    }
}
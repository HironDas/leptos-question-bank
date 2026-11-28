use crate::components::ui::toast::ToastProvider;
use crate::components::Layout;
use crate::pages::{AddEditQuestion, ClassSettingRoute, HomePage, UnAuthorized, ViewQuestions};
use crate::pages::{Login, NotFound, Signup};
use crate::util::auth::{is_auth, AuthContext};

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::ProtectedParentRoute;
use leptos_router::Lazy;
use leptos_router::{
    components::{Route, Router, Routes},
    // ParentRoute,
    StaticSegment,
};
use singlestage::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let auth = AuthContext::new();
    provide_context(auth);

    let authed = Resource::new(|| (), |_| is_auth());
    let academic_setting: Lazy<ClassSettingRoute> = Lazy::new();

    Effect::new(move |_| {
        if let Some(result) = authed.get() {
            match result {
                Ok(Some(value)) => auth.set_authenticated(value),
                Ok(None) => auth.authenticated.set(None),
                Err(err) => {
                    leptos::logging::error!("Authentication check failed: {:?}", err);
                    auth.authenticated.set(None)
                }
            }
        }
    });

    let condition = move || auth.is_authenticated();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-question-bank.css" />

        // sets the document title
        <Title text="Question Bank" />

        // content for this welcome page
        <ThemeProvider>
            <ToastProvider>
            <Router>
                <main>
                    <Routes fallback=move || view! { <NotFound /> }>
                        <Route path=StaticSegment("") view=Login />
                        <Route path=StaticSegment("/signup") view=Signup />
                        <Route path=StaticSegment("/unauthorized") view=UnAuthorized />
                        <Route path=StaticSegment("/health") view=move || view! {} />
                        <ProtectedParentRoute
                        redirect_path = ||"/unauthorized"
                        condition=condition
                        path=StaticSegment("") view=Layout>
                            <Route path=StaticSegment("/home") view=HomePage />
                            // <Route path=StaticSegment("academic-setting") view=ClassSetting />
                            <Route path=StaticSegment("academic-setting") view=academic_setting />
                            <Route path=StaticSegment("/view") view=ViewQuestions />
                            <Route path=StaticSegment("/question") view=AddEditQuestion />
                        </ProtectedParentRoute>
                    </Routes>
                </main>
            </Router>
            </ToastProvider>
        </ThemeProvider>
    }
}

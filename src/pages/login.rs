use leptos::{logging::log, prelude::*};
use singlestage::*;
// use web_sys::SubmitEvent;

use crate::{components::ui::spinner::Spinner, error::AppError, server_function::login::Login};

/// Matches the per-account lockout window in `UserService::login` (5 attempts / 900s).
const LOCKOUT_SECS: i64 = 15 * 60;

#[component]
pub fn Login() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let pending = login.pending();
    let value = login.value();
    let error_msg = RwSignal::new("".to_string());

    // Unix timestamp (seconds) until which login is locked after a RateLimited error.
    let locked_until = RwSignal::new(None::<i64>);
    // Current time, ticked every second on the client while the page is mounted.
    let now = RwSignal::new(chrono::Utc::now().timestamp());
    let remaining = move || {
        locked_until
            .get()
            .map(|until| (until - now.get()).max(0))
            .unwrap_or(0)
    };
    let is_locked = move || remaining() > 0;

    // Effects only run in the browser, so the interval is never created during SSR.
    Effect::new(move || {
        if let Ok(handle) = set_interval_with_handle(
            move || now.set(chrono::Utc::now().timestamp()),
            std::time::Duration::from_secs(1),
        ) {
            on_cleanup(move || handle.clear());
        }
    });

    // Clear the lock and its message once the countdown reaches zero.
    Effect::new(move || {
        if locked_until.get().is_some() && !is_locked() {
            locked_until.set(None);
            error_msg.set("".to_string());
        }
    });

    Effect::new(move || {
        if let Some(result) = value.get() {
            match result {
                Err(AppError::RateLimited) => {
                    locked_until.set(Some(chrono::Utc::now().timestamp() + LOCKOUT_SECS));
                    error_msg.set("Too many login attempts.".to_string());
                }
                Err(err) => {
                    log!("err-->: {:#?}", err);
                    error_msg.set(err.to_string());
                }
                Ok(_) => {
                    error_msg.set("".to_string());
                    let _ = window().location().set_href("/home");
                }
            }
        }
    });

    let countdown = move || {
        let secs = remaining();
        format!("Please try again in {:02}:{:02}", secs / 60, secs % 60)
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen px-4">
        <ActionForm action=login >
        <Card class="w-full sm:w-sm">
            <CardHeader>
                <CardTitle>"Log in to your account"</CardTitle>
                <CardDescription>
                    "Enter your details below to log in to your account"
                </CardDescription>
            </CardHeader>
            <CardContent>
                <div class="form grid gap-6">
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-email">"Username/Email"</Label>
                        <Input input_type="text" name="login[id]" />
                    </div>
                    <div class="grid gap-2">
                        // <div class="flex items-center gap-2">
                            // <a
                            //     href="#"
                            //     class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                            // >
                            //     "Forgot your password?"
                            // </a>
                        // </div>
                        <Label label_for="demo-card-form-password">"Password"</Label>
                        <Input input_type="password" name="login[password]" />
                    </div>
                </div>
            </CardContent>
            <CardFooter class="flex flex-col items-center gap-2">
                <Button button_type="submit" class="w-full" attr:disabled = move||pending.get() || is_locked()>
                <Show when = move ||pending.get() fallback= ||view!{"Log in"}>
                "Processing..."<Spinner />
                </Show>
                </Button>
                <Show when= move||error_msg.get() !="" fallback=||view!{""}>
                    <Alert variant="destructive">
                        {icon!(icondata::FiAlertTriangle)}
                        <AlertTitle>"Something went wrong!"</AlertTitle>
                        <AlertDescription>
                            {error_msg}
                            <Show when=is_locked>
                                <span class="block font-mono">{countdown}</span>
                            </Show>
                        </AlertDescription>
                    </Alert>
                </Show>
                <p class="mt-4 text-center text-sm">
                    "Don't have an account? "<a href="/signup" class="underline-offset-4 hover:underline">
                    "Sign up "
                    </a>
                </p>
            </CardFooter>
        </Card>
        </ActionForm>
        </div>
    }
}

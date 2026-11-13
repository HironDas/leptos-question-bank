use leptos::{logging::log, prelude::*};
use singlestage::*;
// use web_sys::SubmitEvent;

use crate::{components::ui::spinner::Spinner, server_function::login::Login};

#[component]
pub fn Login() -> impl IntoView {
    let login = ServerAction::<Login>::new();
    let pending = login.pending();
    let value = login.value();
    let error_msg = RwSignal::new("".to_string());

    Effect::new(move || {
        if value.get().is_some() {
            if let Err(err) = value.get().unwrap() {
                log!("err-->: {:#?}", err);
                error_msg.set(err.to_string());
            } else {
                error_msg.set("".to_string());
            }
        }
    });

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
                <Button button_type="submit" class="w-full" attr:disabled = move||pending.get()>
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

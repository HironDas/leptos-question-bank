use crate::components::ui::spinner::Spinner;
use crate::server_function::signup::*;

use leptos::{logging::log, prelude::*};
use leptos_router::{lazy_route, LazyRoute};
use singlestage::*;

#[component]
pub fn Signup() -> impl IntoView {
    let has_reset = RwSignal::new(false);
    let signup = ServerAction::<SignupAction>::new();
    let value = signup.value();
    let pending = signup.pending();
    let has_error = move || value.with(|v| v.is_some() && matches!(v, Some(Err(_))));

    Effect::new(move || {
        if !has_reset.get() {
            signup.clear();
            has_reset.set(true);
        }
    });

    log!("Error: {}", has_error());
    log!("Pending: {}", pending.get());
    log!("Value: {:?}", value.get());
    log!("reset: {:?}", has_reset.get());

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen px-4">
        <ActionForm action=signup>
            <Card class="w-full sm:w-sm">
                <CardHeader>
                    <CardTitle>"Create new account"</CardTitle>
                    <CardDescription>
                        "Enter your details below to create a new account"
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <div class="form grid gap-6">
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-username">"Username"</Label>
                            <Input name="user[username]" input_type="text" id="demo-card-form-username" />
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-email">"Email"</Label>
                            <Input name="user[email]" input_type="email" id="demo-card-form-email" />
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-password">"Password"</Label>
                            <Input name="user[password]" input_type="password" id="demo-card-form-password" />
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-cpassword">"Confirm Password"</Label>
                            <Input name="user[confirm_password]" input_type="password" id="demo-card-form-cpassword" />
                        </div>
                    </div>
                </CardContent>
                <CardFooter class="flex flex-col items-center gap-2">
                    <Button button_type="submit" class="w-full" attr:disabled=move|| pending.get()>
                        "Sign up"
                        <Show when = move ||pending.get() fallback= ||view!{""}>
                        <Spinner />
                        </Show>
                    </Button>
                    <Show when= move|| has_error() fallback=||view!{""}>
                        <Alert variant="destructive">
                            {icon!(icondata::FiAlertTriangle)}
                            <AlertTitle>"Something went wrong!"</AlertTitle>
                            <AlertDescription>
                                "Signup Failed! Please fill all fields correctly."
                            </AlertDescription>
                        </Alert>
                    </Show>
                </CardFooter>
            </Card>
        </ActionForm>
        </div>
    }
}

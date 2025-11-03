use crate::domain::user_email::UserEmail;
use crate::server_function::signup::*;
use crate::{components::ui::spinner::Spinner, domain::user_username::Username};

use leptos::{logging::log, prelude::*};
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

    let username = RwSignal::new(String::from(""));
    let email = RwSignal::new(String::from(""));

    let debounced_username = Memo::new(move |_| {
        let username = username.get();
        Username::parse(username)
    });

    let debounced_email = Memo::new(move |_| {
        let email = email.get();
        UserEmail::parse(email)
    });

    let check_user = Resource::new(
        move || debounced_username.get(),
        |name| async move {
            // log!("Checking username: {:?}", name);
            match name {
                Ok(name) => {
                    let username = name.as_ref().to_string();
                    match is_user_taken(username).await {
                        Ok(true) => (false, "Username already taken".to_string()),
                        Ok(false) => (true, "".to_string()),
                        Err(_err) => (false, "Internal server error".to_string()),
                    }
                }
                Err(err) => {
                    // log!("Error: {}", err.message.as_ref().unwrap());
                    (false, err.message.as_ref().unwrap().to_string())
                }
            }
        },
    );

    let check_email = Resource::new(
        move || debounced_email.get(),
        move |email| async move {
            match email {
                Ok(email) => match is_email_exists(email.as_ref().to_string()).await {
                    Ok(true) => (false, "Email already exists".to_string()),
                    Ok(false) => (true, "".to_string()),
                    Err(_err) => (false, "Internal server error".to_string()),
                },
                Err(err) => {
                    // log!("Error: {}", err.message.as_ref().unwrap());
                    (false, err.message.as_ref().unwrap().to_string())
                }
            }
        },
    );

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
                            <Input on:keyup = move |event| {
                                let value = event_target_value(&event);
                                username.set(value);
                            } name="user[username]" input_type="text" id="demo-card-form-username" />
                            <Show when=move||!username.get().is_empty() && !check_user.get().unwrap().0 fallback={move||view!{<p>""</p>}}>
                            <p class="text-(--destructive) text-xs">{check_user.get().unwrap().1}</p>
                            </Show>
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-email">"Email"</Label>
                            <Input on:blur = move |event| {
                                let value = event_target_value(&event);
                                email.set(value);
                            } name="user[email]" input_type="email" id="demo-card-form-email" />
                            <Show when=move||!email.get().is_empty() && !check_email.get().unwrap().0 fallback={move||view!{<p class="text-xs">""</p>}}>
                                <p class="text-(--destructive) text-xs">{check_email.get().unwrap().1}</p>
                            </Show>
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
                        <Show when = move ||pending.get() fallback= ||view!{"Sign up"}>
                        "Processing..."<Spinner />
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

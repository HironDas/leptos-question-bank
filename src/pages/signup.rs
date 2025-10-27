use leptos::prelude::*;
use singlestage::*;
use crate::server_function::signup::*;

#[component]
pub fn Signup() -> impl IntoView {
    let signup = ServerAction::<Signup>::new();
    let _value = signup.value();

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
                            <Input name="username" input_type="text" id="demo-card-form-username" />
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-email">"Email"</Label>
                            <Input name="email" input_type="email" id="demo-card-form-email" />
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-password">"Password"</Label>
                            <Input name="password" input_type="password" id="demo-card-form-password" />
                        </div>
                        <div class="grid gap-2">
                            <Label label_for="demo-card-form-cpassword">"Confirm Password"</Label>
                            <Input name="confirm_password" input_type="password" id="demo-card-form-cpassword" />
                        </div>
                    </div>
                </CardContent>
                <CardFooter class="flex flex-col items-center gap-2">
                    <Button button_type="submit" class="w-full">
                        "Sign up"
                    </Button>
                </CardFooter>
            </Card>
        </ActionForm>
        </div>
    }
}

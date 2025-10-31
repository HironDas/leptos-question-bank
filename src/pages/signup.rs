use crate::server_function::signup::*;
use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};
use singlestage::*;

#[derive(Debug)]
pub struct Signup;

#[lazy_route]
impl LazyRoute for Signup {
    fn data() -> Self {
        Self
    }
    fn view(_this: Self) -> AnyView {
        let signup = ServerAction::<SignupAction>::new();
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
                        <Button button_type="submit" class="w-full">
                            "Sign up" <span class="animate-spin h-10 w-10 border-4 border-blue-500 rounded-full border-t-transparent"></span>
                        </Button>
                        <Show when=move||true fallback=||view!{""}>
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
        }.into_any()
    }
}

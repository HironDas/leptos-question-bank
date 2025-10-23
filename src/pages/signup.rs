use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn Signup() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen px-4">
        <Card class="w-full sm:w-sm">
            <CardHeader>
                <CardTitle>"Create new account"</CardTitle>
                <CardDescription>
                    "Enter your details below to create a new account"
                </CardDescription>
            </CardHeader>
            <CardContent>
                <form class="form grid gap-6">
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-username">"Username"</Label>
                        <Input input_type="text" id="demo-card-form-username" />
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-email">"Email"</Label>
                        <Input input_type="email" id="demo-card-form-email" />
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-password">"Password"</Label>
                        <Input input_type="password" id="demo-card-form-password" />
                    </div>
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-cpassword">"Confirm Password"</Label>
                        <Input input_type="password" id="demo-card-form-cpassword" />
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex flex-col items-center gap-2">
                <Button button_type="button" class="w-full">
                    "Sign up"
                </Button>
            </CardFooter>
        </Card>
        </div>
    }
}

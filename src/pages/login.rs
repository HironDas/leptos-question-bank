use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center min-h-screen px-4">
        <Card class="w-full sm:w-sm">
            <CardHeader>
                <CardTitle>"Log in to your account"</CardTitle>
                <CardDescription>
                    "Enter your details below to log in to your account"
                </CardDescription>
            </CardHeader>
            <CardContent>
                <form class="form grid gap-6">
                    <div class="grid gap-2">
                        <Label label_for="demo-card-form-email">"Email"</Label>
                        <Input input_type="email" id="demo-card-form-email" />
                    </div>
                    <div class="grid gap-2">
                        <div class="flex items-center gap-2">
                            <Label label_for="demo-card-form-password">"Password"</Label>
                            <a
                                href="#"
                                class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                            >
                                "Forgot your password?"
                            </a>
                        </div>
                        <Input input_type="password" id="demo-card-form-password" />
                    </div>
                </form>
            </CardContent>
            <CardFooter class="flex flex-col items-center gap-2">
                <Button button_type="button" class="w-full">
                    "Log in"
                </Button>
                <p class="mt-4 text-center text-sm">
                    "Don't have an account? "<a href="#" class="underline-offset-4 hover:underline">
                        "Sign up"
                    </a>
                </p>
            </CardFooter>
        </Card>
        </div>
        
    }
}
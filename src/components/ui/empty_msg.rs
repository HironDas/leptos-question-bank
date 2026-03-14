use icondata::{self, Icon};
use leptos::prelude::*;
use singlestage::*;

#[component]
pub fn EmptyMsg(
    icon: Icon,
    title: String,
    description: String,
    button_text: String,
    button_on_click: impl FnMut(web_sys::MouseEvent) + Send + 'static,
) -> impl IntoView {
    view! {<Empty>
        <EmptyHeader>
            <EmptyMedia variant="icon">{icon!(icon)}</EmptyMedia>
            <EmptyTitle>{title}</EmptyTitle>
            <EmptyDescription>
                {description}
            </EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
            <div class="flex gap-2">
                <Button on:click=button_on_click>{button_text}</Button>
                // <Button variant="outline">"Import Project"</Button>
            </div>
        </EmptyContent>
        // <Link as_button=true size="sm" class="text-(--muted-foreground)" href="#">
        //     "Learn More"
        //     {icon!(icondata::LuArrowUpRight)}
        // </Link>
    </Empty>}
}

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::components::Outlet;
use singlestage::*;

use crate::components::ThemeSwitcher;

#[component]
fn SidebarButton() -> impl IntoView {
    let sidebar = expect_context::<SidebarContext>();

    view! {
        <Tooltip side="bottom" align="start" value="Toggle sidebar">
            <Button variant="ghost" size="sm-icon">
                <Show
                    when=move || { sidebar.hidden.get() }
                    fallback=move || match sidebar.side.get().as_str() {
                        "right" => view! { {icon!(icondata::LuPanelRightClose)} }.into_any(),
                        _ => view! { {icon!(icondata::LuPanelLeftClose)} }.into_any(),
                    }
                >
                    {match sidebar.side.get().as_str() {
                        "right" => view! { {icon!(icondata::LuPanelRightOpen)} }.into_any(),
                        _ => view! { {icon!(icondata::LuPanelLeftOpen)} }.into_any(),
                    }}
                </Show>
            </Button>
        </Tooltip>
    }
}

#[component]
pub fn Layout() -> impl IntoView {
    view! {
     <SidebarProvider>
        <Sidebar class="inset-shadow-sm">
            <SidebarHeader>
            <A href="/home">
                <div class="flex m-2">
                    <div class="flex items-center justify-center bg-(--primary)
                        text-(--primary-foreground) min-w-12 max-w-12 min-h-12 max-h-12
                        rounded-md shadow-sm font-bold text-3xl">"Q"
                    </div>
                    <div class="content-center ml-2">
                        <p class="text-center font-semibold">"Question Bank"</p>
                        // <p class="text-center text-sm">"v"{env!("CARGO_PKG_VERSION")}</p>
                    </div>
                </div>
            </A>
            </SidebarHeader>
            <SidebarContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <A href="/home">
                                {icon!(icondata::LuLayoutDashboard)} <span>"Dashboard"</span>
                            </A>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <A href="/set-question">
                                {icon!(icondata::AiFileExclamationOutlined)} <span>"Set Question"</span>
                            </A>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <A href="/view">
                                {icon!(icondata::AiFileSearchOutlined)}
                                <span>"View Questions"</span>
                            </A>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <A href="/history">
                                {icon!(icondata::AiHistoryOutlined)} <span>"Exam History"</span>
                            </A>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
                <SidebarSeparator/>
                </SidebarContent>
                <SidebarFooter>
                    <div class="content-center ml-2 bg-(--primary) rounded-sm">
                        <p class="text-center text-sm text-(--primary-foreground)">"v-"{env!("CARGO_PKG_VERSION")}</p>
                        <p class="text-center text-xs text-(--primary-foreground)">"© 2025 hcdas@outlook.com"</p>
                    </div>
                </SidebarFooter>
        </Sidebar>
        <main>
            <header class="bg-(--background) sticky inset-x-0 top-0 isolate flex shrink-0
            items-center gap-2 border-b z-10">
                <div class="flex h-14 w-full items-center justify-between gap-2 px-4">
                    <SidebarTrigger>
                        <SidebarButton />
                    </SidebarTrigger>
                    <ThemeSwitcher/>
                </div>
            </header>
            <div class="mx-2 sm:mx-12 my-8">
                <Outlet />
            </div>
        </main>
     </SidebarProvider>
    }
}

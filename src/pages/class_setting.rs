use icondata::{LuColumns3, LuListTree, LuPlus};
use leptos::{html, logging::log, prelude::*};
use leptos_router::{lazy_route, LazyRoute};
// use leptos_drag_reorder::provide_drag_reorder;
use crate::pages::class_setting::column_view::ColumnView;
use crate::{
    components::ui::empty_msg::EmptyMsg,
    server_function::{
        academic_helper::class::{AddClass, UpdateClass},
        academic_setting::{academic_setting, Class},
    },
};
use singlestage::*;

// use crate::server_function::academic_setting::Class;

#[derive(Debug)]
pub struct ClassSettingRoute {
    classes_resource: Resource<Vec<Class>>,
}

#[lazy_route]
impl LazyRoute for ClassSettingRoute {
    fn data() -> Self {
        let classes_resource = Resource::new(
            move || (),
            move |_| async move {
                match academic_setting().await {
                    Ok(data) => data,
                    Err(err) => {
                        log!("Error fetching classes: {:?}", err);
                        vec![]
                    }
                }
            },
        );

        ClassSettingRoute { classes_resource }
    }

    fn view(this: Self) -> AnyView {
        let class_dialog_open = RwSignal::new(false);
        let add_class_action = ServerAction::<AddClass>::new();

        let view = RwSignal::new("tree");

        // --- Derived: Button barinat for view button ---
        let tree_btn_variant = Memo::new(move |_| {
            if view.get() == "tree" {
                "default".to_string()
            } else {
                "outline".to_string()
            }
        });

        let column_btn_variant = Memo::new(move |_| {
            if view.get() == "column" {
                "default".to_string()
            } else {
                "outline".to_string()
            }
        });

        let class_name_ref = NodeRef::<html::Input>::new();
        let class_name_bn_ref = NodeRef::<html::Input>::new();

        let value = add_class_action.value();
        let ClassSettingRoute { classes_resource } = this;
        let classes = RwSignal::new(Vec::<Class>::new());

        let update_class_action = ServerAction::<UpdateClass>::new();

        // Derived memo for Card class to avoid passing a closure directly
        let card_class = Memo::new(move |_| {
            if view.get() == "tree" {
                "w-full md:w-2/3 lg:w-1/2 mx-auto".to_string()
            } else {
                "w-full".to_string()
            }
        });

        Effect::new(move || {
            if value.get().map(|v| v.is_ok()).unwrap_or(false) {
                class_dialog_open.set(false);
                // classes_resource.refetch();

                if let Some(Ok(new_class)) = value.get() {
                    log!("New class received in Effect: {:?}", new_class);
                    // Update the classes RwSignal with the new class
                    classes.update(|classes_vec| {
                        classes_vec.push(new_class);
                        // Optional: If you want to keep the "order" consistent
                        classes_vec.sort_by_key(|c| c.order);
                    });
                }
            }

            if update_class_action
                .value()
                .get()
                .map(|v| v.is_ok())
                .unwrap_or(false)
            {
                // 1. Get the updated class from the Action
                if let Some(Ok(updated_class)) = update_class_action.value().get() {
                    log!("Updated class received in Effect: {:?}", updated_class);
                    // 2. Use .update() on the RwSignal
                    classes.update(|classes_vec| {
                        // Use .iter().position() on the internal Vec
                        // 'c' here will be &Class
                        let index = classes_vec.iter().position(|c| c.id == updated_class.id);

                        if let Some(idx) = index {
                            // Update only this specific index
                            classes_vec[idx] = updated_class;
                        }
                    });

                    log!("Updated: {:?}", classes.get());
                }
            }
        });

        view! {
            <Transition fallback=move || {
                view! { <PageSkeleton /> }
            }>
                {move || Suspend::new(async move {
                    let data = classes_resource.await;
                    log!("Pulling data: {:?}", data);
                    classes.set(data);

                    view! {
                        <Show
                            when=move || !classes.get().is_empty()
                            fallback=move || {
                                view! {
                                    <EmptyMsg
                                        button_on_click=move |_| class_dialog_open.set(true)
                                        button_text="Create Class".to_string()
                                        icon=icondata::LuSchool
                                        title=String::from("No Classes Yet")
                                        description=String::from(
                                            "You haven't created any classes yet. Get started by creating your first class.",
                                        )
                                    />
                                }
                            }
                        >
                            <Card class=card_class>
                                <div class="px-4 flex justify-between">
                                <div class="flex gap-2">
                                    <Button size="small" variant=tree_btn_variant on:click=move|_|view.set("tree")>
                                        {icon!(LuListTree)}
                                    </Button>
                                    <Button size="small" variant=column_btn_variant on:click=move|_| view.set("column")>
                                        {icon!(LuColumns3)}
                                    </Button>
                                </div>
                                    <Button
                                        size="small"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            class_name_ref
                                                .get()
                                                .map(|input| {
                                                    input.set_value("");
                                                });
                                            class_name_bn_ref
                                                .get()
                                                .map(|input| {
                                                    input.set_value("");
                                                });
                                            class_dialog_open.set(true);
                                        }
                                    >
                                        {icon!(LuPlus)}
                                        "Class"
                                    </Button>
                                </div>
                                <Show when=move||view.get()=="tree" fallback=move|| view!{<ColumnView classes=classes/>}>
                                <Accordion class="w-full">
                                    <For
                                        each=move || classes.get()
                                        key=|class| class.id
                                        children=move |class_item| {
                                            let id = class_item.id;
                                            let memoized_class = Memo::new(move |_| {
                                                classes
                                                    .get()
                                                    .into_iter()
                                                    .find(|c| c.id == id)
                                                    .unwrap_or(class_item.clone())
                                            });
                                            // Capture the ID so the memo can find this specific item later

                                            // Create a memo that tracks the specific class in the main RwSignal

                                            view! {
                                                <ClassAccordionItem
                                                    class=memoized_class
                                                    update_class_action=update_class_action
                                                />
                                            }
                                        }
                                    />
                                </Accordion>
                                </Show>
                            </Card>
                        </Show>

                        <Dialog open=class_dialog_open>
                            <DialogTrigger slot>
                                <Button size="small" class="hidden">
                                    {icon!(LuPlus)}
                                    "Class"
                                </Button>
                            </DialogTrigger>
                            <DialogClose />
                            <DialogHeader>
                                <DialogTitle>"Create New Class"</DialogTitle>
                                <DialogDescription>
                                    "Add a new class to the academic settings."
                                </DialogDescription>
                            </DialogHeader>
                            <ActionForm action=add_class_action>
                                <DialogContent>
                                    <div class="form">
                                        <div class="grid gap-4">
                                            <div class="grid gap-2">
                                                <Label label_for="class_name">"Class Name (English)"</Label>
                                                <InputGroup>
                                                    <Input
                                                        node_ref=class_name_ref
                                                        name="class[name]"
                                                        input_type="text"
                                                        placeholder="Enter class name in English"
                                                        id="class_name"
                                                    />
                                                    <InputGroupAddon align="inline-end">
                                                        <Tooltip value="Class name should be less than 50 characters">
                                                            <Button variant="ghost" aria_label="Info" size="icon-xs">
                                                                {icon!(icondata::LuInfo)}
                                                            </Button>
                                                        </Tooltip>
                                                    </InputGroupAddon>
                                                </InputGroup>
                                            </div>
                                            <div class="grid gap-2">
                                                <Label label_for="class_name_bn">
                                                    "Class Name (বাংলা)"
                                                </Label>
                                                <InputGroup>
                                                    <Input
                                                        node_ref=class_name_bn_ref
                                                        name="class[name_bn]"
                                                        input_type="text"
                                                        placeholder="Enter class name in Bangla"
                                                        id="class_name_bn"
                                                    />
                                                    <InputGroupAddon align="inline-end">
                                                        <Tooltip value="Class name should be less than 120 characters">
                                                            <Button variant="ghost" aria_label="Info" size="icon-xs">
                                                                {icon!(icondata::LuInfo)}
                                                            </Button>
                                                        </Tooltip>
                                                    </InputGroupAddon>
                                                </InputGroup>
                                            </div>
                                        </div>
                                    </div>
                                </DialogContent>

                                <footer class="justify-end flex-row gap-2 flex form mt-4">
                                    <Button
                                        size="small"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            class_dialog_open.set(false);
                                        }
                                        variant="outline"
                                    >
                                        "Cancel"
                                    </Button>
                                    <Button
                                        size="small"
                                        button_type="submit"
                                        attr:disabled=move || add_class_action.pending().get()
                                    >

                                        <Show
                                            when=move || add_class_action.pending().get()
                                            fallback=|| view! { "Create Class" }
                                        >
                                            "Processing..."
                                            <Spinner />
                                        </Show>
                                    </Button>
                                </footer>

                            </ActionForm>
                        </Dialog>
                    }
                })}

            </Transition>
        }.into_any()
    }
}

/// Documentation for [`PageSkeleton`]
#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <Card class="w-full md:w-2/3 lg:w-1/2 mx-auto h-[270px]">
            <div class="px-4 flex justify-between">
                <Skeleton class="h-8 w-[38px] rounded-md" />
                <Skeleton class="h-8 w-[78.76px] rounded-md" />
            </div>

            <div class="px-6">
                <Skeleton class="h-9 w-full rounded-md" />
                <Separator />
                <Skeleton class="h-9 w-full rounded-md mt-2" />
                <Separator />
                <Skeleton class="h-9 w-full rounded-md mt-2" />
            </div>

        </Card>
    }
}

pub mod chapter_item;
pub mod class_item;
pub mod column_view;
pub mod subject_item;

use class_item::ClassAccordionItem;

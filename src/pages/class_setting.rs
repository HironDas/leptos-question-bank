use icondata::{AiEditOutlined, LuPlus, VsListTree};
use leptos::{html, logging::log, prelude::*};
use leptos_router::{lazy_route, LazyRoute};
// use leptos_drag_reorder::provide_drag_reorder;
use singlestage::*;

use crate::{
    components::ui::empty_msg::EmptyMsg,
    server_function::{
        academic_helper::{
            class::{AddClass, UpdateClass},
            subject::{subject, AddSubject},
        },
        academic_setting::{academic_setting, Class, Subject},
    },
};

// use crate::server_function::academic_setting::Class;

#[derive(Debug)]
pub struct ClassSettingRoute {
    classes: Resource<Vec<Class>>,
}

#[lazy_route]
impl LazyRoute for ClassSettingRoute {
    fn data() -> Self {
        let classes = Resource::new(
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

        ClassSettingRoute { classes }
    }

    fn view(this: Self) -> AnyView {
        let class_dialog_open = RwSignal::new(false);
        let add_class_action = ServerAction::<AddClass>::new();

        let class_name_ref = NodeRef::<html::Input>::new();
        let class_name_bn_ref = NodeRef::<html::Input>::new();

        let value = add_class_action.value();
        let ClassSettingRoute { classes } = this;

        // let pending = addClassAction.pending();
        Effect::new(move || {
            if value.get().map(|v| v.is_ok()).unwrap_or(false) {
                class_dialog_open.set(false);
                classes.refetch();
            }
        });

        view! {
            <Suspense fallback=move || {
                view! { <PageSkeleton /> }
            }>
                {move || Suspend::new(async move {
                    let classes_data = classes.await;
                    log!("Classes data: {:?}", classes_data);

                    view! {
                        <Show
                            when=move || !classes_data.is_empty()
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
                            <Card class="w-full md:w-1/2 lg:w-1/3 mx-auto">
                                <div class="px-4 flex justify-between">
                                    <Button size="small" variant="outline">
                                        {icon!(VsListTree)}
                                    </Button>
                                    <Button
                                        size="small"
                                        on:click=move |_| {
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
                                <Accordion class="w-full">
                                    <For
                                        each=move || classes.get().unwrap()
                                        key=|class| class.id.clone()
                                        let(class)
                                    >
                                        <ClassAccordionItem class=class />
                                    </For>
                                </Accordion>
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

            </Suspense>
        }.into_any()
    }
}

#[component]
fn ClassAccordionItem(class: Class) -> impl IntoView {
    let class = RwSignal::new(class);
    let class_accordion_open = RwSignal::new(false);

    let class_dialog_update_open = RwSignal::new(false);

    let subject_dialog_open = RwSignal::new(false);

    let update_class_action = ServerAction::<UpdateClass>::new();
    let update_class_value = update_class_action.value();

    

    let add_subject_action = ServerAction::<AddSubject>::new();
    let subject_value = add_subject_action.value();
    let input_ref = NodeRef::<html::Input>::new();

    Effect::new(move || {
        if subject_value.get().map(|v| v.is_ok()).unwrap_or(false) {
            subject_dialog_open.set(false);
        }
    });

    let subjects: Resource<Vec<Subject>> = Resource::new(
        move || (class_accordion_open.get(), subject_value.get()),
        move |(expand, _)| async move {
            log!("ClassAccordionItem expand: {}", expand);
            if expand {
                log!("Fetching subjects for class id: {}", class.get().id);
                match subject(class.get().id).await {
                    Ok(data) => data,
                    Err(err) => {
                        log!("Error fetching subjects: {:?}", err);
                        vec![]
                    }
                }
            } else {
                vec![]
            }
        },
    );
    view! {
        <AccordionItem
            draggable=true
            name=format!("class-{}", class.get().id)
            open=class_accordion_open
        >
            <AccordionTrigger
                class="font-bold pb-2"
                on:click=move |ev| {
                    ev.prevent_default();
                    class_accordion_open.set(!class_accordion_open.get_untracked());
                }
            >
                <div class="flex items-center justify-between w-full">
                    <div class="flex h-5 items-center space-x-2 text-sm">
                        <span>{class.get().name}</span>
                        <Separator vertical=true />
                        <span class="font-bengali">{class.get().name_bn}</span>
                    </div>

                    <div class="flex">
                        <Dialog open=subject_dialog_open>
                            <DialogTrigger slot>
                                <Tooltip value="Add Subject">
                                    <Button
                                        variant="ghost"
                                        size="small"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            input_ref
                                                .get()
                                                .map(|input| {
                                                    input.set_value("");
                                                });
                                            subject_dialog_open.set(true);
                                        }
                                    >
                                        {icon!(LuPlus)}
                                    </Button>
                                </Tooltip>
                            </DialogTrigger>
                            <DialogClose />
                            <DialogHeader>
                                <DialogTitle>"Add Subject"</DialogTitle>
                                <DialogDescription>
                                    "Add a new subject to the class."
                                </DialogDescription>
                            </DialogHeader>
                            <ActionForm action=add_subject_action>
                                <DialogContent>
                                    <div class="form">
                                        <div class="grid gap-4">

                                            <Input
                                                name="class_id"
                                                input_type="hidden"
                                                value=class.get().id.to_string()
                                            />

                                            <div class="grid gap-2">
                                                <Label label_for="subject_title">"Title"</Label>
                                                <InputGroup>
                                                    <Input
                                                        name="title"
                                                        input_type="text"
                                                        placeholder="Enter subject title"
                                                        node_ref=input_ref
                                                    />
                                                    // prop:node_ref=input_refan
                                                    // input_node_ref=input_ref
                                                    <InputGroupAddon align="inline-end">
                                                        <Tooltip value="Subject title should be less than 120 characters">
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
                                            subject_dialog_open.set(false);
                                        }
                                        variant="outline"
                                    >
                                        "Cancel"
                                    </Button>
                                    <Button
                                        size="small"
                                        button_type="submit"
                                        attr:disabled=move || add_subject_action.pending().get()
                                    >

                                        <Show
                                            when=move || add_subject_action.pending().get()
                                            fallback=|| view! { "Create Subject" }
                                        >
                                            "Processing..."
                                            <Spinner />
                                        </Show>
                                    </Button>
                                </footer>

                            </ActionForm>
                        </Dialog>

                        <Dialog open=class_dialog_update_open>
                            <DialogTrigger slot>
                                <Tooltip value="Edit Class">
                                    <Button
                                        variant="ghost"
                                        size="small"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            class_dialog_update_open.set(true);
                                        }
                                    >
                                        {icon!(AiEditOutlined)}
                                    </Button>
                                </Tooltip>
                            </DialogTrigger>

                            <DialogClose />

                            <DialogHeader>
                                <DialogTitle>"Edit Class"</DialogTitle>
                                <DialogDescription>
                                    "Update the class to the academic settings."
                                </DialogDescription>
                            </DialogHeader>

                            <ActionForm action=update_class_action>
                                <DialogContent>
                                    <div class="form grid gap-4">
                                        <div class="grid gap-2">
                                            <Input
                                                name="class[class_id]"
                                                input_type="hidden"
                                                value=class.get().id.to_string()
                                            />
                                            <Label label_for="class_name">"Class Name (English)"</Label>
                                            <InputGroup>
                                                <Input
                                                    id="class_name"
                                                    value=class.get().name
                                                    name="class[name]"
                                                    input_type="text"
                                                    placeholder="Enter class name in English"
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
                                                    id="class_name_bn"
                                                    value=class.get().name_bn
                                                    name="class[name_bn]"
                                                    input_type="text"
                                                    placeholder="Enter class name in Bangla"
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
                                </DialogContent>

                                <footer class="justify-end flex-row gap-2 flex form mt-4">
                                    <Button
                                        size="small"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            class_dialog_update_open.set(false);
                                        }
                                        variant="outline"
                                    >
                                        "Cancel"
                                    </Button>
                                    <Button
                                        size="small"
                                        button_type="submit"
                                        attr:disabled=move || update_class_action.pending().get()
                                    >

                                        <Show
                                            when=move || update_class_action.pending().get()
                                            fallback=|| view! { "Update Class" }
                                        >
                                            "Processing..."
                                            <Spinner />
                                        </Show>
                                    </Button>
                                </footer>
                            </ActionForm>
                        </Dialog>

                    </div>
                </div>

            </AccordionTrigger>
            <AccordionContent class="pb-0">
                <Suspense fallback=|| {
                    view! {
                        <div class="flex justify-center items-center mb-2">
                            <Spinner />
                        </div>
                    }
                }>

                    <Show
                        when=move || !subjects.get().unwrap_or_default().is_empty()
                        fallback=move || {
                            view! { <div class="text-gray-400 pb-2">"No Subject to show!"</div> }
                        }
                    >
                        <Accordion class="pb-0">
                            <For
                                each=move || subjects.get().unwrap_or_default()
                                key=|subject| subject.id.clone()
                                let(subject)
                            >
                                <SubjectAccordionItem subject=subject />
                            </For>
                        </Accordion>
                    </Show>
                </Suspense>
            </AccordionContent>
        </AccordionItem>
    }
    /*  view! {
        <AccordionItem name="class" open=false>
            <AccordionTrigger class="font-bold pb-2">
                <div class="flex items-center justify-between w-full">
                    <div class="flex h-5 items-center space-x-2 text-sm">
                        <span>"Class One"</span>
                        <Separator vertical=true />
                        <span class="font-bengali">"প্রথম শ্রেণী"</span>
                    </div>

                    <div class="flex">
                        <Tooltip value="Add Subject">
                            <Button variant="ghost" size="small">
                                {icon!(LuPlus)}
                            </Button>
                        </Tooltip>
                        <Tooltip value="Edit Class">
                            <Button variant="ghost" size="small">
                                {icon!(AiEditOutlined)}
                            </Button>
                        </Tooltip>
                    </div>

                </div>
            </AccordionTrigger>
            <AccordionContent class="pb-0">
                <Accordion class="pb-0">
                    <AccordionItem name="subject" class="mx-4 pb-0">
                        <AccordionTrigger>
                            <div class="flex items-center justify-between w-full">
                                <div class="flex items-center space-x-2 text-sm">
                                    <span>"Bangla"</span>
                                    <Separator vertical=true />
                                    <span class="font-bengali">"বাংলা"</span>
                                </div>
                                <div class="flex">
                                    <Tooltip value="Add Chapter">
                                        <Button variant="ghost" size="small">
                                            {icon!(LuPlus)}
                                        </Button>
                                    </Tooltip>
                                    <Tooltip value="Edit Subject">
                                        <Button variant="ghost" size="small">
                                            {icon!(AiEditOutlined)}
                                        </Button>
                                    </Tooltip>
                                </div>
                            </div>
                        </AccordionTrigger>
                        <AccordionContent class="flex flex-col mx-2">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-2">
                                    <span>
                                        "শিক্ষা ও মনুষ্যত্ব"
                                    </span>
                                    <span class="text-gray-400 text-xs">"Chapter 1"</span>
                                    <span class="text-gray-400">"(40)"</span>
                                </div>
                                <Tooltip value="Edit Chapter">
                                    <Button variant="ghost" size="small">
                                        {icon!(AiEditOutlined)}
                                    </Button>
                                </Tooltip>
                            </div>

                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-2">
                                    <span>"একাত্তরের দিনগুলি"</span>
                                    <span class="text-gray-400 text-xs">"Chapter 2"</span>
                                    <span class="text-gray-400">"(20)"</span>
                                </div>
                                <Tooltip value="Edit Chapter">
                                    <Button variant="ghost" size="small">
                                        {icon!(AiEditOutlined)}
                                    </Button>
                                </Tooltip>
                            </div>
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem name="subject" class="mx-4 pb-0">
                        <AccordionTrigger>
                            <div class="flex items-center justify-between w-full">
                                <div class="flex items-center space-x-2 text-sm">
                                    <span>"English"</span>
                                    <Separator vertical=true />
                                    <span class="font-bengali">"ইংরেজি"</span>
                                </div>
                                <div class="flex">
                                    <Tooltip value="Add Chapter">
                                        <Button variant="ghost" size="small">
                                            {icon!(LuPlus)}
                                        </Button>
                                    </Tooltip>
                                    <Tooltip value="Edit Subject">
                                        <Button variant="ghost" size="small">
                                            {icon!(AiEditOutlined)}
                                        </Button>
                                    </Tooltip>
                                </div>
                            </div>
                        </AccordionTrigger>
                        <AccordionContent>
                            <p class="text-gray-400">"No chapter to show!"</p>
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>

            </AccordionContent>
        </AccordionItem>
        <AccordionItem name="class" open=false>
            <AccordionTrigger class="font-bold pb-2">
                <div class="flex items-center justify-between w-full">
                    <div class="flex h-5 items-center space-x-2 text-sm">
                        <span>"Class Two"</span>
                        <Separator vertical=true />
                        <span class="font-bengali">
                            "দ্বিতীয় শ্রেণী"
                        </span>
                    </div>
                    <div class="flex">
                        <Tooltip value="Add Subject">
                            <Button variant="ghost" size="small">
                                {icon!(LuPlus)}
                            </Button>
                        </Tooltip>
                        <Tooltip value="Edit Class">
                            <Button variant="ghost" size="small">
                                {icon!(AiEditOutlined)}
                            </Button>
                        </Tooltip>
                    </div>
                </div>
            </AccordionTrigger>
            <AccordionContent>
                <p class="text-gray-400">"No subject to show!"</p>
            </AccordionContent>
        </AccordionItem>
    }*/
}

#[component]
fn SubjectAccordionItem(subject: Subject) -> impl IntoView {
    let is_bangle = RwSignal::new(!subject.title.is_ascii());
    view! {
        <AccordionItem name=format!("subject-{}", subject.id.to_string()) class="mx-4 pb-0">
            <AccordionTrigger>
                <div class="flex items-center justify-between w-full">
                    <div class="flex items-center space-x-2 text-sm">
                        <span class=("font-bengali", move || is_bangle.get())>{subject.title}</span>
                    </div>
                    <div class="flex">
                        <Tooltip value="Add Chapter">
                            <Button variant="ghost" size="small">
                                {icon!(LuPlus)}
                            </Button>
                        </Tooltip>
                        <Tooltip value="Edit Subject">
                            <Button variant="ghost" size="small">
                                {icon!(AiEditOutlined)}
                            </Button>
                        </Tooltip>
                    </div>
                </div>
            </AccordionTrigger>
            <AccordionContent class="flex flex-col mx-2">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-2">
                        <span>"শিক্ষা ও মনুষ্যত্ব"</span>
                        <span class="text-gray-400 text-xs">"Chapter 1"</span>
                        <span class="text-gray-400">"(40)"</span>
                    </div>
                    <Tooltip value="Edit Chapter">
                        <Button variant="ghost" size="small">
                            {icon!(AiEditOutlined)}
                        </Button>
                    </Tooltip>
                </div>

                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-2">
                        <span>"একাত্তরের দিনগুলি"</span>
                        <span class="text-gray-400 text-xs">"Chapter 2"</span>
                        <span class="text-gray-400">"(20)"</span>
                    </div>
                    <Tooltip value="Edit Chapter">
                        <Button variant="ghost" size="small">
                            {icon!(AiEditOutlined)}
                        </Button>
                    </Tooltip>
                </div>
            </AccordionContent>
        </AccordionItem>
    }
}

/// Documentation for [`PageSkeleton`]
#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <Card class="w-full md:w-1/2 lg:w-1/3 mx-auto h-[270px]">
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

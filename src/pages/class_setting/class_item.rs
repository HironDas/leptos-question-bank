use icondata::{AiEditOutlined, LuPlus};
use leptos::{html, logging::log, prelude::*};
use singlestage::*;

use crate::{
    pages::class_setting::subject_item::SubjectAccordionItem,
    server_function::{
        academic_helper::{
            class::UpdateClass,
            subject::{self, AddSubject, UpdateSubject},
        },
        academic_setting::{Class, Subject},
    },
};

#[component]
pub fn ClassAccordionItem(
    class: Memo<Class>,
    update_class_action: ServerAction<UpdateClass>,
) -> impl IntoView {
    // let class = RwSignal::new(item.get());
    let class_accordion_open = RwSignal::new(false);

    let class_dialog_update_open = RwSignal::new(false);

    let subject_dialog_open = RwSignal::new(false);

    let update_class_value = update_class_action.value();

    let add_subject_action = ServerAction::<AddSubject>::new();
    let subject_value = add_subject_action.value();
    let input_ref = NodeRef::<html::Input>::new();

    let update_subject_action = ServerAction::<UpdateSubject>::new();
    let update_subject_value = update_subject_action.value();

    let subjects = RwSignal::new(Vec::<Subject>::new());

    let has_expanded_once = Memo::new(move |prev_state: Option<&bool>| {
        // 1. Check if the previous state was already true
        // 2. Or if the accordion is currently open
        // Note the * to dereference the &bool
        prev_state.copied().unwrap_or(false) || class_accordion_open.get()
    });

    let subjects_resource: Resource<Vec<Subject>> = Resource::new(
        move || has_expanded_once.get(),
        move |expand| async move {
            log!("ClassAccordionItem expand: {}", expand);
            if expand {
                log!("Fetching subjects for class id: {}", class.get().id);
                match subject::subject(class.get().id).await {
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

    Effect::new(move || {
        if let Some(subjects_data) = subjects_resource.get() {
            subjects.set(subjects_data);
        }
    });

    Effect::new(move || {
        if subject_value.get().map(|v| v.is_ok()).unwrap_or(false) {
            subject_dialog_open.set(false);
            subjects.update(|subjects| {
                if let Some(Ok(new_subject)) = subject_value.get() {
                    log!("New subject received in Effect: {:?}", new_subject);
                    subjects.push(new_subject);
                    // Optional: If you want to keep the "order" consistent
                    subjects.sort_by_key(|s| s.order);
                }
            });
        }

        if update_subject_value
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
        {
            subjects.update(|subjects| {
                if let Some(Ok(updated_subject)) = update_subject_value.get() {
                    log!("Updated subject received in Effect: {:?}", updated_subject);
                    let index = subjects.iter().position(|s| s.id == updated_subject.id);
                    if let Some(index) = index {
                        subjects[index] = updated_subject;
                    }
                    // Optional: If you want to keep the "order" consistent
                    subjects.sort_by_key(|s| s.order);
                }
            });
        }

        if update_class_value.get().map(|v| v.is_ok()).unwrap_or(false) {
            class_dialog_update_open.set(false);
        }
    });

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
                        <span>{move || class.get().name}</span>
                        <Separator vertical=true />
                        <span class="font-bengali">{move || class.get().name_bn}</span>
                    </div>

                    <div class="flex">
                       <AddSubjectDialog input_ref class add_subject_action  subject_dialog_open/>
                       <UpdateClassDialog class_dialog_update_open update_class_action class/>

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
                        when=move || !subjects.get().is_empty()
                        fallback=move || {
                            view! { <div class="text-gray-400 pb-2">"No Subject to show!"</div> }
                        }
                    >
                        <Accordion class="pb-0">
                            <For
                                each=move || subjects.get()
                                key=|subject| subject.id
                                children=move |subject| {
                                    let id = subject.id;
                                    let memoized_subject = Memo::new(move |_| {
                                        subjects
                                            .get()
                                            .into_iter()
                                            .find(|s| s.id == id)
                                            .unwrap_or(subject.clone())
                                    });
                                    view! {
                                        <SubjectAccordionItem
                                            subject=memoized_subject
                                            update_subject_action=update_subject_action
                                        />
                                    }
                                }
                            />
                        </Accordion>
                    </Show>
                </Suspense>
            </AccordionContent>
        </AccordionItem>
    }
}

#[component]
pub fn UpdateClassDialog(
    #[prop(into)] class_dialog_update_open: RwSignal<bool>,
    // class_input_ref: NodeRef<html::Input>,
    update_class_action: ServerAction<UpdateClass>,
    class: Memo<Class>,
) -> impl IntoView {
    view! {
        <Dialog open=class_dialog_update_open>
            <DialogTrigger slot>
                <Tooltip value="Edit Class">
                    <Button
                        variant="ghost"
                        size="small"
                        on:click=move |ev| {
                            ev.prevent_default();
                            ev.stop_propagation();
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
                                prop:value=move || class.get().id.to_string()
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
    }
}

#[component]
pub fn AddSubjectDialog(
    subject_dialog_open: RwSignal<bool>,
    input_ref: NodeRef<html::Input>,
    add_subject_action: ServerAction<AddSubject>,
    class: Memo<Class>,
) -> impl IntoView {
    view! {
        <Dialog open=subject_dialog_open>
            <DialogTrigger slot>
                <Tooltip value="Add Subject">
                    <Button
                        variant="ghost"
                        size="small"
                        on:click=move |ev| {
                            ev.prevent_default();
                            // ev.stop_propagation();
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
    }
}

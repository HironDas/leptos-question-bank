use icondata::{AiEditOutlined, LuPlus};
use leptos::{html, logging::log, prelude::*};
use singlestage::*;

use crate::{
    pages::class_setting::chapter_item::ChapterAccordionContent,
    server_function::{
        academic_helper::{
            chapter::{subject_chapter, AddChapter, UpdateChapter},
            subject::UpdateSubject,
        },
        academic_setting::{Chapter, Subject},
    },
};

#[component]
pub fn SubjectAccordionItem(
    subject: Memo<Subject>,
    update_subject_action: ServerAction<UpdateSubject>,
) -> impl IntoView {
    let is_bangle = Memo::new(move |_| !subject.get().title.is_ascii());
    let subject_accordion_open = RwSignal::new(false);
    let chapter_dialog_open = RwSignal::new(false);

    let chapter_input_ref = NodeRef::<html::Input>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let add_chapter_action = ServerAction::<AddChapter>::new();
    let chapter_value = add_chapter_action.value();

    let subject_dialog_open = RwSignal::new(false);
    let update_subject_value = update_subject_action.value();
    let chapters = RwSignal::new(Vec::<Chapter>::new());

    let update_chapter_action = ServerAction::<UpdateChapter>::new();
    let update_chapter_value = update_chapter_action.value();

    Effect::new(move || {
        if update_chapter_value
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
        {
            // chapter_dialog_open.set(false);
            let updated_chapter = update_chapter_value.get().unwrap().unwrap();
            chapters.update(move |chapters_vec| {
                if let Some(pos) = chapters_vec.iter().position(|c| c.id == updated_chapter.id) {
                    chapters_vec[pos] = updated_chapter;
                }
            });
        }
    });

    Effect::new(move || {
        if chapter_value.get().map(|v| v.is_ok()).unwrap_or(false) {
            chapter_dialog_open.set(false);
            chapters.update(move |chapters_vec| {
                chapter_value.get().map(|new_chapter| {
                    if let Ok(new_chapter) = new_chapter {
                        chapters_vec.push(new_chapter);
                    }
                });
            })
        }
    });

    Effect::new(move || {
        if update_subject_value
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
        {
            subject_dialog_open.set(false);
        }
    });

    let has_expanded_once = Memo::new(move |prev_state| {
        prev_state.copied().unwrap_or(false) || subject_accordion_open.get()
    });

    let chapters_resource: Resource<Vec<Chapter>> = Resource::new(
        move || has_expanded_once.get(),
        move |expand| async move {
            log!("SubjectAccordionItem expand: {}", expand);
            if expand {
                log!("Fetching chapters for subject id: {}", subject.get().id);
                match subject_chapter(subject.get().id).await {
                    Ok(data) => data,
                    Err(err) => {
                        log!("Error fetching chapters: {:?}", err);
                        vec![]
                    }
                }
            } else {
                vec![]
            }
        },
    );

    Effect::new(move || {
        if let Some(chapters_vec) = chapters_resource.get() {
            chapters.set(chapters_vec);
        }
    });
    view! {
        <AccordionItem
            open=subject_accordion_open
            attr::name=move || format!("subject-{}", subject.get().id.to_string())
            class="mx-4 pb-0"
        >
            <AccordionTrigger on:click=move |ev| {
                ev.prevent_default();
                subject_accordion_open.set(!subject_accordion_open.get_untracked());
            }>
                <div class="flex items-center justify-between w-full">
                    <div class="flex items-center space-x-2 text-sm">
                        <span class=(
                            "font-bengali",
                            move || is_bangle.get(),
                        )>{move || subject.get().title}</span>
                    </div>
                    <div class="flex">
                        <Dialog open=chapter_dialog_open>
                            <DialogTrigger slot>
                                <Tooltip value="Add Chapter">
                                    <Button
                                        variant="ghost"
                                        size="small"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            chapter_input_ref
                                                .get()
                                                .map(|input| {
                                                    input.set_value("");
                                                });
                                            log!(
                                                "Opening chapter dialog for subject id: {}", subject.get().id
                                            );
                                            chapter_dialog_open.set(true);
                                        }
                                    >
                                        {icon!(LuPlus)}
                                    </Button>
                                </Tooltip>
                            </DialogTrigger>
                            <DialogClose />
                            <DialogHeader>
                                <DialogTitle>"Add Chapter"</DialogTitle>
                                <DialogDescription>
                                    "Add a new chapter to the subject."
                                </DialogDescription>
                            </DialogHeader>
                            <ActionForm action=add_chapter_action>
                                <DialogContent>
                                    <div class="form">
                                        <div class="grid gap-4">

                                            <Input
                                                name="subject_id"
                                                input_type="hidden"
                                                prop:value=move || subject.get().id.to_string()
                                            />

                                            <div class="grid gap-2">
                                                <Label label_for="chapter_title">"Title"</Label>
                                                <InputGroup>
                                                    <Input
                                                        id="chapter_title"
                                                        name="title"
                                                        input_type="text"
                                                        placeholder="Enter chapter title"
                                                        node_ref=chapter_input_ref
                                                    />
                                                    // prop:node_ref=input_refan
                                                    // input_node_ref=input_ref
                                                    <InputGroupAddon align="inline-end">
                                                        <Tooltip value="Chapter title should be less than 120 characters">
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
                                            chapter_dialog_open.set(false);
                                        }
                                        variant="outline"
                                    >
                                        "Cancel"
                                    </Button>
                                    <Button
                                        size="small"
                                        button_type="submit"
                                        attr:disabled=move || add_chapter_action.pending().get()
                                    >

                                        <Show
                                            when=move || add_chapter_action.pending().get()
                                            fallback=|| view! { "Create Chapter" }
                                        >
                                            "Processing..."
                                            <Spinner />
                                        </Show>
                                    </Button>
                                </footer>

                            </ActionForm>
                        </Dialog>

                        <UpdateSubjectDialog input_ref subject_dialog_open subject update_subject_action/>
                    </div>
                </div>
            </AccordionTrigger>
            <AccordionContent class="flex flex-col mx-2">
                <Suspense fallback=|| {
                    view! {
                        <div class="flex justify-center items-center mb-2">
                            <Spinner />
                        </div>
                    }
                }>

                    <Show
                        when=move || !chapters.get().is_empty()
                        fallback=move || {
                            view! { <div class="text-gray-400 pb-2">"Add no chapter yet!"</div> }
                        }
                    >
                        <For
                            each=move || chapters.get()
                            key=|chapter| chapter.id.clone()
                            children=move |chapter| {
                                let memo_chapter = Memo::new(move |_| {
                                    chapters
                                        .get()
                                        .into_iter()
                                        .find(|c| c.id == chapter.id)
                                        .unwrap_or(chapter.clone())
                                });
                                view! {
                                    <ChapterAccordionContent
                                        chapter=memo_chapter
                                        update_chapter_action=update_chapter_action
                                    />
                                }
                            }
                        />

                    </Show>
                </Suspense>
            </AccordionContent>
        </AccordionItem>
    }
}

#[component]
pub fn UpdateSubjectDialog(
    subject_dialog_open: RwSignal<bool>,
    input_ref: NodeRef<html::Input>,
    subject: Memo<Subject>,
    update_subject_action: ServerAction<UpdateSubject>,
) -> impl IntoView {
    view! {
        <Dialog open=subject_dialog_open>
            <DialogTrigger slot>
                <Tooltip value="Update Subject">
                    <Button
                        variant="ghost"
                        size="small"
                        on:click=move |ev| {
                            ev.prevent_default();
                            ev.stop_propagation();
                            input_ref
                                .get()
                                .map(|input| {
                                    input.set_value(&subject.get().title);
                                });
                            subject_dialog_open.set(true);
                        }
                    >
                        {icon!(AiEditOutlined)}
                    </Button>
                </Tooltip>
            </DialogTrigger>
            <DialogClose />
            <DialogHeader>
                <DialogTitle>"Update Subject"</DialogTitle>
                <DialogDescription>"Update the subject title."</DialogDescription>
            </DialogHeader>
            <ActionForm action=update_subject_action>
                <DialogContent>
                    <div class="form">
                        <div class="grid gap-4">

                            <Input
                                name="subject_id"
                                input_type="hidden"
                                prop:value=move || subject.get().id.to_string()
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
                        attr:disabled=move || update_subject_action.pending().get()
                    >

                        <Show
                            when=move || update_subject_action.pending().get()
                            fallback=|| view! { "Update Subject" }
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

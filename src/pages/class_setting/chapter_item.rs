use icondata::AiEditOutlined;
use leptos::{html, logging::log, prelude::*};
use singlestage::*;

use crate::server_function::{academic_helper::chapter::UpdateChapter, academic_setting::Chapter};

#[component]
pub fn ChapterAccordionContent(
    chapter: Memo<Chapter>,
    update_chapter_action: ServerAction<UpdateChapter>,
) -> impl IntoView {
    let is_bangle = Memo::new(move |_| !chapter.get().title.is_ascii());
    let update_chapter_value = update_chapter_action.value();
    let chapter_dialog_open = RwSignal::new(false);
    let chapter_input_ref = NodeRef::<html::Input>::new();

    Effect::new(move || {
        if update_chapter_value
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
        {
            chapter_dialog_open.set(false);
        }
    });

    view! {
        <AccordionContent class="flex flex-col mx-2">
            <div class="flex items-center justify-between w-full">
                <div class="flex items-center space-x-2 text-sm">
                    <span class=(
                        "font-bengali",
                        move || is_bangle.get(),
                    )>{move || chapter.get().title}</span>
                    <span class="text-gray-400 text-xs">
                        "Chapter "{move || chapter.get().order}
                    </span>
                    <span class="text-gray-400">"("{move || chapter.get().question_count}")"</span>
                </div>
                <div class="flex">
                    <UpdateChapterDialog chapter_dialog_open chapter_input_ref update_chapter_action chapter/>
                </div>
            </div>
        </AccordionContent>
    }
}

#[component]
pub fn UpdateChapterDialog(
    #[prop(into)] chapter_dialog_open: RwSignal<bool>,
    chapter_input_ref: NodeRef<html::Input>,
    update_chapter_action: ServerAction<UpdateChapter>,
    chapter: Memo<Chapter>,
) -> impl IntoView {
    view! {
        <Dialog open=chapter_dialog_open>
            <DialogTrigger slot>
                <Tooltip value="Update Chapter">
                    <Button
                        variant="ghost"
                        size="small"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            chapter_input_ref
                                .get()
                                .map(|input| {
                                    input.set_value(&chapter.get().title);
                                });
                            log!(
                                "Opening chapter dialog for chapter id: {}", chapter.get().id
                            );
                            chapter_dialog_open.set(true);
                        }
                    >
                        {icon!(AiEditOutlined)}
                    </Button>
                </Tooltip>
            </DialogTrigger>
            <DialogClose />
            <DialogHeader>
                <DialogTitle>"Update Chapter"</DialogTitle>
                <DialogDescription>"Update the chapter title."</DialogDescription>
            </DialogHeader>
            <ActionForm action=update_chapter_action>
                <DialogContent>
                    <div class="form">
                        <div class="grid gap-4">

                            <Input
                                name="chapter_id"
                                input_type="hidden"
                                prop:value=move || chapter.get().id.to_string()
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
                            ev.stop_propagation();
                            chapter_dialog_open.set(false);
                        }
                        variant="outline"
                    >
                        "Cancel"
                    </Button>
                    <Button
                        size="small"
                        button_type="submit"
                        attr:disabled=move || update_chapter_action.pending().get()
                    >

                        <Show
                            when=move || update_chapter_action.pending().get()
                            fallback=|| view! { "Update Chapter" }
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

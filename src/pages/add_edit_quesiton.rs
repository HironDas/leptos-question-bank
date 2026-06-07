use icondata::{
    LuBold, LuHeading1, LuHeading2, LuHeading3, LuItalic, LuList, LuListOrdered, LuMinus, LuSigma,
    LuTable, LuUnderline, LuX,
};
use leptos::{html, prelude::*};
use leptos_router::hooks::use_query_map;
use singlestage::*;
use web_sys::MouseEvent;

use crate::{
    domain::question::{AddQuestionInput, AddQuestionOptionInput},
    server_function::question::AddQuestion,
    util::insert_markdown_at_cursor::Syntex,
};

use crate::util::insert_markdown_at_cursor::insert_markdown;

#[component]
pub fn AddEditQuestion() -> impl IntoView {
    #[derive(Clone)]
    enum Difficulty {
        Easy = 0,
        Medium,
        Hard,
    }

    let query_map = use_query_map();

    let chapter_id = move || query_map.read().get("chapter"); // chapter_id_u32.get_untracked();
    let chapter_id = chapter_id()
        .and_then(|val| val.parse::<u32>().ok())
        .unwrap_or(0);

    let class_id = move || query_map.read().get("class"); //class_id_u32.get_untracked();
    let class_id = class_id()
        .and_then(|val| val.parse::<u32>().ok())
        .unwrap_or(0);

    let subject_id = move || query_map.read().get("subject"); //subject_id_u32.get_untracked();
    let subject_id = subject_id()
        .and_then(|val| val.parse::<u32>().ok())
        .unwrap_or(0);

    // Effect::new(move || if class_id.ok_or("error") {});

    // --- Form refs ---
    let question_text_ref = NodeRef::<html::Textarea>::new();
    let answer_text_ref = NodeRef::<html::Textarea>::new();
    let option_refs = StoredValue::new(
        (0..8)
            .map(|_| NodeRef::<html::Input>::new())
            .collect::<Vec<_>>(),
    );
    let option_check_refs = StoredValue::new(
        (0..8)
            .map(|_| NodeRef::<html::Input>::new())
            .collect::<Vec<_>>(),
    );
    let option_count = RwSignal::new(4u32);

    // --- Server actions ---
    let add_question_action = ServerAction::<AddQuestion>::new();
    let add_question_value = add_question_action.value();

    // --- Dialog state ---
    // let question_dialog_open = RwSignal::new(false);
    let question_type = RwSignal::new("objective".to_string());
    let question_difficulty = RwSignal::new(Difficulty::Medium);

    // --- Derived: button variants for question type toggle ---
    let obj_btn_variant = Memo::new(move |_| {
        if question_type.get() == "objective" {
            "default".to_string()
        } else {
            "outline".to_string()
        }
    });
    let subj_btn_variant = Memo::new(move |_| {
        if question_type.get() == "subjective" {
            "default".to_string()
        } else {
            "outline".to_string()
        }
    });

    let easy_difficulty_variant = Memo::new(move |_| match question_difficulty.get() {
        Difficulty::Easy => "default".to_string(),
        _ => "outline".to_string(),
    });
    let medium_difficulty_variant = Memo::new(move |_| match question_difficulty.get() {
        Difficulty::Medium => "default".to_string(),
        _ => "outline".to_string(),
    });
    let hard_difficulty_variant = Memo::new(move |_| match question_difficulty.get() {
        Difficulty::Hard => "default".to_string(),
        _ => "outline".to_string(),
    });

    // --- Submit handler ---
    let submit_question = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let text = question_text_ref
            .get()
            .map(|el| el.value())
            .unwrap_or_default();
        let qtype = question_type.get_untracked();

        let difficulty = question_difficulty.get_untracked();
        let answer = answer_text_ref
            .get()
            .map(|el| el.value())
            .filter(|s| !s.is_empty());

        let mut options = Vec::new();
        if qtype == "objective" {
            let count = option_count.get_untracked() as usize;
            for i in 0..count {
                let opt_text = option_refs.get_value()[i]
                    .get()
                    .map(|el| el.value())
                    .unwrap_or_default();
                let is_correct = option_check_refs.get_value()[i]
                    .get()
                    .map(|el| el.checked())
                    .unwrap_or(false);
                if !opt_text.trim().is_empty() {
                    options.push(AddQuestionOptionInput {
                        option_text: opt_text,
                        is_correct,
                        order: i as u32,
                    });
                }
            }
        }

        let input = AddQuestionInput {
            question_text: text,
            question_type: qtype,
            chapter_id: chapter_id,
            class_id: class_id,
            subject_id: subject_id,
            answer_text: answer,
            difficulty: difficulty as u32,
            options,
        };

        add_question_action.dispatch(AddQuestion { input });
    };

    let reset_question = move |ev: MouseEvent| {
        ev.prevent_default();
        question_text_ref.get().map(|el| el.set_value(""));
        answer_text_ref.get().map(|el| el.set_value(""));
        for i in 0..8usize {
            option_refs.get_value()[i].get().map(|el| el.set_value(""));
            option_check_refs.get_value()[i]
                .get()
                .map(|el| el.set_checked(false));
        }
        option_count.set(4);
        question_type.set("objective".to_string());
        question_difficulty.set(Difficulty::Medium);
        // question_dialog_open.set(true);
    };

    let preview = RwSignal::new(false);

    view! {
        <div class="w-full">
            <form on:submit=submit_question>
                <FieldGroup>
                    <FieldSet>
                        <FieldLegend>"Add New Question"</FieldLegend>
                        <FieldDescription>
                             "Create a new question for the selected chapter."
                        </FieldDescription>
                        <FieldGroup>
                            <Field orientation="horizontal">
                                <div class="flex gap-2 items-center">
                                    // <Label>"Question Type:"</Label>
                                    <Tooltip value="Question Type">
                                    <ButtonGroup>
                                        <Button
                                            size="sm"
                                            variant=obj_btn_variant
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                question_type.set("objective".to_string());
                                            }
                                        >
                                            "Objective"
                                        </Button>
                                        <Button
                                            size="sm"
                                            variant=subj_btn_variant
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                question_type.set("subjective".to_string());
                                            }
                                        >
                                            "Subjective"
                                        </Button>
                                    </ButtonGroup>
                                    </Tooltip>
                                </div>
                                <div class="flex gap-2 items-center">
                                    // <Label>"Difficulty:"</Label>
                                    <Tooltip value="Question Difficulty">
                                    <ButtonGroup>
                                        <Button
                                            size="sm"
                                            variant=easy_difficulty_variant
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                question_difficulty.set(Difficulty::Easy);
                                            }
                                        >
                                            "Easy"
                                        </Button>
                                        <Button
                                            size="sm"
                                            variant=medium_difficulty_variant
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                question_difficulty.set(Difficulty::Medium);
                                            }
                                        >
                                            "Medium"
                                        </Button>
                                        <Button
                                            size="sm"
                                            variant=hard_difficulty_variant
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                question_difficulty.set(Difficulty::Hard);
                                            }
                                        >
                                            "Hard"
                                        </Button>
                                    </ButtonGroup>
                                    </Tooltip>
                                </div>
                            </Field>
                            <Field>
                                // Question text
                                <div class="grid gap-2">
                                    <Label label_for="question_text" class="mb-2">"Question"</Label>
                                    <div class="flex h-4 items-center space-x-2 text-sm">
                                    <Button on:click=move|ev| {ev.prevent_default(); preview.update(move|val| *val = !*val);} variant="ghost">{move || if preview.get()"Preview" else "Continue Editing"}</Button>
                                        <Separator vertical=true />
                                        <Tooltip value="Heading 1">
                                        <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Heading1)} variant="ghost" aria_label="Heading 1">{icon!(LuHeading1)}</Button>
                                        </Tooltip>
                                        <Tooltip value="Heading 2">
                                        <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Heading2)} variant="ghost" aria_label="Heading 2">{icon!(LuHeading2)}</Button>
                                        </Tooltip>
                                        <Tooltip value="Heading 3">
                                        <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Heading3)} variant="ghost" aria_label="Heading 3">{icon!(LuHeading3)}</Button>
                                        </Tooltip>
                                        <Separator vertical=true />
                                        <Tooltip value="Add bold text">
                                        <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Bold)} variant="ghost" aria_label="Toggle Bold">{icon!(LuBold)}</Button>
                                        </Tooltip>
                                        <Tooltip value="Add italic text">
                                            <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Italic)} variant="ghost" aria_label="Toggle italic">{icon!(LuItalic)} </Button>
                                        </Tooltip>
                                        <Tooltip value="Add text underline">
                                            <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Underscore)} variant="ghost" aria_label="Underline">{icon!(LuUnderline)} </Button>
                                        </Tooltip>
                                        <Tooltip value="Add Dash">
                                            <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Dash)} variant="ghost" aria_label="Underline">{icon!(LuMinus)} </Button>
                                        </Tooltip>
                                        <Separator vertical=true />
                                        <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Bullet)} variant="ghost" aria_label="Bullet-point">{icon!(LuList)} </Button>
                                        <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(question_text_ref, Syntex::Order)} variant="ghost" aria_label="Ordered List">{icon!(LuListOrdered)} </Button>
                                        <Separator vertical=true />

                                        <Popover>
                                            <PopoverTrigger>
                                                <Button button_type="button" variant="ghost" aria_label="Add a Table">{icon!(LuTable)} </Button>
                                            </PopoverTrigger>
                                            <PopoverContent class="w-80">

                                                    <FieldSet>
                                                        <FieldGroup class="[&_input]:w-20 gap-2">
                                                            <Field orientation="horizontal">
                                                                <Tooltip side="right" align="center" value="Row">
                                                                    <Input input_type="number" value="2" autofocus=true/>
                                                                </Tooltip>
                                                                <div>{icon!(LuX)}</div>
                                                                <Tooltip side="left" align="center" value="Column">
                                                                    <Input input_type="number" value="3"/>
                                                                </Tooltip>
                                                                <Button on:click=move|e|e.prevent_default()>"Add"</Button>
                                                            </Field>

                                                        </FieldGroup>
                                                    </FieldSet>

                                            </PopoverContent>
                                        </Popover>
                                        <Tooltip value="Add formula">
                                            <Button on:click=move|ev| ev.prevent_default() variant="ghost" aria_label="Underline">{icon!(LuSigma)} </Button>
                                        </Tooltip>

                                    </div>
                                    <Separator class="my-2" />
                                    <textarea
                                        node_ref=question_text_ref
                                        class="flex min-h-[120px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                                        placeholder="Enter your question here..."
                                        rows="3"
                                    ></textarea>
                                </div>
                            </Field>
                            <Field>
                            // Objective: Options section
                            <Show when=move || question_type.get() == "objective">
                                <div class="grid gap-2">
                                    <Label>"Options"</Label>
                                    <p class="text-xs text-muted-foreground">
                                        "Mark the correct option(s) with the checkbox."
                                    </p>
                                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-x-4 gap-y-2">
                                        <For
                                            each=move || 0..option_count.get()
                                            key=|i| *i
                                            children=move |i| {
                                                let idx = i as usize;
                                                view! {
                                                    <div class="flex items-center gap-2">
                                                        <span class="text-sm w-6 font-mono">
                                                            {move || format!("{})", i + 1)}
                                                        </span>
                                                        <input
                                                            type="text"
                                                            node_ref=option_refs.get_value()[idx].clone()
                                                            class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
                                                            placeholder=move || format!("Option {}", i + 1)
                                                        />
                                                        <input
                                                            type="checkbox"
                                                            node_ref=option_check_refs.get_value()[idx].clone()
                                                            class="h-4 w-4"
                                                        />
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                    <div class="flex gap-2">
                                        <Button
                                            size="small"
                                            variant="outline"
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                if option_count.get() > 2 {
                                                    let idx = (option_count.get() - 1) as usize;
                                                    option_refs.get_value()[idx].get().map(|el| el.set_value(""));
                                                    option_check_refs.get_value()[idx]
                                                        .get()
                                                        .map(|el| el.set_checked(false));
                                                    option_count.update(|c| *c -= 1);
                                                }
                                            }
                                        >
                                            "- Remove Option"
                                        </Button>
                                        <Button
                                            size="small"
                                            variant="outline"
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                if option_count.get() < 8 {
                                                    option_count.update(|c| *c += 1);
                                                }
                                            }
                                        >
                                            "+ Add Option"
                                        </Button>
                                    </div>
                                </div>
                            </Show>
                            </Field>
                            <Field>
                                // Subjective: Answer section
                                <Show when=move || question_type.get() == "subjective">
                                    <div class="grid gap-2">
                                        <Label label_for="answer_text">"Answer"</Label>
                                        <textarea
                                            node_ref=answer_text_ref
                                            class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                                            placeholder="Enter the answer..."
                                            rows="3"
                                        ></textarea>
                                    </div>
                                </Show>
                            </Field>
                            <Field orientation="horizontal">
                                <Button
                                    size="small"
                                    on:click=reset_question
                                    variant="outline"
                                >
                                    "Reset"
                                </Button>
                                <Button
                                    size="small"
                                    button_type="submit"
                                    attr:disabled=move || add_question_action.pending().get()
                                >
                                    <Show
                                        when=move || add_question_action.pending().get()
                                        fallback=|| view! { "Save Question" }
                                    >
                                        "Saving..."
                                        <Spinner />
                                    </Show>
                                </Button>
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                </FieldGroup>
            </form>
        </div>
    }
}

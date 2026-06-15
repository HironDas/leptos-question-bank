use crate::components::ui::rich_text_editor::RichTextEditor;

use leptos::{html, prelude::*};
use leptos_router::hooks::{use_navigate, use_query};
use leptos_router::params::Params;
use singlestage::*;
use web_sys::MouseEvent;

use crate::{
    domain::question::{AddQuestionInput, AddQuestionOptionInput},
    server_function::question::AddQuestion,
};

// 1. Define a struct that maps your browser query parameters
#[derive(Params, PartialEq, Clone, Debug)]
struct QuestionQueryParams {
    id: Option<u32>,
    class: Option<u32>,
    subject: Option<u32>,
    chapter: Option<u32>,
}

#[derive(Clone, PartialEq, Debug)]
enum Mode {
    Edit(u32),
    Create {
        class_id: u32,
        subject_id: u32,
        chapter_id: u32,
    },
}

#[component]
pub fn AddEditQuestion() -> impl IntoView {
    #[derive(Clone)]
    enum Difficulty {
        Easy = 0,
        Medium,
        Hard,
    }

    let query_params = use_query::<QuestionQueryParams>();
    let navigate = use_navigate();

    let page_mode = Memo::new(move |_| {
        if let Ok(params) = query_params.get() {
            if let Some(id) = params.id {
                return Some(Mode::Edit(id));
            } else if let (Some(class_id), Some(subject_id), Some(chapter_id)) =
                (params.class, params.subject, params.chapter)
            {
                return Some(Mode::Create {
                    chapter_id,
                    subject_id,
                    class_id,
                });
            }
        }
        None
    });

    Effect::new(move || {
        if page_mode.get().is_none() {
            navigate("/view", Default::default());
        }
    });

    // --- Form refs ---
    let question_text_ref = NodeRef::<html::Textarea>::new();
    let answer_text_ref = NodeRef::<html::Textarea>::new();
    // let table_pop_ref = NodeRef::<html::Button>::new();

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

        if let Some(Mode::Create {
            class_id,
            subject_id,
            chapter_id,
        }) = page_mode.get()
        {
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
        }
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
                                <RichTextEditor title=String::from("Question") textarea_ref = question_text_ref />
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
                                        <RichTextEditor title=String::from("Answer") textarea_ref = answer_text_ref />
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

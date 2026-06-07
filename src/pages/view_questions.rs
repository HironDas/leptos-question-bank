use crate::components::ui::empty_msg::EmptyMsg;
use crate::domain::question::Question;
use crate::server_function::academic_helper::chapter::subject_chapter;
use crate::server_function::academic_helper::subject::subject;
use crate::server_function::academic_setting::{academic_setting, Chapter, Class, Subject};
use crate::server_function::question::get_questions;
use icondata::{FiCheckCircle, LuPlus, MdiCloseCircle};
use leptos::{logging::log, prelude::*};
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;
use singlestage::*;

#[component]
pub fn ViewQuestions() -> impl IntoView {
    // --- Data signals ---
    let classes = RwSignal::new(Vec::<Class>::new());
    let subjects = RwSignal::new(Vec::<Subject>::new());
    let chapters = RwSignal::new(Vec::<Chapter>::new());
    let questions = RwSignal::new(Vec::<Question>::new());

    // let is_bangle = Memo::new(move |_| !subject.get().title.is_ascii());

    // --- Selection signals (stored as Strings for the <Select>) ---
    let selected_class_id = RwSignal::new(String::new());
    let selected_subject_id = RwSignal::new(String::new());
    let selected_chapter_id = RwSignal::new(String::new());

    // --- Load classes on mount ---
    let classes_resource = Resource::new(
        || (),
        |_| async move {
            match academic_setting().await {
                Ok(data) => data,
                Err(err) => {
                    log!("Error fetching classes: {:?}", err);
                    vec![]
                }
            }
        },
    );

    // --- Fetch subjects when class changes ---
    let subjects_resource = Resource::new(
        move || selected_class_id.get(),
        move |class_id_str| async move {
            if class_id_str.is_empty() {
                return vec![];
            }
            let class_id: u32 = class_id_str.parse().unwrap_or(0);
            if class_id == 0 {
                return vec![];
            }
            match subject(class_id).await {
                Ok(data) => data,
                Err(err) => {
                    log!("Error fetching subjects: {:?}", err);
                    vec![]
                }
            }
        },
    );

    // --- Fetch chapters when subject changes ---
    let chapters_resource = Resource::new(
        move || selected_subject_id.get(),
        move |subject_id_str| async move {
            if subject_id_str.is_empty() {
                return vec![];
            }
            let subject_id: u32 = subject_id_str.parse().unwrap_or(0);
            if subject_id == 0 {
                return vec![];
            }
            match subject_chapter(subject_id).await {
                Ok(data) => data,
                Err(err) => {
                    log!("Error fetching chapters: {:?}", err);
                    vec![]
                }
            }
        },
    );

    // --- Fetch questions when chapter changes ---
    let questions_resource = Resource::new(
        move || selected_chapter_id.get(),
        move |chapter_id_str| async move {
            if chapter_id_str.is_empty() {
                return vec![];
            }
            let chapter_id: u32 = chapter_id_str.parse().unwrap_or(0);
            if chapter_id == 0 {
                return vec![];
            }
            match get_questions(chapter_id).await {
                Ok(data) => data,
                Err(err) => {
                    log!("Error fetching questions: {:?}", err);
                    vec![]
                }
            }
        },
    );

    // --- Sync resources to signals ---
    Effect::new(move || {
        if let Some(data) = classes_resource.get() {
            classes.set(data);
        }
    });

    Effect::new(move || {
        if let Some(data) = subjects_resource.get() {
            subjects.set(data);
        }
    });

    Effect::new(move || {
        if let Some(data) = chapters_resource.get() {
            chapters.set(data);
        }
    });

    Effect::new(move || {
        if let Some(data) = questions_resource.get() {
            questions.set(data);
        }
    });

    // --- Cascading reset: when class changes, clear subject/chapter/questions ---
    Effect::new(move || {
        let _ = selected_class_id.get();
        selected_subject_id.set(String::new());
        selected_chapter_id.set(String::new());
        subjects.set(Vec::new());
        chapters.set(Vec::new());
        questions.set(Vec::new());
    });

    // --- Cascading reset: when subject changes, clear chapter/questions ---
    Effect::new(move || {
        let _ = selected_subject_id.get();
        selected_chapter_id.set(String::new());
        chapters.set(Vec::new());
        questions.set(Vec::new());
    });

    // --- Cascading reset: when chapter changes, clear questions ---
    Effect::new(move || {
        let _ = selected_chapter_id.get();
        questions.set(Vec::new());
    });

    // --- Handle new question added ---
    // Effect::new(move || {
    //     if add_question_value.get().map(|v| v.is_ok()).unwrap_or(false) {
    //         question_dialog_open.set(false);
    //         if let Some(Ok(new_question)) = add_question_value.get() {
    //             questions.update(|q| q.push(new_question));
    //         }
    //     }
    // });

    // --- Derived IDs for the form ---
    let chapter_id_u32 = Memo::new(move |_| selected_chapter_id.get().parse::<u32>().unwrap_or(0));
    let class_id_u32 = Memo::new(move |_| selected_class_id.get().parse::<u32>().unwrap_or(0));
    let subject_id_u32 = Memo::new(move |_| selected_subject_id.get().parse::<u32>().unwrap_or(0));

    // --- Derived: selected chapter name for display ---
    let selected_chapter_name = Memo::new(move |_| {
        let id_str = selected_chapter_id.get();
        if id_str.is_empty() {
            return String::new();
        }
        let id: u32 = id_str.parse().unwrap_or(0);
        chapters
            .get()
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.title.clone())
            .unwrap_or_default()
    });

    let enumarated_question = move || {
        questions
            .get()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, Question)>>()
    };

    view! {
        <div class="p-2 sm:p-0">

            // --- Filter selects ---
            <div class="flex flex-wrap gap-4 mb-6 items-end">
                // Class select
                <div class="flex flex-col gap-1">
                    <Label label_for="class-select">"Class"</Label>
                    <Select value=selected_class_id class="h-9 min-w-[180px]">
                        <option value="">"-- Select Class --"</option>
                        <For
                            each=move || classes.get()
                            key=|c| c.id
                            children=move |class| {
                                view! {
                                    <option value=class.id.to_string()>
                                        {format!("{} - {}", class.name, class.name_bn)}
                                    </option>
                                }
                            }
                        />
                    </Select>
                </div>

                // Subject select
                <div class="flex flex-col gap-1">
                    <Label label_for="subject-select">"Subject"</Label>
                    <Select value=selected_subject_id class="h-9 min-w-[180px]">
                        <option value="">"-- Select Subject --"</option>
                        <For
                            each=move || subjects.get()
                            key=|s| s.id
                            children=move |subj| {
                                view! {
                                    <option value=subj.id.to_string()>
                                        {subj.title.clone()}
                                    </option>
                                }
                            }
                        />
                    </Select>
                </div>

                // Chapter select
                <div class="flex flex-col gap-1">
                    <Label label_for="chapter-select">"Chapter"</Label>
                    <Select value=selected_chapter_id class="h-9 min-w-[180px]">
                        <option value="">"-- Select Chapter --"</option>
                        <For
                            each=move || chapters.get()
                            key=|c| c.id
                            children=move |chapter| {
                                view! {
                                    <option value=chapter.id.to_string()>
                                        {format!("{} ({} questions)", chapter.title, chapter.question_count)}
                                    </option>
                                }
                            }
                        />
                    </Select>
                </div>
            </div>

            // --- Question grid area ---
            <Show
                when=move || !selected_chapter_id.get().is_empty()
                fallback=move || {
                    view! {
                        <div class="text-center text-muted-foreground py-12">
                            <p>"Select a Class, Subject, and Chapter to view questions."</p>
                        </div>
                    }
                }
            >
                <div class="mb-4 flex items-center justify-between">
                    <h2 class="text-lg font-semibold">
                        <Show
                            when=move || !selected_chapter_name.get().is_empty()
                            fallback=move || view! { <p>"Questions"</p> }
                        >
                            <p>
                                "Questions in: "
                                <span class:font-bengali=move || !selected_chapter_name.get().is_ascii()>
                                    {move || selected_chapter_name.get()}
                                </span>
                            </p>
                        </Show>
                    </h2>
                    <a  href=move ||{format!("/question?class={}", selected_class_id.get())} class = "singlestage-btn-primary singlestage-btn-sm">
                        {icon!(LuPlus)}
                        "Add Question"
                    </a>
                </div>

                // Question grid
                <Show
                    when=move || !questions.get().is_empty()
                    fallback=move || {
                        view! {
                            // <div class="text-center text-muted-foreground py-8 border rounded-md">
                            //     <p>"No questions yet. Click 'Add Question' to create one."</p>
                            // </div>
                            <EmptyMsg
                                button_on_click= move |_| { let nevigate = use_navigate(); nevigate("/question", NavigateOptions::default()); }
                                button_text="Add Question".to_string()
                                icon=icondata::LuNotebookText
                                title=String::from("No Question Yet")
                                description=String::from(
                                    "No questions yet. Click 'Add Question' to create one.",
                                )
                            />
                        }
                    }
                >
                    <div class="grid grid-cols-1 gap-4">
                        <For
                            each=enumarated_question
                            key=|(_,q)| q.id
                            children=move |(index,question)| {
                                view! { <QuestionCard question=question index = index /> }
                            }
                        />
                    </div>
                </Show>
            </Show>

            // // --- Add Question Dialog ---
            // <Dialog class="w-full" open=question_dialog_open>
            //     <DialogTrigger slot>
            //         <div></div>
            //     </DialogTrigger>
            //     <DialogClose />
            //     <DialogHeader>
            //         <DialogTitle>"Add New Question"</DialogTitle>
            //         <DialogDescription>
            //             "Create a new question for the selected chapter."
            //         </DialogDescription>
            //     </DialogHeader>
            //     <form on:submit=submit_question>
            //         <DialogContent class="max-h-[70vh] overflow-y-auto">
            //             <div class="form grid gap-4">
            //                 // Question type toggle
            //                 <div class="flex gap-2 items-center">
            //                     <Label>"Question Type:"</Label>
            //                     <ButtonGroup>
            //                         <Button
            //                             size="sm"
            //                             variant=obj_btn_variant
            //                             on:click=move |ev| {
            //                                 ev.prevent_default();
            //                                 question_type.set("objective".to_string());
            //                             }
            //                         >
            //                             "Objective"
            //                         </Button>
            //                         <Button
            //                             size="sm"
            //                             variant=subj_btn_variant
            //                             on:click=move |ev| {
            //                                 ev.prevent_default();
            //                                 question_type.set("subjective".to_string());
            //                             }
            //                         >
            //                             "Subjective"
            //                         </Button>
            //                     </ButtonGroup>
            //                 </div>

            //                 <div class="flex gap-2 items-center">
            //                     <Label>"Difficulty:"</Label>
            //                     <ButtonGroup>
            //                         <Button
            //                             size="sm"
            //                             variant=easy_difficulty_variant
            //                             on:click=move |ev| {
            //                                 ev.prevent_default();
            //                                 question_difficulty.set(Difficulty::Easy);
            //                             }
            //                         >
            //                             "Easy"
            //                         </Button>
            //                         <Button
            //                             size="sm"
            //                             variant=medium_difficulty_variant
            //                             on:click=move |ev| {
            //                                 ev.prevent_default();
            //                                 question_difficulty.set(Difficulty::Medium);
            //                             }
            //                         >
            //                             "Medium"
            //                         </Button>
            //                         <Button
            //                             size="sm"
            //                             variant=hard_difficulty_variant
            //                             on:click=move |ev| {
            //                                 ev.prevent_default();
            //                                 question_difficulty.set(Difficulty::Hard);
            //                             }
            //                         >
            //                             "Hard"
            //                         </Button>
            //                     </ButtonGroup>
            //                 </div>

            //                 // Question text
            //                 <div class="grid gap-2">
            //                     <Label label_for="question_text">"Question"</Label>
            //                     <div class="flex h-5 items-center space-x-4 text-sm">"Toolbar will be here"</div>
            //                     <Separator class="my-2" />
            //                     <textarea
            //                         node_ref=question_text_ref
            //                         class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            //                         placeholder="Enter your question here..."
            //                         rows="3"
            //                     ></textarea>
            //                 </div>

            //                 // Objective: Options section
            //                 <Show when=move || question_type.get() == "objective">
            //                     <div class="grid gap-2">
            //                         <Label>"Options"</Label>
            //                         <p class="text-xs text-muted-foreground">
            //                             "Mark the correct option(s) with the checkbox."
            //                         </p>
            //                         <For
            //                             each=move || 0..option_count.get()
            //                             key=|i| *i
            //                             children=move |i| {
            //                                 let idx = i as usize;
            //                                 view! {
            //                                     <div class="flex items-center gap-2">
            //                                         <span class="text-sm w-6 font-mono">
            //                                             {move || format!("{})", i + 1)}
            //                                         </span>
            //                                         <input
            //                                             type="text"
            //                                             node_ref=option_refs.get_value()[idx].clone()
            //                                             class="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            //                                             placeholder=move || format!("Option {}", i + 1)
            //                                         />
            //                                         <input
            //                                             type="checkbox"
            //                                             node_ref=option_check_refs.get_value()[idx].clone()
            //                                             class="h-4 w-4"
            //                                         />
            //                                     </div>
            //                                 }
            //                             }
            //                         />
            //                         <div class="flex gap-2">
            //                             <Button
            //                                 size="small"
            //                                 variant="outline"
            //                                 on:click=move |ev| {
            //                                     ev.prevent_default();
            //                                     if option_count.get() > 2 {
            //                                         let idx = (option_count.get() - 1) as usize;
            //                                         option_refs.get_value()[idx].get().map(|el| el.set_value(""));
            //                                         option_check_refs.get_value()[idx]
            //                                             .get()
            //                                             .map(|el| el.set_checked(false));
            //                                         option_count.update(|c| *c -= 1);
            //                                     }
            //                                 }
            //                             >
            //                                 "- Remove Option"
            //                             </Button>
            //                             <Button
            //                                 size="small"
            //                                 variant="outline"
            //                                 on:click=move |ev| {
            //                                     ev.prevent_default();
            //                                     if option_count.get() < 8 {
            //                                         option_count.update(|c| *c += 1);
            //                                     }
            //                                 }
            //                             >
            //                                 "+ Add Option"
            //                             </Button>
            //                         </div>
            //                     </div>
            //                 </Show>

            //                 // Subjective: Answer section
            //                 <Show when=move || question_type.get() == "subjective">
            //                     <div class="grid gap-2">
            //                         <Label label_for="answer_text">"Answer"</Label>
            //                         <textarea
            //                             node_ref=answer_text_ref
            //                             class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            //                             placeholder="Enter the answer..."
            //                             rows="3"
            //                         ></textarea>
            //                     </div>
            //                 </Show>
            //             </div>
            //         </DialogContent>

            //         <footer class="justify-end flex-row gap-2 flex form mt-4">
            //             <Button
            //                 size="small"
            //                 on:click=move |ev| {
            //                     ev.prevent_default();
            //                     question_dialog_open.set(false);
            //                 }
            //                 variant="outline"
            //             >
            //                 "Cancel"
            //             </Button>
            //             <Button
            //                 size="small"
            //                 button_type="submit"
            //                 attr:disabled=move || add_question_action.pending().get()
            //             >
            //                 <Show
            //                     when=move || add_question_action.pending().get()
            //                     fallback=|| view! { "Save Question" }
            //                 >
            //                     "Saving..."
            //                     <Spinner />
            //                 </Show>
            //             </Button>
            //         </footer>
            //     </form>
            // </Dialog>
        </div>
    }
}

#[component]
fn QuestionCard(question: Question, index: usize) -> impl IntoView {
    let expanded = RwSignal::new(false);
    let question = StoredValue::new(question);
    let variant = match question.get_value().question_type {
        crate::domain::question::QuestionType::Objective => {
            "bg-blue-50 text-blue-700" // dark:bg-blue-950 dark:text-blue-300"
        }
        crate::domain::question::QuestionType::Subjective => {
            "bg-green-50 text-green-700" // dark:bg-green-950 dark:text-green-300"
        }
    };
    // let order = question.get_value().order;
    let question_text = question.get_value().question_text.clone();
    let qtype_str = question.get_value().question_type.to_string();

    view! {
        <Card class="cursor-pointer hover:shadow-md transition-shadow">
            <div class="p-4" on:click=move |_| expanded.set(!expanded.get())>
                <div class="flex items-start justify-between mb-2">
                    <span class="text-xs font-mono text-muted-foreground">
                        "#" {index + 1}
                    </span>
                    <Badge class=variant>
                        {qtype_str}
                    </Badge>
                </div>

                <p class="text-sm font-medium line-clamp-3" class:font-bengali=move||!question_text.is_ascii()>
                    {question_text.clone()}
                </p>

                <Show when=move || expanded.get()>
                    <div class="mt-3 pt-3 border-t text-sm">
                        {match question.get_value().question_type {
                            crate::domain::question::QuestionType::Objective => {
                                let opts = question.get_value().options.clone();
                                view! {
                                    <div class="grid gap-1">
                                        <p class="text-xs font-semibold text-muted-foreground">
                                            "Options:"
                                        </p>
                                        <For
                                            each=move || opts.clone()
                                            key=|o| o.id
                                            children=move |opt| {
                                                let correct = opt.is_correct;
                                                let text = opt.option_text.clone();
                                                view! {
                                                    <div class="flex items-center gap-2">
                                                        {if correct {
                                                            view! {
                                                                <span class="text-green-500 w-4 h-4 shrink-0 flex items-center justify-center [&>svg]:w-4 [&>svg]:h-4">
                                                                    {icon!(FiCheckCircle)}
                                                                </span>
                                                            }.into_any()
                                                        } else {
                                                            view! {
                                                                <span class="text-red-400 w-4 h-4 shrink-0 flex items-center justify-center [&>svg]:w-4 [&>svg]:h-4">
                                                                    {icon!(MdiCloseCircle)}
                                                                </span>
                                                            }.into_any()
                                                        }}
                                                        <span class:font-bengali=move || !text.is_ascii() class=if correct {
                                                            "text-green-600 font-medium"
                                                        } else {
                                                            "font-medium"
                                                        }>
                                                            {text.clone()}
                                                        </span>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }.into_any()
                            }
                            crate::domain::question::QuestionType::Subjective => {
                                let ans = question.get_value().answer_text.clone().unwrap_or_default();
                                view! {
                                    <div>
                                        <p class="text-xs font-semibold text-muted-foreground">
                                            "Answer:"
                                        </p>
                                        <p class="text-green-600">{ans}</p>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </Show>
            </div>
        </Card>
    }
}

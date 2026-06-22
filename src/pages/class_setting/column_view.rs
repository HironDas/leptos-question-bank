use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::IntoView;
use singlestage::*;

use crate::pages::class_setting::chapter_item::UpdateChapterDialog;
use crate::pages::class_setting::class_item::UpdateClassDialog;
use crate::pages::class_setting::subject_item::UpdateSubjectDialog;
use crate::server_function::academic_helper::chapter::subject_chapter;
use crate::server_function::academic_helper::chapter::UpdateChapter;
use crate::server_function::academic_helper::class::UpdateClass;
use crate::server_function::academic_helper::subject;
use crate::server_function::academic_helper::subject::UpdateSubject;
use crate::server_function::academic_setting::Chapter;
use crate::server_function::academic_setting::Class;
use crate::server_function::academic_setting::Subject;

#[component]

pub fn ColumnView(
    #[prop(into)] classes: Signal<Vec<Class>>,
    update_class_action: ServerAction<UpdateClass>,
) -> impl IntoView {
    println!("{:?}", classes);
    let selected_class_id: RwSignal<Option<u32>> = RwSignal::new(None);
    // let update_class_action = ServerAction::<UpdateClass>::new();
    // let update_class_value = update_class_action.value();

    let subjects: RwSignal<Vec<Subject>> = RwSignal::new(Vec::new());
    let selected_subject_id: RwSignal<Option<u32>> = RwSignal::new(None);

    let chapters: RwSignal<Vec<Chapter>> = RwSignal::new(Vec::new());
    let hide_chapter: RwSignal<bool> = RwSignal::new(false);

    let update_subject_action = ServerAction::<UpdateSubject>::new();
    let update_subject_value = update_subject_action.value();

    Effect::new(move || {
        // 1. Force Leptos to track this signal immediately on every single run
        let current_action_value = update_subject_value.get();
        // 2. Safely unpack the tracked value using standard pattern matching
        if let Some(Ok(updated_subject)) = current_action_value {
            subjects.update(move |subjects_vec| {
                if let Some(pos) = subjects_vec.iter().position(|c| c.id == updated_subject.id) {
                    subjects_vec[pos] = updated_subject;
                }
            });
        }
    });

    let update_chapter_action = ServerAction::<UpdateChapter>::new();
    let update_chapter_value = update_chapter_action.value();

    Effect::new(move || {
        // 1. Force Leptos to track this signal immediately on every single run
        let current_action_value = update_chapter_value.get();
        println!("Update Cahpter Value {:?}", current_action_value);
        // 2. Safely unpack the tracked value using standard pattern matching
        if let Some(Ok(updated_chapter)) = current_action_value {
            println!("chapter is updated===> {:?}", updated_chapter);

            chapters.update(move |chapters_vec| {
                if let Some(pos) = chapters_vec.iter().position(|c| c.id == updated_chapter.id) {
                    chapters_vec[pos] = updated_chapter;
                }
            });
        }
    });

    let subjects_resource: Resource<Vec<Subject>> = Resource::new(
        move || selected_class_id.get(),
        move |class_id| async move {
            if let Some(id) = class_id {
                log!("Fetching subjects for class id: {}", id);
                match subject::subject(id).await {
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

    let chapters_resource: Resource<Vec<Chapter>> = Resource::new(
        move || selected_subject_id.get(),
        move |subject_id| async move {
            if let Some(id) = subject_id {
                log!("Fetching chapters for subject id: {}", id);
                match subject_chapter(id).await {
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
        if let Some(chapters_data) = chapters_resource.get() {
            chapters.set(chapters_data);
        }
    });

    view! {
        <div class="px-4">
            <Breadcrumb>
                <BreadcrumbItem>
                    "Academic Setting"
                </BreadcrumbItem>
                <BreadcrumbSeparator class:hidden=move || selected_class_id.get().is_none() />
                <BreadcrumbItem>
                    { move || if let Some(id) = selected_class_id.get() {
                        log!("class id==>{}",id);
                        classes.get().iter().find(|class| class.id == id).map(|c| c.name.clone()).unwrap_or_else(|| "".to_string())
                    } else { "".to_string() } }
                </BreadcrumbItem>
                <BreadcrumbSeparator class:hidden=move || selected_subject_id.get().is_none() />
                <BreadcrumbItem>
                    { move || if let Some(id) = selected_subject_id.get() {
                        subjects.get().iter().find(|subject| subject.id == id).map(|s| s.title.clone()).unwrap_or_else(|| "".to_string())
                    } else { "".to_string() } }
                </BreadcrumbItem>
            </Breadcrumb>
        </div>
        <div class="flex w-full h-full divide-x">
            <div class="w-1/3 px-2">
                <div class="flex w-full max-w-md flex-col gap-2">
                       <ItemGroup class="divide-y">
                           <For each=move || classes.get() key=|class| class.id let(class)
                           children=move |class| {
                               let memo_class = Memo::new(move |_| {
                                   classes.get()
                                       .into_iter()
                                       .find(|c| c.id == class.id)
                                       .unwrap_or_else(|| class.clone())
                               });
                               view! {
                                   <ClassUpdateModal
                                       class=memo_class
                                       update_class_action=update_class_action
                                       selected_class_id
                                       hide_chapter
                                   />
                               }
                           }/>
                       </ItemGroup>
                   </div>
            // <Separator vertical=true/>
            </div>
            <div class="w-1/3 px-2">
                <div class="flex w-full max-w-md flex-col gap-2">
                   <ItemGroup class="divide-y">
                       <For each=move || subjects.get() key=|subject| subject.id children= move|subject|{
                           let memo_subject = Memo::new(move|_|{
                               subjects.get().into_iter().find(|s| s.id == subject.id).unwrap_or(subject.clone())
                           });

                           view!{
                               <SubjectUpdateModal subject=memo_subject update_subject_action selected_subject_id hide_chapter/>
                           }
                       }/>

                   </ItemGroup>
               </div>
            </div>
            // <Separator vertical=true/>

            <div class="w-1/3 px-2">
            <div class="flex w-full max-w-md flex-col gap-2">
               <ItemGroup class="divide-y" class:hidden = move||hide_chapter.get()>
                   <For each=move || chapters.get() key=|chapter| chapter.id
                   children=move |chapter| {
                       let memo_chapter = Memo::new(move |_| {
                           chapters
                               .get()
                               .into_iter()
                               .find(|c| c.id == chapter.id)
                               .unwrap_or_else(|| chapter.clone())
                       });
                       view! {
                           <ChapterUpdateModal
                               chapter=memo_chapter
                               update_chapter_action=update_chapter_action
                           />
                       }
                   }
                   />
               </ItemGroup>
           </div>
            </div>
        </div>
    }
}

#[component]
fn ChapterUpdateModal(
    chapter: Memo<Chapter>,
    update_chapter_action: ServerAction<UpdateChapter>,
) -> impl IntoView {
    let chapter_dialog_open = RwSignal::new(false);
    let chapter_input_ref = NodeRef::<html::Input>::new();

    let update_chapter_value = update_chapter_action.value();

    Effect::new(move || {
        if update_chapter_value
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
        {
            chapter_dialog_open.set(false);
        }
    });

    let is_bengali = Memo::new(move |_| !chapter.get().title.is_ascii());

    view! {
        <Item size="sm" class="border-b border-b-[var(--border)] last:border-b-0 rounded-none">
             <ItemContent class="block">
                 <ItemTitle class:font-bengali=move|| is_bengali.get()>{move||chapter.get().title}</ItemTitle>
                 <ItemDescription >{move||format!("chapter {} ({})",chapter.get().order, chapter.get().question_count)}</ItemDescription>

             </ItemContent>
             <ItemActions>
                <UpdateChapterDialog chapter_dialog_open chapter_input_ref update_chapter_action chapter/>
            </ItemActions>
        </Item>
    }
}

#[component]
fn ClassUpdateModal(
    class: Memo<Class>,
    update_class_action: ServerAction<UpdateClass>,
    selected_class_id: RwSignal<Option<u32>>,
    hide_chapter: RwSignal<bool>,
) -> impl IntoView {
    let class_dialog_open = RwSignal::new(false);

    let update_class_value = update_class_action.value();

    Effect::new(move || {
        if update_class_value.get().map(|v| v.is_ok()).unwrap_or(false) {
            class_dialog_open.set(false);
        }
    });

    let item_variant = Memo::new(move |_| {
        if let Some(id) = selected_class_id.get() {
            if class.get().id == id {
                "muted".to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    });

    view! {
        <a href="#" on:click=move|ev|{
            ev.prevent_default();
            selected_class_id.set(Some(class.get().id));
            hide_chapter.set(true);
            // chapters.set(Vec::new());
        }>
               <Item size="sm" variant=item_variant>
                   <ItemContent class="block">
                       <ItemTitle>{move||class.get().name}</ItemTitle>
                       <ItemDescription class="font-bengali">{move||class.get().name_bn}</ItemDescription>
                   </ItemContent>
                   <ItemActions>
                        <UpdateClassDialog class_dialog_update_open=class_dialog_open update_class_action class/>
                       {icon!(icondata::LuChevronRight, class="size-4")}
                   </ItemActions>
               </Item>
        </a>
    }
}

#[component]
fn SubjectUpdateModal(
    subject: Memo<Subject>,
    update_subject_action: ServerAction<UpdateSubject>,
    selected_subject_id: RwSignal<Option<u32>>,
    hide_chapter: RwSignal<bool>,
) -> impl IntoView {
    let subject_dialog_open = RwSignal::new(false);
    let input_ref = NodeRef::<html::Input>::new();

    let update_subject_value = update_subject_action.value();

    Effect::new(move || {
        if update_subject_value
            .get()
            .map(|v| v.is_ok())
            .unwrap_or(false)
        {
            subject_dialog_open.set(false);
        }
    });

    let item_variant = Memo::new(move |_| {
        if let Some(id) = selected_subject_id.get() {
            if subject.get().id == id {
                "muted".to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    });

    view! {
        <a href="#" on:click=move|ev|{
            ev.prevent_default();
            selected_subject_id.set(Some(subject.get().id));
            hide_chapter.set(false);
        }>{
            let is_bengali = Memo::new(move |_| !subject.get().title.is_ascii());
              view!{ <Item size="sm" variant=item_variant>
                   <ItemContent class="block">
                       <ItemTitle class:font-bengali=move|| is_bengali.get()>{move||subject.get().title}</ItemTitle>
                   </ItemContent>
                   <ItemActions>
                      <UpdateSubjectDialog input_ref subject_dialog_open subject update_subject_action/>
                       {icon!(icondata::LuChevronRight, class="size-4")}
                   </ItemActions>
               </Item>}
        }
        </a>
    }
}

use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::IntoView;
use singlestage::*;

use crate::pages::class_setting::chapter_item::UpdateChapterDialog;
use crate::server_function::academic_helper::chapter::subject_chapter;
use crate::server_function::academic_helper::chapter::UpdateChapter;
use crate::server_function::academic_helper::subject;
use crate::server_function::academic_setting::Chapter;
use crate::server_function::academic_setting::Class;
use crate::server_function::academic_setting::Subject;

#[component]
pub fn ColumnView(#[prop(into)] classes: Signal<Vec<Class>>) -> impl IntoView {
    println!("{:?}", classes);
    let selected_class_id: RwSignal<Option<u32>> = RwSignal::new(None);

    let subjects: RwSignal<Vec<Subject>> = RwSignal::new(Vec::new());
    let selected_subject_id: RwSignal<Option<u32>> = RwSignal::new(None);

    let chapters: RwSignal<Vec<Chapter>> = RwSignal::new(Vec::new());
    let hide_chapter: RwSignal<bool> = RwSignal::new(false);

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
                           <For each=move || classes.get() key=|class| class.id let(class)>
                                <a href="#" on:click=move|ev|{
                                    ev.prevent_default();
                                    selected_class_id.set(Some(class.id));
                                    hide_chapter.set(true);
                                    // chapters.set(Vec::new());
                                }>
                                       <Item size="sm">
                                           <ItemContent class="block">
                                               <ItemTitle>{class.name}</ItemTitle>
                                               <ItemDescription class="font-bengali">{class.name_bn}</ItemDescription>
                                           </ItemContent>
                                           <ItemActions>
                                               <Button variant="ghost" size="icon" class="rounded-full">
                                                   {icon!(icondata::AiEditOutlined)}
                                               </Button>
                                               {icon!(icondata::LuChevronRight, class="size-4")}
                                           </ItemActions>
                                       </Item>
                                </a>
                           </For>
                       </ItemGroup>
                   </div>
            // <Separator vertical=true/>
            </div>
            <div class="w-1/3 px-2">
                <div class="flex w-full max-w-md flex-col gap-2">
                   <ItemGroup class="divide-y">
                       <For each=move || subjects.get() key=|subject| subject.id let(subject)>
                            <a href="#" on:click=move|ev|{
                                ev.prevent_default();
                                selected_subject_id.set(Some(subject.id));
                                hide_chapter.set(false);
                            }>{
                                let is_bengali = !subject.title.is_ascii();
                                  view!{ <Item size="sm">
                                       <ItemContent class="block">
                                           <ItemTitle class:font-bengali=move|| is_bengali>{subject.title}</ItemTitle>
                                       </ItemContent>
                                       <ItemActions>
                                           <Button variant="ghost" size="icon" class="rounded-full">
                                               {icon!(icondata::AiEditOutlined)}
                                           </Button>
                                           {icon!(icondata::LuChevronRight, class="size-4")}
                                       </ItemActions>
                                   </Item>}
                            }
                            </a>
                       </For>
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
                               .unwrap_or(chapter.clone())
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

    let is_bengali = !chapter.get().title.is_ascii();

    view! {
        <Item size="sm" class="border-b border-b-[var(--border)] last:border-b-0 rounded-none">
             <ItemContent class="block">
                 <ItemTitle class:font-bengali=move|| is_bengali>{chapter.get().title}</ItemTitle>
                 <ItemDescription >{format!("chapter {} ({})",chapter.get().order, chapter.get().question_count)}</ItemDescription>

             </ItemContent>
             <ItemActions>
                <UpdateChapterDialog chapter_dialog_open chapter_input_ref update_chapter_action chapter/>
            </ItemActions>
        </Item>
    }
}

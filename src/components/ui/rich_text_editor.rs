use leptos::IntoView;
use leptos::{html, prelude::*};
use leptos_router::components::A;
use singlestage::*;

use crate::util::insert_latex_formula::insert_formula;
use crate::util::insert_latex_formula::MathSyntex;
use crate::util::insert_markdown_at_cursor::{insert_markdown, Syntex};
use crate::util::preview_markdown::MarkdownViewer;

#[component]
pub fn RichTextEditor(title: String, textarea_ref: NodeRef<html::Textarea>) -> impl IntoView {
    let table_row_ref = NodeRef::<html::Input>::new();
    let table_column_ref = NodeRef::<html::Input>::new();
    let preview = RwSignal::new(false);
    let (question_text, set_question_text) = signal(String::from(""));

    let formula_panel = RwSignal::new(false);

    let label_title = title.clone();

    view! {
        <div class="grid gap-2">
            <Label class="mb-2">{label_title}</Label>
            <div class="flex h-4 items-center space-x-2 text-sm">
            <Button on:click=move|ev| {
                ev.prevent_default();
                preview.update(move|val| *val = !*val);
                set_question_text.set(textarea_ref
                    .get()
                    .map(|el| el.value())
                    .unwrap_or_default());
            } variant="ghost">{move || if !preview.get(){"Preview"} else {"Continue Editing"}}</Button>
            <Show when=move||!preview.get() fallback= || view!{""}>
                <Separator vertical=true />
                <Tooltip value="Heading 1">
                <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Heading1)} variant="ghost" aria_label="Heading 1">{icon!(icondata::LuHeading1)}</Button>
                </Tooltip>
                <Tooltip value="Heading 2">
                <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Heading2)} variant="ghost" aria_label="Heading 2">{icon!(icondata::LuHeading2)}</Button>
                </Tooltip>
                <Tooltip value="Heading 3">
                <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Heading3)} variant="ghost" aria_label="Heading 3">{icon!(icondata::LuHeading3)}</Button>
                </Tooltip>
                <Separator vertical=true />
                <Tooltip value="Add bold text">
                <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Bold)} variant="ghost" aria_label="Toggle Bold">{icon!(icondata::LuBold)}</Button>
                </Tooltip>
                <Tooltip value="Add italic text">
                    <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Italic)} variant="ghost" aria_label="Toggle italic">{icon!(icondata::LuItalic)} </Button>
                </Tooltip>
                <Tooltip value="Add text underline">
                    <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Underscore)} variant="ghost" aria_label="Underline">{icon!(icondata::LuUnderline)} </Button>
                </Tooltip>
                <Tooltip value="Add Dash">
                    <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Dash)} variant="ghost" aria_label="Underline">{icon!(icondata::LuMinus)} </Button>
                </Tooltip>
                <Separator vertical=true />
                <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Bullet)} variant="ghost" aria_label="Bullet-point">{icon!(icondata::LuList)} </Button>
                <Button on:click=move|ev|{ ev.prevent_default(); insert_markdown(textarea_ref, Syntex::Order)} variant="ghost" aria_label="Ordered List">{icon!(icondata::LuListOrdered)} </Button>
                <Separator vertical=true />

                <Popover>
                    <PopoverTrigger>
                        <Button  button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTable)} </Button>
                    </PopoverTrigger>
                    <PopoverContent class="w-80">

                            <FieldSet>
                                <FieldGroup class="[&_input]:w-20 gap-2">
                                    <Field orientation="horizontal">
                                        <Tooltip side="right" align="center" value="Row">
                                            <Input node_ref=table_row_ref input_type="number" value="2" autofocus=true/>
                                        </Tooltip>
                                        <div>{icon!(icondata::LuX)}</div>
                                        <Tooltip side="left" align="center" value="Column">
                                            <Input node_ref=table_column_ref input_type="number" value="3"/>
                                        </Tooltip>
                                        <Button on:click=move|e|{
                                            e.prevent_default();
                                            let row = table_row_ref.get()
                                            .map(|el| el.value())
                                            .and_then(|val| val.parse::<u32>().ok())
                                            .unwrap_or(0);

                                            let column = table_column_ref.get()
                                            .map(|el| el.value())
                                            .and_then(|el| el.parse::<u32>().ok())
                                            .unwrap_or(0);
                                            insert_markdown(textarea_ref, Syntex::Table(row, column));
                                        }>"Add"</Button>
                                    </Field>

                                </FieldGroup>
                            </FieldSet>

                    </PopoverContent>
                </Popover>
                <Tooltip value="Add formula">
                <Button on:click=move|ev| {
                    ev.prevent_default();
                    insert_markdown(textarea_ref, Syntex::Formula);
                    formula_panel.set(true);
                } variant="ghost" aria_label="Underline">{icon!(icondata::LuSigma)} </Button>
                </Tooltip>
                <Separator vertical=true/>
                <Popover>
                    <PopoverTrigger>
                        <Button  button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTally5)} </Button>
                    </PopoverTrigger>
                    <PopoverContent class="w-70">

                            <FieldSet>
                                <FieldGroup class="gap-2">
                                    <Field orientation="horizontal">
                                        <Button on:click=move|ev| {
                                            ev.prevent_default();
                                            insert_markdown(textarea_ref, Syntex::Tally(1));
                                        }  button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTally1)} </Button>
                                        <Button on:click=move|ev| {
                                            ev.prevent_default();
                                            insert_markdown(textarea_ref, Syntex::Tally(2));
                                        } button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTally2)} </Button>
                                        <Button on:click=move|ev| {
                                            ev.prevent_default();
                                            insert_markdown(textarea_ref, Syntex::Tally(3));
                                        } button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTally3)} </Button>
                                        <Button on:click=move|ev| {
                                            ev.prevent_default();
                                            insert_markdown(textarea_ref, Syntex::Tally(4));
                                        }  button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTally4)} </Button>
                                        <Button on:click=move|ev| {
                                            ev.prevent_default();
                                            insert_markdown(textarea_ref, Syntex::Tally(5));
                                        } button_type="button" variant="ghost" aria_label="Add a Table">{icon!(icondata::LuTally5)} </Button>
                                    </Field>
                                </FieldGroup>
                            </FieldSet>

                    </PopoverContent>
                </Popover>
                </Show>

            </div>
            <Separator class="my-2" />
            <Show when = move || !preview.get() fallback = move||view!{
                <div class="flex min-h-[120px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50">
                    <MarkdownViewer content=question_text />
                </div>
            }>
            <div class="flex gap-2">
            <textarea
                name = title.clone().to_lowercase()
                node_ref=textarea_ref
                class="flex min-h-[120px] rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                class=("w-2/3",move||formula_panel.get())
                class:w-full= move ||!formula_panel.get()
                placeholder="Enter your question here..."
                rows="3"
                prop:value = question_text
                />
                <div class=move||if formula_panel.get() {"w-1/3"} else{"hidden"}>
                    <Card class="w-full h-full pt-0 gap-0">
                        <div class="flex flex-row justify-between w-full items-center">
                            <div class="m-2 flex">Formula <A target="_blank" href="https://latexeditor.lagrida.com/">{icon!(icondata::LuInfo, width=15, class="mx-2")}</A></div>
                            <Button  class="m-2" on:click=move|ev|{ev.prevent_default(); formula_panel.set(false);} variant="ghost">{icon!(icondata::LuCircleX)}</Button>
                        </div>
                        <CardContent>
                        <div class="grid grid-cols-8 gap-2">
                            // <Button variant="ghost">{icon!(icondata::LuPlus)}</Button>
                            // <Button variant="ghost">{icon!(icondata::LuMinus)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Times)} >{icon!(icondata::LuX)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Division)}>{icon!(icondata::LuDivide)}</Button>
                            // <Button variant="ghost">{icon!(icondata::LuEqual)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Neq)}>{icon!(icondata::LuEqualNot)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Approx)}>{icon!(icondata::LuEqualApproximately)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Pm)}>{icon!(icondata::LuDiff)}</Button>
                            // <Button variant="ghost">{icon!(icondata::LuPercent)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Frac)}>{icon!(icondata::MdiAbTesting)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Sub)}>{icon!(icondata::LuSubscript)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Sup)}>{icon!(icondata::LuSuperscript)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Root)}>{icon!(icondata::LuRadical)}</Button>
                            <Button variant="ghost" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::Pi)}>{icon!(icondata::LuPi)}</Button>
                        </div>
                        <Button class="w-full" on:click=move|ev|{ev.prevent_default(); insert_formula(textarea_ref, MathSyntex::NewLine)}>New Line {icon!(icondata::LuCornerDownRight)}</Button>
                        </CardContent>
                    </Card>
                </div>
            </div>
            </Show>
        </div>
    }
}

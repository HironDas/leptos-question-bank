use leptos::prelude::*;
use leptos::IntoView;
use pulldown_cmark::Event;
use pulldown_cmark::{Options, Parser};
use ratex_layout::{layout, LayoutOptions};
use ratex_svg::SvgOptions;

pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_MATH);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();

    let mut modified_events = Vec::new();

    parser.for_each(|event| match event {
        Event::DisplayMath(latex_text) => {
            if let Ok(svg_string) = compile_latex_to_svg(&latex_text) {
                let wrapped_html = format!(
                    "<div class='math-block my-4 text-center'>{}</div>",
                    svg_string
                );
                modified_events.push(Event::Html(wrapped_html.into()));
            } else {
                modified_events.push(Event::Text(latex_text))
            }
        }
        Event::InlineMath(latex_text) => {
            if let Ok(svg_string) = compile_latex_to_svg(&latex_text) {
                let wrapped_html = format!(
                    "<span class='inline-math mx-1 align-middle'>{}</span>",
                    svg_string
                );
                modified_events.push(Event::Html(wrapped_html.into()));
            } else {
                modified_events.push(Event::Text(latex_text));
            }
        }
        other_event => modified_events.push(other_event),
    });

    pulldown_cmark::html::push_html(&mut html_output, modified_events.into_iter());
    html_output
}

#[component]
pub fn MarkdownViewer(#[prop(into)] content: Signal<String>) -> impl IntoView {
    let theme_context = use_context::<ThemeProviderContext>();
    let html_content = Memo::new(move |_| render_markdown(&content.get()));

    let is_dark_mode = move || {
        if let Some(ctx) = theme_context.clone() {
            // Check if mode is explicitly Dark, or check the theme name directly
            // Adjust this matching arm based on your exact `Mode` or `Theme` enum variants
            ctx.mode.get() == Mode::Dark
        } else {
            false
        }
    };

    view! {
        <div class="prose dark:prose-invert max-w-none" class:invert-math=is_dark_mode inner_html=move||html_content.get()/>
    }
}

use ratex_svg::render_to_svg;
use singlestage::Mode;
use singlestage::ThemeProviderContext;

fn compile_latex_to_svg(latex: &str) -> Result<String, String> {
    // Step 1: Parse the TeX source string into an AST (Abstract Syntax Tree)
    let ast = ratex_parser::parse(latex).map_err(|e| format!("Parsing error: {:?}", e))?;
    let layout_option = LayoutOptions::default();
    let layout_box = layout(&ast, &layout_option);

    // Step 2: Compute spatial dimensions and coordinates (em sizing, baselines)
    // Pass display mode settings to determine inline vs block formatting blocks
    let display_list = ratex_layout::to_display_list(&layout_box);

    // Step 3: Configure SVG rules (enable glyph nesting for standalone output)
    let mut options = SvgOptions::default();
    options.embed_glyphs = true; // Prevents external webfont stylesheet dependencies
    options.font_size = 14.0;
    // Step 4: Serialize the calculated spatial layout directly into an SVG string
    // let mut svg_buffer = Vec::new();
    Ok(render_to_svg(&display_list, &options)) // .map_err(|e| format!("SVG Generation error: {:?}", e))?

    //String::from_utf8(svg_buffer).map_err(|e| format!("UTF-8 decoding error: {:?}", e))
}

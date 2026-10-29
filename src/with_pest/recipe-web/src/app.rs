use crate::scrollbars::Scrollbars;
use recipe_web::{HUMMUS, Ingredient, Inline, PASTA, Recipe};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{HtmlDialogElement, HtmlElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

const STORAGE_KEY: &str = "mise.recipe.draft.v1";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn save(source: &str) -> bool {
    storage().is_some_and(|storage| storage.set_item(STORAGE_KEY, source).is_ok())
}

#[derive(Properties, PartialEq)]
struct IconProps {
    name: &'static str,
}

#[function_component(Icon)]
fn icon(props: &IconProps) -> Html {
    let path = match props.name {
        "leaf" => "M20 4c-8-2-16 2-14 10 2 8 14 4 14-10ZM4 21 15 10M10 15v-5m0 5h5",
        "copy" => "M9 9h11v11H9zM15 5V3H3v12h2",
        "trash" => "M3 6h18M9 6V3h6v3M6 6l1 15h10l1-15M10 10v7m4-7v7",
        "book" => "M12 5C8 2 4 3 2 4v15c3-1 7-1 10 1 3-2 7-2 10-1V4c-2-1-6-2-10 1Zm0 0v15",
        "arrow" => "M5 12h14m-5-5 5 5-5 5",
        "clock" => "M12 8v5l3 2M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0Z",
        "people" => {
            "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2m20 0v-2a4 4 0 0 0-3-3.87M13 3.13a4 4 0 0 1 0 7.75M13 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
        }
        "check" => "m5 12 4 4L19 6",
        "close" => "m6 6 12 12M6 18 18 6",
        "print" => "M6 9V3h12v6M6 17H3V9h18v8h-3M6 14h12v7H6z",
        "code" => "m8 5-7 7 7 7m8-14 7 7-7 7m-3-16-3 18",
        "shield" => "m12 2 8 4v6c0 5-8 10-8 10S4 17 4 12V6l8-4Zm-4 10 3 3 5-6",
        _ => "M12 8v5m0 3v.01M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0Z",
    };
    html! { <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.65" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d={path}/></svg> }
}

#[function_component(App)]
pub fn app() -> Html {
    let source = use_state(|| {
        storage()
            .and_then(|s| s.get_item(STORAGE_KEY).ok().flatten())
            .unwrap_or_else(|| HUMMUS.to_owned())
    });
    let saved = use_state(|| storage().is_some());
    let toast = use_state(String::new);
    let dialog = use_node_ref();
    let guide_content = use_node_ref();
    let guide_open = use_state(|| false);
    let backdrop_pressed = use_mut_ref(|| false);
    let recipe = use_memo((*source).clone(), |source| Recipe::parse(source));

    let update = {
        let source = source.clone();
        let saved = saved.clone();
        Callback::from(move |text: String| {
            saved.set(save(&text));
            source.set(text);
        })
    };
    let oninput = {
        let update = update.clone();
        Callback::from(move |event: InputEvent| {
            update.emit(event.target_unchecked_into::<HtmlTextAreaElement>().value())
        })
    };
    let clear = {
        let update = update.clone();
        Callback::from(move |_| update.emit(String::new()))
    };
    let example = {
        let update = update.clone();
        Callback::from(move |event: Event| {
            let select = event.target_unchecked_into::<HtmlSelectElement>();
            let text = if select.value() == "pasta" {
                PASTA
            } else {
                HUMMUS
            };
            update.emit(text.to_owned());
            select.set_value("");
        })
    };
    let restore = {
        let update = update.clone();
        Callback::from(move |_| update.emit(HUMMUS.to_owned()))
    };
    let copy = {
        let source = source.clone();
        let toast = toast.clone();
        Callback::from(move |_| {
            let text = (*source).clone();
            let toast = toast.clone();
            spawn_local(async move {
                let Some(window) = web_sys::window() else {
                    return;
                };
                if !window.is_secure_context() {
                    toast.set(
                        "Copy needs HTTPS or localhost. Select the source and copy it manually."
                            .to_owned(),
                    );
                    return;
                }
                let result = JsFuture::from(window.navigator().clipboard().write_text(&text)).await;
                toast.set(
                    if result.is_ok() {
                        "Recipe copied to clipboard."
                    } else {
                        "Clipboard unavailable. Select the source and copy it manually."
                    }
                    .to_owned(),
                );
            });
        })
    };
    let open_guide = {
        let dialog = dialog.clone();
        let guide_open = guide_open.clone();
        Callback::from(move |_| {
            if let Some(dialog) = dialog.cast::<HtmlDialogElement>()
                && dialog.show_modal().is_ok()
            {
                guide_open.set(true);
            }
        })
    };
    let close_guide = {
        let dialog = dialog.clone();
        Callback::from(move |_| {
            if let Some(dialog) = dialog.cast::<HtmlDialogElement>() {
                dialog.close();
            }
        })
    };
    // Require both press and click to be outside the dialog. A selection or
    // scrollbar drag that starts inside must never dismiss the guide.
    let on_guide_pointerdown = {
        let dialog = dialog.clone();
        let backdrop_pressed = backdrop_pressed.clone();
        Callback::from(move |event: web_sys::PointerEvent| {
            *backdrop_pressed.borrow_mut() = dialog
                .cast::<HtmlDialogElement>()
                .is_some_and(|dialog| outside_dialog(&dialog, event.client_x(), event.client_y()));
        })
    };
    let on_guide_click = {
        let dialog = dialog.clone();
        let backdrop_pressed = backdrop_pressed.clone();
        Callback::from(move |event: MouseEvent| {
            let pressed_outside = std::mem::take(&mut *backdrop_pressed.borrow_mut());
            if pressed_outside
                && let Some(dialog) = dialog.cast::<HtmlDialogElement>()
                && outside_dialog(&dialog, event.client_x(), event.client_y())
            {
                dialog.close();
            }
        })
    };
    let on_guide_pointercancel = {
        let backdrop_pressed = backdrop_pressed.clone();
        Callback::from(move |_: web_sys::PointerEvent| *backdrop_pressed.borrow_mut() = false)
    };
    let on_guide_close = {
        let guide_open = guide_open.clone();
        Callback::from(move |_: Event| guide_open.set(false))
    };
    let print = Callback::from(move |_| {
        if let Some(window) = web_sys::window() {
            let _ = window.print();
        }
    });
    let dismiss_toast = {
        let toast = toast.clone();
        Callback::from(move |_| toast.set(String::new()))
    };

    html! {
        <>
        <Scrollbars revision={(*source).clone()} controls="page-scroll-root"/>
        <header class="site-header">
            <a class="brand" href="./" aria-label="Mise home"><span class="brand-mark"><Icon name="leaf"/></span>{"mise"}<span class="brand-period">{"."}</span><span class="brand-caption">{"a place for your recipes"}</span></a>
            <nav aria-label="Main navigation"><span class="nav-active">{"Playground"}</span><button class="nav-link" onclick={open_guide.clone()}><Icon name="book"/>{"Language guide"}</button><a class="github-link" href="https://github.com/ohmycloud/rust-parser" target="_blank" rel="noopener noreferrer">{"GitHub"}<span aria-hidden="true">{"↗"}</span></a></nav>
        </header>
        <main class="page">
            <section class="intro">
                <div><div class="eyebrow"><span class="tiny-line"/>{"THE RECIPE PLAYGROUND"}</div><h1>{"A little structure."}<br/><em>{"Something delicious."}</em></h1><p>{"Write naturally. Add a few ingredients. Watch your recipe come together."}</p></div>
                <div class="intro-note"><span class="rust-badge"><span class="status-dot"/>{"POWERED BY RUST + WASM"}</span><p>{"Just you, your recipe, and your browser."}<br/>{"No accounts. No uploads. No fuss."}</p></div>
            </section>
            <div class="workspace">
                <section class="source-panel" aria-labelledby="source-heading">
                    <div class="panel-heading"><div class="panel-title"><span class="section-number">{"01"}</span><h2 id="source-heading">{"Make it yours"}</h2><span class="file-type">{".recipe"}</span></div><label class="example-select"><span class="sr-only">{"Load an example (replaces current draft)"}</span><select onchange={example} value=""><option value="" disabled=true selected=true>{"Load an example"}</option><option value="hummus">{"Everyday hummus"}</option><option value="pasta">{"Tomato pasta"}</option></select></label></div>
                    <div class="editor-card">
                        <div class="editor-topbar"><span class="window-dots"><i/><i/><i/></span><span>{"your-next-favorite.recipe"}</span><span class="editor-language">{"RECIPE LANG"}</span></div>
                        <SourceEditor source={(*source).clone()} {oninput}/>
                        <div class="editor-status"><span><span class="status-dot"/>{if *saved { "Saved on this device" } else { "Local storage unavailable" }}</span><span>{format!("{} lines · {} characters", source.split('\n').count(), source.chars().count())}</span></div>
                    </div>
                    <div class="editor-actions"><button class="text-button guide-button" onclick={open_guide.clone()}><Icon name="book"/>{"A quick syntax guide"}<span aria-hidden="true">{"↗"}</span></button><div><button class="button subtle" onclick={clear} disabled={source.is_empty()}><Icon name="trash"/>{"Clear"}</button><button class="button" onclick={copy} disabled={source.is_empty()}><Icon name="copy"/>{"Copy source"}</button></div></div>
                    <aside class="tip"><span class="tip-symbol">{"✳"}</span><p><strong>{"Less formatting. More cooking."}</strong>{" Wrap an ingredient in "}<code>{"{braces}"}</code>{", add an amount, and leave the rest to us."}</p></aside>
                </section>
                <section class="preview-panel" aria-labelledby="preview-heading">
                    <div class="panel-heading"><div class="panel-title"><span class="section-number">{"02"}</span><h2 id="preview-heading">{"Ready for the kitchen"}</h2></div><span class="live-badge"><span class="status-dot"/>{"Live preview"}</span></div>
                    {if source.trim().is_empty() {
                        html!{<div class="empty-state"><span class="empty-icon"><Icon name="leaf"/></span><span class="eyebrow">{"A FRESH START"}</span><h2>{"Every good recipe"}<br/><em>{"starts somewhere."}</em></h2><p>{"Your ingredients, tools, and steps will appear here as you write."}</p><button class="button primary" onclick={restore}>{"Try the hummus recipe"}<Icon name="arrow"/></button></div>}
                    } else {
                        match &*recipe {
                            Ok(recipe) => html!{<RecipePreview recipe={recipe.clone()} onprint={print}/>},
                            Err(error) => html!{<div class="error-state" role="alert"><h3>{"A little hiccup in the recipe"}</h3><pre>{error}</pre><button class="button" onclick={open_guide.clone()}>{"Open syntax guide"}</button></div>},
                        }
                    }}
                </section>
            </div>
            <footer class="site-footer"><span><Icon name="leaf"/>{"A small language for good food."}</span><span><Icon name="shield"/>{"Parsed locally. Your recipes stay yours."}</span><span>{"BUILT WITH RUST"}<span class="footer-star">{"✳"}</span></span></footer>
        </main>
        <dialog ref={dialog} class="guide-dialog" aria-labelledby="guide-title" onclose={on_guide_close} onpointerdown={on_guide_pointerdown} onpointercancel={on_guide_pointercancel} onclick={on_guide_click}>
            <div class="guide-shell">
            <div ref={guide_content.clone()} id="guide-content" class="guide-content">
            <div class="dialog-header"><div><span class="eyebrow">{"A FEW SIMPLE INGREDIENTS"}</span><h2 id="guide-title">{"The recipe language"}</h2></div><button class="icon-button" aria-label="Close syntax guide" onclick={close_guide}><Icon name="close"/></button></div>
            <p class="dialog-description">{"Plain text, with a little extra meaning. These are all the building blocks you need."}</p>
            <div class="syntax-table">
                {for [
                    ("Name & metadata", ">> name: Everyday hummus", "Use a new line for each metadata field."),
                    ("Tags", ">> tags: vegan, quick, everyday", "Separate tags with commas."),
                    ("Ingredient", "{chickpeas}(400 g)", "The amount is optional: {salt} works too."),
                    ("Kitchen tool", "&{blender}", "Collect your equipment automatically."),
                    ("Timer", "t{3 minutes}", "Make cooking times easy to spot."),
                    ("Related recipe", "@{tomato sauce}(100 ml)", "Reference another recipe by name."),
                    ("Comment", "/* a note to yourself */", "Visible in the source, hidden in the recipe."),
                    ("Backstory", "\n---\nA story worth sharing.", "Put --- on its own line, with no trailing spaces."),
                ].into_iter().map(|(name, syntax, note)| html!{<div class="syntax-row"><strong>{name}</strong><div><code>{syntax}</code><p>{note}</p></div></div>})}
            </div>
            <p class="guide-note"><Icon name="info"/>{"Ingredient and tool names currently use the parser’s Latin-character syntax; Unicode prose is preserved. Unrecognized syntax is treated as plain text, not a validation error."}</p>
            </div>
            {if *guide_open {
                html!{<Scrollbars target={guide_content} revision="guide-open" controls="guide-content"/>}
            } else {Html::default()}}
            </div>
        </dialog>
        <div class={classes!("toast", if toast.is_empty() { "toast-hidden" } else { "" })} role="status" aria-live="polite"><Icon name="info"/><span>{(*toast).clone()}</span><button class="icon-button" aria-label="Dismiss notification" onclick={dismiss_toast}><Icon name="close"/></button></div>
        </>
    }
}

fn outside_dialog(dialog: &HtmlDialogElement, x: i32, y: i32) -> bool {
    let rect = dialog.get_bounding_client_rect();
    (x as f64) < rect.left()
        || (x as f64) > rect.right()
        || (y as f64) < rect.top()
        || (y as f64) > rect.bottom()
}

#[derive(Properties, PartialEq)]
struct SourceEditorProps {
    source: String,
    oninput: Callback<InputEvent>,
}

#[function_component(SourceEditor)]
fn source_editor(props: &SourceEditorProps) -> Html {
    let highlight = use_node_ref();
    let gutter = use_node_ref();
    let input = use_node_ref();
    let onscroll = {
        let highlight = highlight.clone();
        let gutter = gutter.clone();
        Callback::from(move |event: Event| {
            let textarea = event.target_unchecked_into::<HtmlTextAreaElement>();
            if let Some(pre) = highlight.cast::<HtmlElement>() {
                pre.set_scroll_top(textarea.scroll_top());
                pre.set_scroll_left(textarea.scroll_left());
            }
            if let Some(pre) = gutter.cast::<HtmlElement>() {
                pre.set_scroll_top(textarea.scroll_top());
            }
        })
    };
    html! {
        <div class="source-editor">
            <pre class="line-numbers" ref={gutter} aria-hidden="true">{(1..=props.source.split('\n').count()).map(|n| n.to_string()).collect::<Vec<_>>().join("\n")}{"\n"}</pre>
            <div class="editor-stack"><pre ref={highlight} class="syntax-highlight" aria-hidden="true">{for props.source.split('\n').map(|line| html!{<>{highlight_line(line)}{"\n"}</>})}</pre><textarea ref={input.clone()} id="recipe-source" aria-label="Recipe source" class="source-input" spellcheck="false" autocapitalize="off" autocomplete="off" wrap="off" value={props.source.clone()} oninput={props.oninput.clone()} {onscroll} placeholder={">> name: Your next favorite\n\nStart with {something delicious}."}/><Scrollbars target={input} revision={props.source.clone()} controls="recipe-source"/></div>
        </div>
    }
}

fn highlight_line(line: &str) -> Html {
    if line.trim_start().starts_with(">>") {
        return html! {<span class="syntax-meta">{line.to_owned()}</span>};
    }
    if line.trim_start().starts_with("/*") || line == "---" {
        return html! {<span class="syntax-comment">{line.to_owned()}</span>};
    }
    let mut nodes = Vec::new();
    let mut index = 0;
    let mut plain_start = 0;
    while index < line.len() {
        let rest = &line[index..];
        let class = if rest.starts_with("&{") {
            Some("syntax-tool")
        } else if rest.starts_with("t{") {
            Some("syntax-timer")
        } else if rest.starts_with("@{") {
            Some("syntax-ref")
        } else if rest.starts_with('{') {
            Some("syntax-ingredient")
        } else {
            None
        };
        if let Some(class) = class
            && let Some(close) = rest.find('}')
        {
            if plain_start < index {
                nodes.push(html! {<>{line[plain_start..index].to_owned()}</>});
            }
            let mut end = index + close + 1;
            if line[end..].starts_with('(')
                && let Some(close) = line[end..].find(')')
            {
                end += close + 1;
            }
            nodes.push(html! {<span class={class}>{line[index..end].to_owned()}</span>});
            index = end;
            plain_start = end;
            continue;
        }
        index += rest.chars().next().unwrap().len_utf8();
    }
    if plain_start < line.len() {
        nodes.push(html! {<>{line[plain_start..].to_owned()}</>});
    }
    html! {<>{for nodes}</>}
}

#[derive(Properties, PartialEq)]
struct RecipePreviewProps {
    recipe: Recipe,
    onprint: Callback<MouseEvent>,
}

#[function_component(RecipePreview)]
fn recipe_preview(props: &RecipePreviewProps) -> Html {
    let recipe = &props.recipe;
    html! {
        <article class="recipe-card">
            <div class="recipe-hero">
                <div class="recipe-hero-top"><span class="recipe-kicker"><Icon name="leaf"/>{"THE GOOD STUFF"}</span><button class="icon-button print-button" onclick={props.onprint.clone()} aria-label="Print recipe" title="Print recipe"><Icon name="print"/></button></div>
                <h2>{recipe.title()}</h2>
                {recipe.metadata.get("description").map(|description| html!{<p class="recipe-description">{description}</p>}).unwrap_or_default()}
                <div class="recipe-meta">{recipe.metadata.get("time").map(|time| html!{<span><Icon name="clock"/>{time}</span>}).unwrap_or_default()}{recipe.metadata.get("servings").map(|servings| html!{<span><Icon name="people"/>{format!("Serves {servings}")}</span>}).unwrap_or_default()}</div>
                <div class="recipe-tags">{for recipe.tags().iter().map(|tag| html!{<span>{tag.replace('-', " ")}</span>})}</div>
                <svg class="hero-drawing" viewBox="0 0 160 160" fill="none" aria-hidden="true"><circle cx="80" cy="82" r="55" stroke="currentColor" stroke-width="1"/><circle cx="80" cy="82" r="44" stroke="currentColor" stroke-width="1"/><path d="M54 85c8-24 42-24 52 0M62 95c7-14 30-14 37 0M68 105c5-8 18-8 23 0M80 58c-9-18 4-28 17-26-2 17-7 24-17 26Zm0 0c-3-17-15-23-25-17 3 12 12 18 25 17Z" stroke="currentColor" stroke-width="1.4" stroke-linecap="round"/></svg>
            </div>
            <div class="recipe-body">
                <div class="mise-grid">
                    <section class="ingredients"><div class="recipe-section-title"><h3>{"Ingredients"}</h3><span>{format!("{:02}", recipe.ingredients.len())}</span></div><p class="section-hint">{"A little prep goes a long way."}</p><div class="ingredient-list">{for recipe.ingredients.iter().map(ingredient_row)}</div>{if recipe.ingredients.is_empty() {html!{<p class="section-hint">{"Add {ingredients} to build your shopping list."}</p>}} else {Html::default()}}</section>
                    <section class="equipment"><div class="recipe-section-title"><h3>{"Kitchen tools"}</h3></div><ul>{for recipe.materials.iter().map(|tool| html!{<li><span class="tool-dash"/>{tool}</li>})}</ul>{if recipe.materials.is_empty() {html!{<p class="section-hint">{"No special tools yet."}</p>}} else {Html::default()}}{if !recipe.references.is_empty() {html!{<div class="related-recipes"><h4>{"Related recipes"}</h4>{for recipe.references.iter().map(|item| html!{<p>{format!("{} {}", item.name, item.amount())}</p>})}</div>}} else {Html::default()}}</section>
                </div>
                <section class="method"><div class="recipe-section-title"><h3>{"Let’s make it"}</h3><span>{format!("{} STEPS", recipe.steps.len())}</span></div><ol>{for recipe.steps.iter().enumerate().map(|(index, step)| html!{<li><span class="step-number">{format!("{:02}", index + 1)}</span><p>{for step.iter().map(inline_view)}</p></li>})}</ol>{if recipe.steps.is_empty() {html!{<p class="section-hint">{"Write your instructions in the editor to get started."}</p>}} else {Html::default()}}</section>
                {recipe.backstory.as_ref().map(|text| html!{<aside class="backstory"><span>{"A NOTE FROM THE KITCHEN"}</span><p>{text}</p></aside>}).unwrap_or_default()}
                <div class="recipe-bottom"><span>{"Made with a little mise."}</span><Icon name="leaf"/></div>
            </div>
        </article>
    }
}

fn ingredient_row(item: &Ingredient) -> Html {
    let amount = item.amount();
    let key = format!("{}|{amount}", item.name);
    html! {<label class="ingredient-row" {key}><input type="checkbox"/><span class="ingredient-name">{&item.name}</span><span class="ingredient-amount">{if amount.is_empty() { "as needed" } else { &amount }}</span></label>}
}

fn inline_view(inline: &Inline) -> Html {
    match inline {
        Inline::Text(text) => html! {<>{text}</>},
        Inline::Ingredient(text) => html! {<mark class="inline-ingredient">{text}</mark>},
        Inline::Material(text) => html! {<mark class="inline-tool">{text}</mark>},
        Inline::Timer(text) => html! {<mark class="inline-timer"><Icon name="clock"/>{text}</mark>},
        Inline::Reference(text) => html! {<mark class="inline-reference">{text}</mark>},
    }
}

//! Compact 20px thumbs. The native element still owns scrolling, selection,
//! wheel/touch behavior, and the editor's existing highlight synchronization.
use gloo_events::EventListener;
use recipe_web::scrollbar::{THUMB_LENGTH, scroll_position, thumb_offset};
use web_sys::{HtmlElement, PointerEvent};
use yew::prelude::*;

#[derive(Clone, Default, PartialEq)]
struct Metrics {
    x: f64,
    y: f64,
    max_x: f64,
    max_y: f64,
    width: f64,
    height: f64,
}

fn measure(target: &Option<NodeRef>) -> Metrics {
    if let Some(target) = target {
        let Some(element) = target.cast::<HtmlElement>() else {
            return Metrics::default();
        };
        return Metrics {
            x: element.scroll_left() as f64,
            y: element.scroll_top() as f64,
            max_x: (element.scroll_width() - element.client_width()).max(0) as f64,
            max_y: (element.scroll_height() - element.client_height()).max(0) as f64,
            width: element.client_width() as f64,
            height: element.client_height() as f64,
        };
    }
    let window = web_sys::window().unwrap();
    let root = window.document().unwrap().document_element().unwrap();
    Metrics {
        x: window.scroll_x().unwrap_or_default(),
        y: window.scroll_y().unwrap_or_default(),
        max_x: (root.scroll_width() - root.client_width()).max(0) as f64,
        max_y: (root.scroll_height() - root.client_height()).max(0) as f64,
        width: root.client_width() as f64,
        height: root.client_height() as f64,
    }
}

fn scroll_to(target: &Option<NodeRef>, vertical: bool, position: f64) {
    if let Some(target) = target {
        if let Some(element) = target.cast::<HtmlElement>() {
            if vertical {
                element.set_scroll_top(position.round() as i32);
            } else {
                element.set_scroll_left(position.round() as i32);
            }
        }
    } else if let Some(window) = web_sys::window() {
        let (x, y) = if vertical {
            (window.scroll_x().unwrap_or_default(), position)
        } else {
            (position, window.scroll_y().unwrap_or_default())
        };
        window.scroll_to_with_x_and_y(x, y);
    }
}

#[derive(Properties, PartialEq)]
pub struct ScrollbarsProps {
    #[prop_or_default]
    pub target: Option<NodeRef>,
    pub revision: String,
    pub controls: &'static str,
}

#[function_component(Scrollbars)]
pub fn scrollbars(props: &ScrollbarsProps) -> Html {
    let metrics = use_state_eq(Metrics::default);
    {
        let metrics = metrics.clone();
        use_effect_with(
            (props.target.clone(), props.revision.clone()),
            move |(target, _)| {
                metrics.set(measure(target));
                let update = {
                    let target = target.clone();
                    move |_: &web_sys::Event| metrics.set(measure(&target))
                };
                let window = web_sys::window().unwrap();
                let listener = if let Some(target) = target {
                    target
                        .cast::<HtmlElement>()
                        .map(|element| EventListener::new(&element, "scroll", update.clone()))
                } else {
                    Some(EventListener::new(&window, "scroll", update.clone()))
                };
                let resize = EventListener::new(&window, "resize", update);
                move || drop((listener, resize))
            },
        );
    }
    let callback = |vertical| {
        let target = props.target.clone();
        Callback::from(move |position| scroll_to(&target, vertical, position))
    };
    html! {
        <div class={classes!("compact-scrollbars", if props.target.is_some() { "editor-scrollbars" } else { "page-scrollbars" })} style={format!("--thumb-length: {THUMB_LENGTH}px")}>
            {if metrics.max_y > 0.0 {
                html!{<ScrollRail vertical=true position={metrics.y} maximum={metrics.max_y} viewport={metrics.height} controls={props.controls} onchange={callback(true)}/>}
            } else {Html::default()}}
            {if metrics.max_x > 0.0 {
                html!{<ScrollRail vertical=false position={metrics.x} maximum={metrics.max_x} viewport={metrics.width} controls={props.controls} onchange={callback(false)}/>}
            } else {Html::default()}}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ScrollRailProps {
    vertical: bool,
    position: f64,
    maximum: f64,
    viewport: f64,
    controls: &'static str,
    onchange: Callback<f64>,
}

#[function_component(ScrollRail)]
fn scroll_rail(props: &ScrollRailProps) -> Html {
    let track = use_node_ref();
    let grab = use_mut_ref(|| None::<f64>);
    let dragging = use_state(|| false);
    let move_to = {
        let track = track.clone();
        let grab = grab.clone();
        let onchange = props.onchange.clone();
        let vertical = props.vertical;
        let maximum = props.maximum;
        Callback::from(move |event: PointerEvent| {
            if let Some(offset) = *grab.borrow()
                && let Some(element) = track.cast::<HtmlElement>()
            {
                let rect = element.get_bounding_client_rect();
                let (pointer, length) = if vertical {
                    (event.client_y() as f64 - rect.top(), rect.height())
                } else {
                    (event.client_x() as f64 - rect.left(), rect.width())
                };
                onchange.emit(scroll_position(pointer, offset, length, maximum));
            }
        })
    };
    let onpointerdown = {
        let track = track.clone();
        let grab = grab.clone();
        let dragging = dragging.clone();
        let move_to = move_to.clone();
        let vertical = props.vertical;
        let maximum = props.maximum;
        let position = props.position;
        Callback::from(move |event: PointerEvent| {
            if event.button() != 0 {
                return;
            }
            let Some(element) = track.cast::<HtmlElement>() else {
                return;
            };
            event.prevent_default();
            let _ = element.focus();
            let rect = element.get_bounding_client_rect();
            let (pointer, length) = if vertical {
                (event.client_y() as f64 - rect.top(), rect.height())
            } else {
                (event.client_x() as f64 - rect.left(), rect.width())
            };
            let start = thumb_offset(position, maximum, length);
            *grab.borrow_mut() = Some(if (start..=start + THUMB_LENGTH).contains(&pointer) {
                pointer - start
            } else {
                THUMB_LENGTH / 2.0
            });
            let _ = element.set_pointer_capture(event.pointer_id());
            dragging.set(true);
            move_to.emit(event);
        })
    };
    let release = {
        let grab = grab.clone();
        let dragging = dragging.clone();
        Callback::from(move |_: PointerEvent| {
            *grab.borrow_mut() = None;
            dragging.set(false);
        })
    };
    let onkeydown = {
        let onchange = props.onchange.clone();
        let position = props.position;
        let maximum = props.maximum;
        let viewport = props.viewport;
        let vertical = props.vertical;
        Callback::from(move |event: KeyboardEvent| {
            let next = match event.key().as_str() {
                "Home" => 0.0,
                "End" => maximum,
                "PageUp" => position - viewport,
                "PageDown" => position + viewport,
                "ArrowUp" if vertical => position - 40.0,
                "ArrowDown" if vertical => position + 40.0,
                "ArrowLeft" if !vertical => position - 40.0,
                "ArrowRight" if !vertical => position + 40.0,
                _ => return,
            };
            event.prevent_default();
            onchange.emit(next.clamp(0.0, maximum));
        })
    };
    let progress = (props.position / props.maximum).clamp(0.0, 1.0) * 100.0;
    let style = if props.vertical {
        format!("top: {progress}%; transform: translateY(-{progress}%);")
    } else {
        format!("left: {progress}%; transform: translateX(-{progress}%);")
    };
    html! {
        <div ref={track} class={classes!("scrollbar-rail", if props.vertical {"scrollbar-vertical"} else {"scrollbar-horizontal"}, (*dragging).then_some("is-dragging"))}
            role="scrollbar" tabindex="0" aria-label={if props.vertical {"Scroll vertically"} else {"Scroll horizontally"}}
            aria-controls={props.controls} aria-orientation={if props.vertical {"vertical"} else {"horizontal"}}
            aria-valuemin="0" aria-valuemax={props.maximum.round().to_string()} aria-valuenow={props.position.clamp(0.0, props.maximum).round().to_string()}
            {onpointerdown} onpointermove={move_to} onpointerup={release.clone()} onpointercancel={release.clone()} onlostpointercapture={release} {onkeydown}>
            <span class="scrollbar-thumb" {style}/>
        </div>
    }
}

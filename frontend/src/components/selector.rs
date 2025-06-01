use crate::hooks::mouseout_click::use_mouseout_click;
use std::{collections::HashMap, fmt::Display, hash::Hash, rc::Rc};
use yew::prelude::*;

#[derive(PartialEq)]
pub enum SelectionMode<T>
where
    T: Clone + PartialEq + 'static,
{
    Single(Callback<T>),
    Multiple(Callback<Vec<T>>),
}

#[derive(Properties, PartialEq)]
pub struct SelectorProps<T>
where
    T: Eq + Hash + Display + Clone + PartialEq + 'static,
{
    pub source_map: Rc<HashMap<T, String>>,
    pub title: String,
    pub uri: String,
    pub selection: SelectionMode<T>,
}

fn matches_fuzzy(query: &str, text: &str) -> bool {
    let mut query_chars = query.chars();
    let mut current = query_chars.next();

    for c in text.chars() {
        if let Some(qc) = current {
            if qc == c {
                current = query_chars.next();
            }
        } else {
            break;
        }
    }
    current.is_none()
}

#[function_component(Selector)]
pub fn selector<T>(props: &SelectorProps<T>) -> Html
where
    T: Eq + Hash + Display + Clone + PartialEq + 'static + Ord,
{
    let source_map = props.source_map.clone();
    let mut tuples = source_map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();

    match &props.selection {
        SelectionMode::Single(_) => tuples.sort_by(|a, b| a.0.cmp(&b.0)),
        SelectionMode::Multiple(_) => tuples.sort_by(|a, b| a.1.cmp(&b.1)),
    }

    let selected_vec = use_state(|| Vec::<T>::new());
    let dropdown_ref = use_node_ref();
    let is_open = use_state(|| false);
    let search_query = use_state(|| "".to_string());

    let button_ref = {
        let is_open = is_open.clone();
        use_mouseout_click(
            Callback::from(move |_| is_open.set(false)),
            vec![dropdown_ref.clone()],
        )
    };

    let dropdown_button_style =
        "w-full flex items-center gap-2 p-1.5 cursor-pointer odd:bg-custom-900 even:bg-zinc-950";
    let img_style = "h-5 w-5 aspect-square";
    let text_style = "text-sm truncate";

    let filtered_tuples = {
        let query = search_query.to_lowercase();
        tuples
            .into_iter()
            .filter(|(_, label)| matches_fuzzy(&query, &label.to_lowercase()))
            .collect::<Vec<_>>()
    };

    html! {
        <div class="relative w-full">
            <input
                ref={button_ref}
                type="text"
                class="mt-2 bg-custom-900 text-sm h-8 px-3 text-white w-full text-center"
                placeholder={props.title.clone()}
                value={(*search_query).clone()}
                onfocus={{
                    let is_open = is_open.clone();
                    Callback::from(move |_| is_open.set(true))
                }}
                oninput={{
                    let search_query = search_query.clone();
                    Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                        search_query.set(input);
                    })
                }}
            />

            <div
                ref={dropdown_ref}
                class={format!(
                    "absolute top-12 w-full flex-col z-10 max-h-64 overflow-y-auto bg-zinc-900 {}",
                    if *is_open { "flex" } else { "hidden" }
                )}
            >
                {
                    filtered_tuples.into_iter().map(|(key, value)| {
                        let key = key.clone();
                        let img_src = format!("{}/{}.png", props.uri, key);

                        match &props.selection {
                            SelectionMode::Single(callback) => {
                                let callback = callback.clone();
                                html! {
                                    <button
                                        onclick={Callback::from(move |_| {
                                            callback.emit(key.clone());
                                        })}
                                        class={dropdown_button_style}
                                    >
                                        <img src={img_src.clone()} class={img_style}/>
                                        <span class={text_style}>{ value }</span>
                                    </button>
                                }
                            }
                            SelectionMode::Multiple(callback) => {
                                let callback = callback.clone();
                                let selected_vec = selected_vec.clone();
                                html! {
                                    <button
                                        onclick={Callback::from(move |_| {
                                            let mut vec = (*selected_vec).clone();
                                            if !vec.contains(&key) {
                                                vec.push(key.clone());
                                                selected_vec.set(vec.clone());
                                                callback.emit(vec);
                                            }
                                        })}
                                        class={dropdown_button_style}
                                    >
                                        <img src={img_src.clone()} class={img_style}/>
                                        <span class={text_style}>{ value }</span>
                                    </button>
                                }
                            }
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

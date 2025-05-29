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
        SelectionMode::Single(_) => {
            tuples.sort_by(|a, b| a.0.cmp(&b.0));
        }
        SelectionMode::Multiple(_) => {
            tuples.sort_by(|a, b| a.1.cmp(&b.1));
        }
    }

    let selected_vec = use_state(|| Vec::<T>::new());
    let dropdown_ref = use_node_ref();
    let is_open = use_state(|| false);

    let button_ref = {
        let is_open = is_open.clone();
        use_mouseout_click(
            Callback::from(move |_| is_open.set(false)),
            vec![dropdown_ref.clone()],
        )
    };

    let button_style = "w-full flex items-center gap-2 p-1 even:bg-slate-800 odd:bg-slate-900";
    let img_style = "h-5 w-5 aspect-square";
    let text_style = "text-sm";

    html! {
        <div class="relative w-full">
            <button
                onclick={Callback::from({
                    let is_open = is_open.clone();
                    move |_| is_open.set(!*is_open)
                })}
                ref={button_ref}
                class="bg-slate-900 h-8 gap-2 w-full flex items-center justify-center"
            >
                <span class="text-shadow">{ props.title.clone() }</span>
            </button>
            <div
                ref={dropdown_ref}
                class={format!(
                    "absolute top-8 w-full flex-col z-10 max-h-64 overflow-y-auto bg-slate-700 {}",
                    if *is_open { "flex" } else { "hidden" }
                )}
            >
                {
                    tuples.into_iter().map(|(key, value)| {
                        let key = key.clone();
                        let img_src = format!("{}/{}.png", props.uri, key);

                        match &props.selection {
                            SelectionMode::Single(callback) => {
                                let callback = callback.clone();
                                // let is_open = is_open.clone();
                                html! {
                                    <button
                                        onclick={Callback::from(move |_| {
                                            callback.emit(key.clone());
                                            // is_open.set(false);
                                        })}
                                        class={button_style}
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
                                        class={button_style}
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

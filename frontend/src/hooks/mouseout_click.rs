use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, Node};
use yew::prelude::*;

/// Hook que retorna um NodeRef e dispara callback ao clicar fora dele e das exceções
#[hook]
pub fn use_mouseout_click(callback: Callback<()>, exceptions: Vec<NodeRef>) -> NodeRef {
    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        let callback = callback.clone();
        let exceptions = exceptions.clone();

        use_effect_with((node_ref.clone(), exceptions.clone()), move |_| {
            let listener = EventListener::new(
                &web_sys::window().unwrap(),
                "mousedown",
                move |event: &Event| {
                    let target = match event.target() {
                        Some(t) => t,
                        None => return,
                    };

                    let target_node: &Node = match target.dyn_ref::<Node>() {
                        Some(n) => n,
                        None => return,
                    };

                    // Verifica se clicou no elemento principal (node_ref)
                    let clicked_in_main = node_ref
                        .cast::<Element>()
                        .map(|el| el.contains(Some(target_node)))
                        .unwrap_or(false);

                    // Verifica se clicou em alguma das exceções
                    let clicked_in_exceptions = exceptions.iter().any(|r| {
                        r.cast::<Element>()
                            .map(|el| el.contains(Some(target_node)))
                            .unwrap_or(false)
                    });

                    // Só dispara o callback se não clicou nem no elemento principal nem nas exceções
                    if !clicked_in_main && !clicked_in_exceptions {
                        callback.emit(());
                    }
                },
            );

            || drop(listener)
        });
    }

    node_ref
}

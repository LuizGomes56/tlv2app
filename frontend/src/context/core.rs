use reqwasm::http::Request;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
use wasm_bindgen_futures::spawn_local;
use yew::{html::ChildrenProps, prelude::*};

use crate::BACKEND_URL;
use crate::{model::server::ServerResponse, pages::formulas::APIFormulas};

#[derive(Clone, PartialEq)]
pub struct CoreContext {
    pub static_formulas: Rc<RefCell<HashMap<String, APIFormulas>>>,
    pub static_champions: UseStateHandle<Rc<HashMap<String, String>>>,
    pub static_items: UseStateHandle<Rc<HashMap<usize, String>>>,
    pub static_runes: UseStateHandle<Rc<HashMap<usize, String>>>,
}

impl CoreContext {
    pub fn get_formulas(&self) -> &Rc<RefCell<HashMap<String, APIFormulas>>> {
        &self.static_formulas
    }

    pub fn get_static_champions(&self) -> &Rc<HashMap<String, String>> {
        &self.static_champions
    }

    pub fn get_static_items(&self) -> &Rc<HashMap<usize, String>> {
        &self.static_items
    }

    pub fn get_static_runes(&self) -> &Rc<HashMap<usize, String>> {
        &self.static_runes
    }
}

#[function_component(CoreProvider)]
pub fn core_provider(props: &ChildrenProps) -> Html {
    let champions_state = use_state(|| Rc::<HashMap<String, String>>::new(HashMap::new()));
    let items_state = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));
    let runes_state = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));
    let formulas_cell =
        Rc::<RefCell<HashMap<String, APIFormulas>>>::new(RefCell::new(HashMap::new()));

    {
        let all_champions = champions_state.clone();
        let all_items = items_state.clone();
        let all_runes = runes_state.clone();

        use_effect_with((), move |_| {
            if all_champions.is_empty() && all_items.is_empty() && all_runes.is_empty() {
                spawn_local(async move {
                    if let Ok(response) =
                        Request::get(&format!("{}/api/static/champions", BACKEND_URL))
                            .send()
                            .await
                    {
                        if let Ok(ServerResponse { data, .. }) = response
                            .json::<ServerResponse<HashMap<String, String>>>()
                            .await
                        {
                            all_champions.set(Rc::new(data));
                        } else {
                            web_sys::console::error_1(&"Erro ao decodificar campeões".into());
                        }
                    } else {
                        web_sys::console::error_1(&"Erro ao requisitar campeões".into());
                    }

                    if let Ok(response) = Request::get(&format!("{}/api/static/items", BACKEND_URL))
                        .send()
                        .await
                    {
                        if let Ok(ServerResponse { data, .. }) = response
                            .json::<ServerResponse<HashMap<usize, String>>>()
                            .await
                        {
                            all_items.set(Rc::new(data));
                        } else {
                            web_sys::console::error_1(&"Erro ao decodificar itens".into());
                        }
                    } else {
                        web_sys::console::error_1(&"Erro ao requisitar itens".into());
                    }

                    if let Ok(response) = Request::get(&format!("{}/api/static/runes", BACKEND_URL))
                        .send()
                        .await
                    {
                        if let Ok(ServerResponse { data, .. }) = response
                            .json::<ServerResponse<HashMap<usize, String>>>()
                            .await
                        {
                            all_runes.set(Rc::new(data));
                        } else {
                            web_sys::console::error_1(&"Erro ao decodificar runas".into());
                        }
                    } else {
                        web_sys::console::error_1(&"Erro ao requisitar runas".into());
                    }
                });
            }
            || ()
        });
    }

    html! {
        <ContextProvider<CoreContext> context={CoreContext {
            static_formulas: formulas_cell,
            static_champions: champions_state,
            static_items: items_state,
            static_runes: runes_state,
        }}>
            { props.children.clone() }
        </ContextProvider<CoreContext>>
    }
}

use reqwest::Client;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::{html::ChildrenProps, prelude::*};

use crate::{model::server::ServerResponse, pages::formulas::APIFormulas};

#[derive(Clone, PartialEq)]
pub struct CoreContext {
    pub static_formulas: UseStateHandle<Rc<HashMap<String, APIFormulas>>>,
    pub static_champions: UseStateHandle<Rc<HashMap<String, String>>>,
    pub static_items: UseStateHandle<Rc<HashMap<usize, String>>>,
    pub static_runes: UseStateHandle<Rc<HashMap<usize, String>>>,
}

impl CoreContext {
    pub fn get_formulas(&self) -> &Rc<HashMap<String, APIFormulas>> {
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
    let formulas_state = use_state(|| Rc::<HashMap<String, APIFormulas>>::new(HashMap::new()));
    let champions_state = use_state(|| Rc::<HashMap<String, String>>::new(HashMap::new()));
    let items_state = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));
    let runes_state = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));

    {
        let formulas_state = formulas_state.clone();
        use_effect_with((), move |_| {
            if
            /* NEVER (!) */
            !formulas_state.is_empty() {
                spawn_local(async move {
                    match reqwest::get("http://localhost:8082/api/formulas/champions").await {
                        Ok(response) => {
                            match response
                                .json::<ServerResponse<HashMap<String, APIFormulas>>>()
                                .await
                            {
                                Ok(res) => {
                                    formulas_state.set(Rc::new(res.data));
                                }
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &"Erro ao decodificar fórmulas".into(),
                                    );
                                    web_sys::console::error_1(&format!("{:?}", e).into());
                                }
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_1(&"Erro na requisição de fórmulas".into());
                            web_sys::console::error_1(&format!("{:?}", e).into());
                        }
                    }
                });
            }
            || ()
        });
    }

    {
        let all_champions = champions_state.clone();
        let all_items = items_state.clone();
        let all_runes = runes_state.clone();

        use_effect_with((), move |_| {
            if all_champions.is_empty() && all_items.is_empty() && all_runes.is_empty() {
                let client = Client::new();

                spawn_local(async move {
                    if let Ok(response) = client
                        .get("http://localhost:8082/api/static/champions")
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

                    if let Ok(response) = client
                        .get("http://localhost:8082/api/static/items")
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

                    if let Ok(response) = client
                        .get("http://localhost:8082/api/static/runes")
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
            static_formulas: formulas_state,
            static_champions: champions_state,
            static_items: items_state,
            static_runes: runes_state,
        }}>
            { props.children.clone() }
        </ContextProvider<CoreContext>>
    }
}

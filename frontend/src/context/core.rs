use reqwasm::http::Request;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::cell::RefMut;
use std::hash::Hash;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::{html::ChildrenProps, prelude::*};

use crate::BACKEND_URL;
use crate::{model::server::ServerResponse, pages::formulas::APIFormulas};

type RcState<T> = UseStateHandle<Rc<T>>;

#[derive(Clone, PartialEq)]
pub struct CoreContext {
    pub static_formulas: Rc<RefCell<HashMap<String, APIFormulas>>>,
    pub static_champions: RcState<HashMap<String, String>>,
    pub static_items: RcState<HashMap<usize, String>>,
    pub static_runes: RcState<HashMap<usize, String>>,
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

async fn get_static_instance<T>(path_name: &str, state_handle: RcState<T>)
where
    T: DeserializeOwned,
{
    match Request::get(&format!("{}/api/static/{}", BACKEND_URL, path_name))
        .send()
        .await
    {
        Ok(response) => match response.json::<ServerResponse<T>>().await {
            Ok(ServerResponse { data, .. }) => {
                state_handle.set(Rc::new(data));
            }
            Err(e) => console::log_1(
                &format!(
                    "Error decoding {} at get_static_instance: {:#?}",
                    path_name, e
                )
                .into(),
            ),
        },
        Err(e) => {
            console::log_1(&format!("Error sending request for {}: {:#?}", path_name, e).into())
        }
    }
}

#[function_component(CoreProvider)]
pub fn core_provider(props: &ChildrenProps) -> Html {
    let static_champions = use_state(|| Rc::<HashMap<String, String>>::new(HashMap::new()));
    let static_items = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));
    let static_runes = use_state(|| Rc::<HashMap<usize, String>>::new(HashMap::new()));
    let formulas_cell =
        Rc::<RefCell<HashMap<String, APIFormulas>>>::new(RefCell::new(HashMap::new()));

    {
        let static_champions = static_champions.clone();
        let static_items = static_items.clone();
        let static_runes = static_runes.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                get_static_instance("champions", static_champions).await;
                get_static_instance("items", static_items).await;
                get_static_instance("runes", static_runes).await;
            });
            || ()
        });
    }

    html! {
        <ContextProvider<CoreContext> context={CoreContext {
            static_formulas: formulas_cell,
            static_champions,
            static_items,
            static_runes,
        }}>
            { props.children.clone() }
        </ContextProvider<CoreContext>>
    }
}

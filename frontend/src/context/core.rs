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

type RcRef<K, V> = Rc<RefCell<HashMap<K, V>>>;

#[derive(Clone, PartialEq)]
pub struct CoreContext {
    pub static_formulas: RcRef<String, APIFormulas>,
    pub static_champions: RcRef<String, String>,
    pub static_items: RcRef<usize, String>,
    pub static_runes: RcRef<usize, String>,
}

impl CoreContext {
    pub fn get_formulas(&self) -> &RcRef<String, APIFormulas> {
        &self.static_formulas
    }

    pub fn get_static_champions(&self) -> &RcRef<String, String> {
        &self.static_champions
    }

    pub fn get_static_items(&self) -> &RcRef<usize, String> {
        &self.static_items
    }

    pub fn get_static_runes(&self) -> &RcRef<usize, String> {
        &self.static_runes
    }
}

async fn get_static_instance<T>(path_name: &str, mut ref_mut: RefMut<'_, T>)
where
    T: DeserializeOwned,
{
    match Request::get(&format!("{}/api/static/{}", BACKEND_URL, path_name))
        .send()
        .await
    {
        Ok(response) => match response.json::<ServerResponse<T>>().await {
            Ok(ServerResponse { data, .. }) => {
                *ref_mut = data;
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
    let static_champions =
        Rc::<RefCell<HashMap<String, String>>>::new(RefCell::new(HashMap::new()));
    let static_items = Rc::<RefCell<HashMap<usize, String>>>::new(RefCell::new(HashMap::new()));
    let static_runes = Rc::<RefCell<HashMap<usize, String>>>::new(RefCell::new(HashMap::new()));
    let formulas_cell =
        Rc::<RefCell<HashMap<String, APIFormulas>>>::new(RefCell::new(HashMap::new()));

    {
        let all_champions = static_champions.clone();
        let all_items = static_items.clone();
        let all_runes = static_runes.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if all_champions.borrow().is_empty() {
                    get_static_instance("champions", all_champions.borrow_mut()).await;
                }
                if all_items.borrow().is_empty() {
                    get_static_instance("items", all_items.borrow_mut()).await;
                }
                if all_runes.borrow().is_empty() {
                    get_static_instance("runes", all_runes.borrow_mut()).await;
                }
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

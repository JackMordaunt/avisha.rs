#![allow(unused)]
#![recursion_limit = "1024"]

use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::rc::Rc;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

use crate::tenant::{Tenant, TenantList};

const KEY: &str = "yew.avisha.self";

pub struct App {
    state: State,
    storage: StorageService,
    link: ComponentLink<Self>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct State {
    pub debug: bool,
    pub tenants: HashSet<Tenant>,
}

pub enum Msg {
    ToggleDebug,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);

        let state = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                restored_model
            } else {
                State::default()
            }
        };

        App {
            link,
            state,
            storage,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDebug => {
                self.state.debug = !self.state.debug;
            }
            Msg::Nope => {}
        };

        self.storage.store(KEY, Json(&self.state));
        true
    }

    fn view(&self) -> Html {
        let debug = if self.state.debug { "debug" } else { "" };
        let toggle_debug = self.link.callback(|_| Msg::ToggleDebug);

        let tenants = Rc::new(self.state.tenants.clone());

        html! {
            <grid class=debug>
                <row>
                    <nav>
                        <brand>
                            <h1 class="logo">
                                {"Avisha"}
                            </h1>
                        </brand>
                        <div>
                            <button class="small" onclick=toggle_debug>
                                {"Debug"}
                            </button>
                        </div>
                    </nav>
                </row>
                <row>
                    <col>
                        <TenantList tenants=&tenants />
                    </col>
                </row>
            </grid>
        }
    }
}

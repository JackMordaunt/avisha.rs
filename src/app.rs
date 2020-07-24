use crate::tenant_form::{Form as TenantForm, Model as TenantFormModel};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::rc::Rc;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.avisha.self";

pub type ID = String;

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub struct Tenant {
    pub id: ID,
    pub name: String,
    pub contact: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub struct Site {
    pub number: String,
    pub kind: SiteKind,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub enum SiteKind {
    Cabin,
    Flat,
    House,
    Other(String),
}

pub struct App {
    state: State,
    storage: StorageService,
    link: ComponentLink<Self>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct State {
    pub debug: bool,
    pub tenants: HashSet<Tenant>,
    pub sites: HashSet<Site>,
}

pub enum Msg {
    RegisterTenant(TenantFormModel),
    ToggleDebug,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("no local storage");

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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RegisterTenant(TenantFormModel { name, contact }) => {
                // Note: create a new ID, whatever else needs to be done to intro a new tenant.
                // In this case id is just the tenant name, assuming tenant name must be unique.

                let tenant = Tenant {
                    id: name.clone(),
                    name,
                    contact,
                };

                if !self.state.tenants.contains(&tenant) {
                    self.state.tenants.insert(tenant);
                }
            }
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

        html! {
            <div class=debug>

                <div class="nav">
                    <h1 class="nav-logo">
                        {"Avisha"}
                    </h1>
                    <div class="nav-item">
                        <button onclick=toggle_debug>
                            {"Debug"}
                        </button>
                    </div>
                </div>

                <div class="container">
                    <div class="row padded">
                        <div class="col">
                            <div class="cards">
                                <div class="card">
                                    <h5 class="card-header">
                                        {"Register Tenant"}
                                    </h5>
                                    <div class="card-body padded">
                                        <TenantForm
                                            submit=self.link.callback(|v| Msg::RegisterTenant(v))
                                        />
                                    </div>
                                </div>
                                <div class="card">
                                    <h5 class="card-header">
                                        {"Register Site"}
                                    </h5>
                                    <div class="card-body">
                                        // <SiteForm/>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="row">
                        <div class="col">
                            {self.tenant_list()}
                        </div>
                        <div class="col">
                            {self.site_list()}
                        </div>
                    </div>
                </div>

            </div>
        }
    }
}

impl App {
    fn tenant_list(&self) -> Html {
        html! {
            <div class="card">
                <h5 class="card-header">
                    {"Tenants"}
                </h5>
                <div class="card-body">
                    <list>
                        {for self.state.tenants.iter().map(|t| html!{
                            <item class="side padded">
                                <p>{format!("Name: {}", &t.name)}</p>
                                <p>{format!("Contact: {}", &t.contact)}</p>
                            </item>
                        })}
                    </list>
                </div>
            </div>
        }
    }

    fn site_list(&self) -> Html {
        html! {
            <div class="card">
                <h5 class="card-header">
                    {"Sites"}
                </h5>
                <div class="card-body">
                    <list>
                        {for self.state.sites.iter().map(|s| html!{
                            <item class="side padded">
                                <p>{format!("Number: {}", &s.number)}</p>
                                <p>{format!("kind: {:?}", &s.kind)}</p>
                            </item>
                        })}
                    </list>
                </div>
            </div>
        }
    }
}

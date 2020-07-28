use crate::site_form::{self, Form as SiteForm, Model as SiteFormModel};
use crate::tenant_form::{self, Form as TenantForm, Model as TenantFormModel};
use crate::validate::{SiteValidator, TenantValidator, Validate};
use std::fmt;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

const KEY: &str = "yew.avisha.self";

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub struct Tenant {
    pub name: String, // primary key
    pub contact: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub struct Site {
    pub number: String, // primary key
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
    pub tenants: HashMap<String, Tenant>,
    pub sites: HashMap<String, Site>,
    pub errors: Vec<String>,
}

pub enum Msg {
    RegisterTenant(TenantFormModel),
    ListSite(SiteFormModel),
    DismissErr(usize),
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
                // TODO: Move validation into validation object. 
                if name.is_empty() {
                    return self.error(format!("tenant name must be non-zero"));
                }

                if self.state.tenants.contains_key(&name) {
                    return self.error(format!("tenant name must be unique"));
                };

                self.state
                    .tenants
                    .insert(name.clone(), Tenant { name, contact });
            }
            Msg::ListSite(SiteFormModel { number, kind }) => {
                self.state.sites.insert(
                    number.clone(),
                    Site {
                        number,
                        kind: kind.into(),
                    },
                );
            }
            Msg::DismissErr(ii) => {
                self.state.errors.remove(ii);
            }
            Msg::Nope => {}
        };

        self.storage.store(KEY, Json(&self.state));
        true
    }

    fn view(&self) -> Html {
        let dismiss_err =
            |ii: usize| -> Callback<_> { self.link.callback(move |_| Msg::DismissErr(ii)) };

        let errors = self.state.errors.iter().enumerate();

        // Fixme: How to avoid cloning the data just to pass it in?
        // - SiteValidator doesn't need access to the data,
        //   just an object that can check the data.
        let site_validator = SiteValidator {
            sites: self.state.sites.clone(),
        };

        html! {
            <div>

                <div class="nav">
                    <h1 class="nav-logo">
                        {"Avisha"}
                    </h1>

                    <div class="notifications">
                        <div class="alerts">
                            {for errors.map(|(ii, e)| html! {
                                <div class="alert danger">
                                    <a
                                        class="close"
                                        onclick=dismiss_err(ii)
                                    >
                                        <i class="fa fa-close"/>
                                    </a>
                                    <p>{e}</p>
                                </div>
                            })}
                        </div>
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
                                        {"List Site"}
                                    </h5>
                                    <div class="card-body padded">
                                        <SiteForm::<SiteValidator>
                                            submit=self.link.callback(|v| Msg::ListSite(v))
                                            validator=site_validator
                                        />
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
                        {for self.state.tenants.values().map(|t| html!{
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
                        {for self.state.sites.values().map(|s| html!{
                            <item class="side padded">
                                <p>{format!("Number: {}", &s.number)}</p>
                                <p>{format!("Kind: {}", &s.kind)}</p>
                            </item>
                        })}
                    </list>
                </div>
            </div>
        }
    }

    fn error(&mut self, msg: String) -> bool {
        self.state.errors.push(msg);
        self.storage.store(KEY, Json(&self.state));
        true
    }
}

impl fmt::Display for SiteKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SiteKind::Flat => "Flat",
                SiteKind::Cabin => "Cabin",
                SiteKind::House => "House",
                SiteKind::Other(kind) => kind,
            }
        )
    }
}

impl From<&str> for SiteKind {
    fn from(s: &str) -> Self {
        let s = s.to_lowercase();
        match s.as_str() {
            "cabin" => SiteKind::Cabin,
            "house" => SiteKind::House,
            "flat" => SiteKind::Flat,
            _ => SiteKind::Other(s),
        }
    }
}

impl From<String> for SiteKind {
    fn from(s: String) -> Self {
        let s = s.to_lowercase();
        match s.as_str() {
            "cabin" => SiteKind::Cabin,
            "house" => SiteKind::House,
            "flat" => SiteKind::Flat,
            _ => SiteKind::Other(s),
        }
    }
}

impl From<site_form::Kind> for SiteKind {
    fn from(kind: site_form::Kind) -> Self {
        use site_form::Kind::*;
        match kind {
            Cabin => SiteKind::Cabin,
            House => SiteKind::House,
            Flat => SiteKind::Flat,
            Other(v) => SiteKind::Other(v),
        }
    }
}
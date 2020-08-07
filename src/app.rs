use crate::lease_form::{self, Form as LeaseForm, Model as LeaseFormModel};
use crate::site_form::{self, Form as SiteForm, Model as SiteFormModel};
use crate::tenant_form::{self, Form as TenantForm, Model as TenantFormModel};
use crate::validate::{SiteValidator, TenantValidator, Validate};

use chrono::{Local, NaiveDate as Date};
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use strum_macros::{Display, EnumIter};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};
use yew::services::ConsoleService;

const KEY: &str = "yew.avisha.self";

// Days is a duration in days.
type Days = u32;

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug, Default)]
pub struct Tenant {
    pub name: String, // primary key
    pub contact: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug, Default)]
pub struct Site {
    pub number: String, // primary key
    pub kind: SiteKind,
    pub lease: Option<Lease>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub enum SiteKind {
    Cabin,
    Flat,
    House,
    Other(String),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub struct Lease {
    pub tenant_name: String,
    pub site_number: String,
    pub term: Term,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Hash, Eq, Debug)]
pub struct Term {
    pub start: Date,
    pub duration: Days,
    pub rent: u32,
}

pub struct App {
    state: State,
    storage: StorageService,
    link: ComponentLink<Self>,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub tenants: HashMap<String, Tenant>,
    pub sites: HashMap<String, Site>,
    pub leases: HashSet<Lease>,

    pub errors: Vec<String>,
}

pub enum Msg {
    RegisterTenant(TenantFormModel),
    ListSite(SiteFormModel),
    LeaseSite(LeaseFormModel),
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
                        lease: None,
                    },
                );
            }
            Msg::LeaseSite(LeaseFormModel {
                site,
                tenant,
                start,
                duration,
                rent,
            }) => {
                ConsoleService::log(&format!("attempting to create lease"));
                // TOOD: Handle data parsing.
                // Should this happen at the form level? (I think so).
                self.state.leases.insert(Lease {
                    tenant_name: tenant.name,
                    site_number: site.number,
                    term: Term {
                        start: start.parse().expect("parsing date"),
                        duration: duration.parse().expect("parsing duration"),
                        rent: rent.parse().expect("parsing rent"),
                    },
                });
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
        let tenant_validator = TenantValidator {
            tenants: self.state.tenants.clone(),
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
                                        <TenantForm::<TenantValidator>
                                            submit=self.link.callback(|v| Msg::RegisterTenant(v))
                                            validator=tenant_validator
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
                                <div class="card">
                                    <h5 class="card-header">
                                        {"Enter Lease"}
                                    </h5>
                                    <div class="card-body padded">
                                        <LeaseForm
                                            submit=self.link.callback(|v| Msg::LeaseSite(v))
                                            state=self.state.clone()
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
                        <div class="col">
                            {self.lease_list()}
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
                                // <p>{format!("Lease: {:?}", &s.lease)}</p>
                                // TODO: create lease.
                                // - CreateLease (Site, Tenant, Start, Duration) -> Lease
                                // {if s.lease.is_none() {
                                //     html! {
                                //         <button onclick=self.link.callback(|_| Msg::Nope)>
                                //             {"Lease Me"}
                                //         </button>
                                //     }
                                // } else {
                                //     html!{}
                                // }}
                            </item>
                        })}
                    </list>
                </div>
            </div>
        }
    }

    fn lease_list(&self) -> Html {
        html! {
            <div class="card">
                <h5 class="card-header">
                    {"Leases"}
                </h5>
                <div class="card-body">
                    <list>
                        {for self.state.leases.iter().map(|l| html!{
                            <item class="side padded">
                                <p>{format!("Tenant: {}", &l.tenant_name)}</p>
                                <p>{format!("Site: {}", &l.site_number)}</p>
                                <p>{format!("{:?}", l.term)}</p>
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

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.number)
    }
}

impl fmt::Display for Tenant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Default for SiteKind {
    fn default() -> Self {
        SiteKind::Cabin
    }
}

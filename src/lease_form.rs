use crate::app::State as AppState;
use crate::app::{Site, Tenant};
use crate::form;

use std::collections::HashMap;
use std::default::Default;
use std::mem;
use std::rc::Rc;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use web_sys::{Event, FocusEvent, HtmlElement, HtmlSelectElement};
use yew::prelude::*;
use yew::services::ConsoleService;
use yew_components::Select;
use yewtil::NeqAssign;

pub struct Form {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub model: Model,
    pub errors: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Model {
    pub site: Site,
    pub tenant: Tenant,
    pub start: String,
    pub duration: String,
    pub rent: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub submit: Callback<Model>,
    pub state: AppState,
}

pub enum Msg {
    Edit(Field),
    Submit,
    Nope,
}

pub enum Field {
    Site(Site),
    Tenant(Tenant),
    Start(String),
    Duration(String),
    Rent(String),
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Form {
            props,
            link,
            model: Model::default(),
            errors: HashMap::new(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edit(field) => {
                match field {
                    Field::Site(v) => self.model.site = v,
                    Field::Tenant(v) => self.model.tenant = v,
                    Field::Start(v) => self.model.start = v,
                    Field::Duration(v) => self.model.duration = v,
                    Field::Rent(v) => self.model.rent = v,
                };
                // self.validate_edit();
            }
            Msg::Submit => {
                // self.validate();
                // if self.props.validator.validate(&self.model).is_ok() {
                //     self.props
                //         .submit
                //         .emit(mem::replace(&mut self.model, Model::default()));
                // }
                self.props
                    .submit
                    .emit(mem::replace(&mut self.model, Model::default()));
            }
            Msg::Nope | _ => {}
        };

        true
    }

    fn view(&self) -> Html {
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });

        let get_error = |field_name: &str| -> Option<String> {
            self.errors.get(field_name).map(|s| s.to_string())
        };

        let sites = self
            .props
            .state
            .sites
            .values()
            .map(|s| s.clone())
            .collect::<Vec<Site>>();

        let tenants = self
            .props
            .state
            .tenants
            .values()
            .map(|t| t.clone())
            .collect::<Vec<Tenant>>();

        html! {
            <form
                onsubmit=submit
            >
                <form::Field
                    label={"Site"}
                    error=get_error("site")
                >
                    <Select<Site>
                        on_change=self.link.callback(|v: Site| Msg::Edit(Field::Site(v)))
                        options=sites
                        selected=&self.model.site
                    />
                </form::Field>

                <form::Field
                    label={"Tenant"}
                    error=get_error("tenant")
                >
                    <Select<Tenant>
                        on_change=self.link.callback(|v| Msg::Edit(Field::Tenant(v)))
                        options=tenants
                        selected=&self.model.tenant
                    />
                </form::Field>

                <form::Field
                    label={"Start"}
                    error=get_error("start")
                >
                    <input
                        type="date"
                        oninput=self.link.callback(|v: InputData| {
                            Msg::Edit(Field::Start(v.value))
                        })
                        value=&self.model.start
                    />
                </form::Field>

                <form::Field
                    label={"Duration (days)"}
                    error=get_error("duration")
                >
                    <input
                        type="number"
                        value=&self.model.duration
                        oninput=self.link.callback(|v: InputData| {
                            Msg::Edit(Field::Duration(v.value))
                        })
                    />
                </form::Field>

                <form::Field
                    label={"Rent (fortnightly)"}
                    error=get_error("rent")
                >
                    <input
                        type="number"
                        value=&self.model.rent
                        oninput=self.link.callback(|v: InputData| {
                            Msg::Edit(Field::Rent(v.value))
                        })
                    />
                </form::Field>

                <button
                    type="submit"
                    disabled={self.errors.len() > 0}
                >
                    {"Lease"}
                </button>
            </form>
        }
    }
}

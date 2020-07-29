use crate::form;
use crate::validate::Validate;
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

pub struct Form<V>
where
    V: Validate<Model = Model> + Clone + PartialEq + 'static,
{
    pub link: ComponentLink<Self>,
    pub props: Props<V>,
    pub model: Model,
    pub errors: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Model {
    pub number: String,
    pub kind: Kind,
}

#[derive(Clone, PartialEq, Debug, EnumIter, Display)]
pub enum Kind {
    Cabin,
    House,
    Flat,
    Other(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props<V>
where
    V: Validate + Clone,
{
    pub submit: Callback<Model>,
    pub validator: V,
}

pub enum Msg {
    Edit(Field),
    Submit,
    Nope,
}

// Cleanup: Can we generate fields based on struct definition?
// Use a macro? 
#[derive(Debug)]
pub enum Field {
    Number(String),
    Kind(Kind),
}

impl<V> Component for Form<V>
where
    V: Validate<Model = Model> + Clone + PartialEq + 'static,
{
    type Message = Msg;
    type Properties = Props<V>;

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
                    Field::Number(value) => self.model.number = value,
                    Field::Kind(value) => self.model.kind = value,
                };
                self.validate_edit();
            }
            Msg::Submit => {
                self.validate();
                if self.props.validator.validate(&self.model).is_ok() {
                    ConsoleService::log(&format!("submitting: {:?}", self.model));
                    self.props
                        .submit
                        .emit(mem::replace(&mut self.model, Model::default()));
                }
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
            self.errors
                .get(field_name)
                .map(|s| s.to_string())
        };

        html! {
            <form
                onsubmit=submit
            >

                <form::Field
                    label={"Site Number"}
                    error=get_error("number")
                >
                    <input
                        type="text"
                        placeholder="Site Number"
                        oninput=self.link.callback(|v: InputData| {
                            Msg::Edit(Field::Number(v.value))
                        })
                        value=&self.model.number
                    />
                </form::Field>
                
                <form::Field
                    label={"Kind"}
                    error=get_error("kind")
                >
                    <Select<Kind>
                        on_change=self.link.callback(|v| Msg::Edit(Field::Kind(v)))
                        options=Kind::iter().collect::<Vec<_>>()
                        selected=&self.model.kind
                    />
                    {match &self.model.kind {
                        Kind::Other(kind) => html! {
                            <input
                                type="text"
                                placeholder="House, Cabin, etc"
                                oninput=self.link.callback(|v: InputData| {
                                    Msg::Edit(Field::Kind(Kind::Other(v.value)))
                                })
                                value=&kind
                            />
                        },
                        _ => html! {},
                    }}
                </form::Field>
                
                <button
                    type="submit"
                    disabled={self.errors.len() > 0}
                >
                    {"List"}
                </button>
            </form>
        }
    }
}

impl<V> Form<V>
where
    V: Validate<Model = Model> + Clone + PartialEq + 'static,
{
    fn validate(&mut self) {
        match self.props.validator.validate(&self.model) {
            Err(errors) => self.errors = errors,
            Ok(_) => self.errors.clear(),
        };
    }

    // validate_edit ignores validation for empty fields.
    fn validate_edit(&mut self) {
        self.validate();

        {
            // TODO: make fieldwise error clearing dynamic.
            if self.model.number.is_empty() {
                self.errors.remove("number");
            }
            if let Kind::Other(k) = &self.model.kind {
                if k.is_empty() {
                    self.errors.remove("kind");
                }
            }
        }
    }
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        let s = s.to_lowercase();
        match s.trim() {
            "cabin" => Kind::Cabin,
            "house" => Kind::House,
            "flat" => Kind::Flat,
            s if s.is_empty() => Kind::Cabin,
            _ => Kind::Other(s),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            number: String::new(),
            kind: Kind::Cabin,
        }
    }
}
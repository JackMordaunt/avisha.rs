use crate::validate::Validate;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use web_sys::{Event, FocusEvent, HtmlSelectElement, HtmlElement};
use yew::prelude::*;
use yew::services::ConsoleService;
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

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Model {
    pub number: String,
    pub kind: String,
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
    Edit(Field, String),
    Submit,
    Nope,
}

// Cleanup: Can we generate fields based on struct definition? 
pub enum Field {
    Number,
    Kind,
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
            model: Model {
                kind: Kind::Cabin.to_string(),
                ..Model::default()
            },
            errors: HashMap::new(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edit(field, value) => {
                match field {
                    Field::Number => self.model.number = value,
                    Field::Kind => self.model.kind = value,
                };
                self.validate_edit();
            }
            Msg::Submit => {
                self.validate();
                if self.props.validator.validate(&self.model).is_ok() {
                    self.props
                        .submit
                        .emit(mem::replace(&mut self.model, Model::default()));
                }
                return true;
            }
            Msg::Nope | _ => {
                return false;
            }
        };

        true
    }

    fn view(&self) -> Html {
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });

        let display_custom_kind_input = match Kind::from(self.model.kind.as_str()) {
            Kind::Other(_) => "display: block",
            _ => "display: none",
        };

        let check_error = |field_name: &str| -> &str {
            if self.errors.contains_key(field_name) {
                "error"
            } else {
                ""
            }
        };

        let get_error = |field_name: &str| -> &str {
            self.errors.get(field_name).map(|s| s.as_str()).unwrap_or("")
        };

        html! {
            <form
                onsubmit=submit
            >
                <div class=check_error("number")>
                    <label>{"Number"}</label>
                    <input
                        type="text"
                        placeholder="Site Number"
                        oninput=self.link.callback(|v: InputData| Msg::Edit(Field::Number, v.value))
                        value=&self.model.number
                    />
                    <div class="alert danger">
                        {get_error("number")}
                    </div>
                </div>
                <div>
                    <label>{"Kind"}</label>
                    <select
                        onchange=self.link.callback(|v: ChangeData| if let ChangeData::Select(v) = v {
                            Msg::Edit(Field::Kind, v.value())
                        } else {
                            Msg::Nope
                        })
                    >
                        {for Kind::iter().map(|k| {
                            if self.model.kind == k.to_string() {
                                html! {
                                    <option value={&k} selected=true>
                                        {&k}
                                    </option>
                                }
                            } else {
                                html! {
                                    <option value={&k}>
                                        {&k}
                                    </option>
                                }
                            }
                        })}
                    </select>
                    <input
                        type="text"
                        style=display_custom_kind_input
                        placeholder="House, Cabin, etc"
                        oninput=self.link.callback(|v: InputData| Msg::Edit(Field::Kind, v.value))
                        value=&self.model.kind
                    />
                </div>
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
    V: Validate<Model=Model> + Clone + PartialEq + 'static
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
            if self.model.kind.is_empty() {
                self.errors.remove("kind");
            }
        }
    }
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        let s = s.to_lowercase();
        match s.as_str() {
            "cabin" => Kind::Cabin,
            "house" => Kind::House,
            "flat" => Kind::Flat,
            _ => Kind::Other(s),
        }
    }
}

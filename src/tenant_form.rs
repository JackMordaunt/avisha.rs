use crate::form;
use crate::validate::Validate;
use std::mem;
use web_sys::{Event, FocusEvent};
use yew::prelude::*;
use yewtil::NeqAssign;
use std::collections::HashMap;

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
    pub name: String,
    pub contact: String,
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

pub enum Field {
    Name(String),
    Contact(String),
}

impl<V> Component for Form<V>
where 
    V: Validate<Model = Model> + Clone + PartialEq + 'static,
{
    type Message = Msg;
    type Properties = Props<V>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Form { props, link, model: Model::default(), errors: HashMap::new() }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edit(field) => {
                match field {
                    Field::Name(value) => self.model.name = value,
                    Field::Contact(value) => self.model.contact = value,
                };
                self.validate_edit();
            }
            Msg::Nope => {
                return false;
            }
            Msg::Submit => {
                self.validate();
                if self.props.validator.validate(&self.model).is_ok() {
                    self.props
                        .submit
                        .emit(mem::replace(&mut self.model, Model::default()));
                }
            }
        }
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
            <form onsubmit=submit>
                <form::Field
                    label={"Name"}
                    error=get_error("name")
                >
                    <input
                        type="text"
                        placeholder="Tenant Name"
                        oninput=self.link.callback(|v: InputData| Msg::Edit(Field::Name(v.value)))
                        value=&self.model.name
                    />
                </form::Field>

                <form::Field
                    label={"Contact"}
                    error=get_error("contact")
                >
                    <input
                        type="text"
                        placeholder="Email or Phone"
                        oninput=self.link.callback(|v: InputData| Msg::Edit(Field::Contact(v.value)))
                        value=&self.model.contact
                    />
                </form::Field>
                
                <button type="submit">
                    {"Register"}
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
            if self.model.name.is_empty() {
                self.errors.remove("name");
            }
        }
    }
}

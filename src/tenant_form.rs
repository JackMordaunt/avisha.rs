use std::mem;
use web_sys::{Event, FocusEvent};
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Form {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub model: Model,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Model {
    pub name: String,
    pub contact: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub submit: Callback<Model>,
}

pub enum Msg {
    Name(InputData),
    Contact(InputData),
    Submit,
    Nope,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let model = Model::default();
        Form { props, link, model }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Name(name) => {
                self.model.name = name.value;
            }
            Msg::Contact(contact) => {
                self.model.contact = contact.value;
            }
            Msg::Nope => {
                return false;
            }
            Msg::Submit => {
                self.props
                    .submit
                    .emit(mem::replace(&mut self.model, Model::default()));
            }
        }
        true
    }

    fn view(&self) -> Html {
        let submit = self.link.callback(|e: FocusEvent| {
            e.prevent_default();
            Msg::Submit
        });

        html! {
            <form onsubmit=submit>
                <div>
                    <label>{"Name"}</label>
                    <input
                        type="text"
                        placeholder="Tenant Name"
                        oninput=self.link.callback(|v| Msg::Name(v))
                        value=&self.model.name
                    />
                </div>
                <div>
                    <label>{"Contact"}</label>
                    <input
                        type="text"
                        placeholder="Email or Phone"
                        oninput=self.link.callback(|v| Msg::Contact(v))
                        value=&self.model.contact
                    />
                </div>
                <button type="submit">
                    {"Register"}
                </button>
            </form>
        }
    }
}

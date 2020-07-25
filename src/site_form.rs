use std::mem;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use web_sys::{Event, FocusEvent, HtmlSelectElement};
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Form {
    pub link: ComponentLink<Self>,
    pub props: Props,
    pub model: Model,
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
pub struct Props {
    pub submit: Callback<Model>,
}

pub enum Msg {
    Number(String),
    Kind(String),
    Submit,
    Nope,
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let model = Model {
            kind: Kind::Cabin.to_string(),
            ..Model::default()
        };
        Form { props, link, model }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Number(number) => {
                self.model.number = number;
            }
            Msg::Kind(kind) => {
                self.model.kind = kind;
            }
            Msg::Submit => {
                self.props
                    .submit
                    .emit(mem::replace(&mut self.model, Model::default()));
            }
            Msg::Nope | _ => {
                return false;
            }
        }
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

        html! {
            <form onsubmit=submit>
                <div>
                    <label>{"Number"}</label>
                    <input
                        type="text"
                        placeholder="Site Number"
                        oninput=self.link.callback(|v: InputData| Msg::Number(v.value))
                        value=&self.model.number
                    />
                </div>
                <div>
                    <label>{"Kind"}</label>
                    <select
                        onchange=self.link.callback(|v: ChangeData| if let ChangeData::Select(v) = v {
                            Msg::Kind(v.value())
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
                        oninput=self.link.callback(|v: InputData| Msg::Kind(v.value))
                        value=&self.model.kind
                    />
                </div>
                <button type="submit">
                    {"List"}
                </button>
            </form>
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

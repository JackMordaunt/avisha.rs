use crate::validate::Validate;
use std::collections::HashMap;
use std::mem;
use yew::Renderable;
use yew::{prelude::*, Component, ComponentLink};
use yewtil::NeqAssign;

/// Field wraps a form input with a label and error display.
pub struct Field {
    pub props: FieldProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct FieldProps {
    pub label: String,
    pub error: Option<String>,
    pub children: Children,
}

impl Component for Field {
    type Message = ();
    type Properties = FieldProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        match &self.props.error {
            None => html! {
                <div>
                    <label>
                        { &self.props.label }
                    </label>
                    { self.props.children.clone() }
                </div>
            },
            Some(err) => html! {
                <div class="error">
                    <label>
                        { &self.props.label }
                    </label>
                    { self.props.children.clone() }
                    <div class="error-message">
                        {err}
                    </div>
                </div>
            },
        }
    }
}

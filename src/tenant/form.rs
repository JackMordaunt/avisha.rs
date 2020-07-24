// use serde_derive::{Deserialize, Serialize};
// use std::collections::HashSet;
// use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

pub struct TenantForm {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub enum Msg {}

impl Component for TenantForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TenantForm { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <form>
            </form>
        }
    }
}

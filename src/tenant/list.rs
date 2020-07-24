use crate::tenant::Tenant;
use std::collections::HashSet;
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::NeqAssign;

pub struct TenantList {
    pub link: ComponentLink<Self>,
    pub props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub tenants: Rc<HashSet<Tenant>>,
}

pub enum Msg {}

impl Component for TenantList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TenantList { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <card>
                <header>
                    <h2>{"Tenants"}</h2>
                </header>
                <section>
                    {for self.props.tenants.iter().map(|t| html!{
                        <item>{&t.name}</item>
                    })}
                </section>
            </card>
        }
    }
}

#![allow(unused)]

use yew::prelude::*;

pub struct App {
    pub link: ComponentLink<Self>,
    pub debug: bool,
}

pub enum Msg {
    ToggleDebug,
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, debug: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDebug => {
                self.debug = !self.debug;
            }
            Msg::Nope => {}
        };
        true
    }

    fn view(&self) -> Html {
        let debug = if self.debug { "debug" } else { "" };
        let toggle_debug = self.link.callback(|_| Msg::ToggleDebug);

        html! {
            <grid class=debug>
                <row>
                    <nav>
                        <brand>
                            <h1 class="logo">
                                {"Avisha"}
                            </h1>
                        </brand>
                        <div>
                            <button class="small" onclick=toggle_debug>
                                {"Debug"}
                            </button>
                        </div>
                    </nav>
                </row>
                <row>
                    <col></col>
                </row>
            </grid>
        }
    }
}

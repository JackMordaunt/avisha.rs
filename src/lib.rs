#![allow(unused)]
#![recursion_limit = "1024"]

mod app;
mod form;
mod lease_form;
mod site_form;
mod tenant_form;
mod validate;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}

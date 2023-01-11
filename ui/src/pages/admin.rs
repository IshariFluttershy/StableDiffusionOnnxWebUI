use std::fs;

use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use reqwasm::http::*;
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen::*;
use log::info;

use crate::components;

use components::range::Range;

pub enum Msg {
    OnChange(String, String),
    Clicked,
    ImageGenerated,
}

pub struct Admin {
    pass: String,
}

impl Component for Admin {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Admin {
            pass: String::from("lel"),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <form action="/adminconnect" method="post" class="form-example">
                    <div class="form-example">
                        <label for="pass">{"Password"}</label>
                        <input type="text" name="pass" id="pass" required=true/>
                    </div>
                    <div class="form-example">
                        <input type="submit" value="Login"/>
                    </div>
                </form>
            </div>
        }
    }
}
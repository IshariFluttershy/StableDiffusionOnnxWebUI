use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use reqwasm::http::*;
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen::*;

use crate::components;

use components::range::Range;

pub enum Msg {
    OnChange(String),
}

pub struct Home {
    prompt: String,
    neg_prompt: String,
    steps: u8,
    guidance: f32,
    width: u16,
    height: u16,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        wasm_logger::init(wasm_logger::Config::default());

        Home {
            prompt: String::from("test creation home"),
            neg_prompt: String::from(""),
            steps: 0,
            guidance: 0.,
            width: 0,
            height: 0,
        }
    }

    //prompt=hahaha&neg_prompt=&steps=15&guidance=7.5&width=512&height=512
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
                
        match msg {
            OnChange(input) => {
                self.prompt = input.clone();
                self.neg_prompt = input.clone();
                self.steps = input.parse::<u8>().unwrap();
                self.guidance = input.parse::<f32>().unwrap();
                self.width = input.parse::<u16>().unwrap();
                self.height = input.parse::<u16>().unwrap();

            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        
        
        spawn_local(async {

            /*let task = Task {
                prompt:"1girl",
                neg_prompt: "1boy",
                steps: 7,
                guidance: 7.5,
                width: 512,
                height: 512,
            };*/

            let resp = Request::post("/command")
                .header("Content-Type", "application/x-www-form-urlencoded")
                //.body("prompt=1girl&neg_prompt=1boy&steps=7&guidance=7.5&width=512&height=512")
                .body(wasm_bindgen::JsValue::from_str("prompt=bonjoure&neg_prompt=aurevoir&steps=7&guidance=7.5&width=512&height=512"))
                //.body(to_value(&task).unwrap())
                .send()
                .await
                .unwrap();
            
            assert_eq!(resp.status(), 200);
        });


        

        let link = ctx.link();

        let on_cautious_change = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::OnChange(input.value()))
        });

        html! {
            <div class="d-flex justify-content-center m-5">
                /*<h1>{"Building a Website in Rust"}</h1>*/
                <br/>
                <br/>
                //<form action="/command" method="post">
                    <div class="input-group">
                        <span class="input-group-text">{"prompt"}</span>
                        <textarea class="form-control" aria-label="prompt" name="prompt"></textarea>
                    </div>
                    <div class="input-group">
                        <span class="input-group-text">{"negative prompt"}</span>
                        <textarea class="form-control" aria-label="negative prompt" name="neg_prompt" ></textarea>
                    </div>
                    <div>
                        <Range value=15. text={"steps"} name={"steps"} min=1. max=150. step=1. on_change={on_cautious_change.clone()}></Range>
                        <Range value=7.5 text={"guidance"} name={"guidance"} min=1. max=25. step=0.1 on_change={on_cautious_change.clone()}></Range>
                        <Range value=512. text={"width"} name={"width"} min=256. max=1024. step=64. on_change={on_cautious_change.clone()}></Range>
                        <Range value=512. text={"height"} name={"height"} min=256. max=1024. step=64. on_change={on_cautious_change}></Range>
                    </div>
                    <div class="button">
                        <button >{"Envoyer le message"}</button>
                    </div>
                //</form>
            </div>
        }
    }
}
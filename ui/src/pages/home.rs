use std::fs;

use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
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

pub struct Home {
    prompt: String,
    neg_prompt: String,
    steps: u8,
    guidance: f32,
    width: u16,
    height: u16,
    rerender_image: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub render_switch: bool,
}

#[function_component(App)]
fn app(props: &Props) -> Html {
    let result = use_state(|| String::from(""));
    {
        let result = result.clone();
        use_effect_with_deps(move |_| {
            let result = result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: String = Request::get("/lastimage")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
                result.set(fetched_data);
        });
        || ()
    }, props.render_switch);
    }

    
    /*let result = use_state(|| props.result.clone());
    info!("result before = {}", (*result));
    {
        let result = result.clone();
        //let result2 = result.clone();
        use_effect(move || {
            let result = result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: String = Request::get("/lastimage")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
                result.set(fetched_data);
            });
            || ()
            /*{info!("result2 before = {}", (*result2));
            result2.set("fetched_data".to_owned());
            info!("result2 after = {}", (*result2))}*/
        });
    }*/

    info!("result after = {}", (*result));
    html! {
        <div>
            <img src={format!("data\\output\\{:0>6}-00.png", (*result))} alt={"Generated Image"}/>
        </div>
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        wasm_logger::init(wasm_logger::Config::default());

        Home {
            prompt: String::from("test creation home"),
            neg_prompt: String::from(""),
            steps: 15,
            guidance: 7.5,
            width: 512,
            height: 512,
            rerender_image: false,
        }
    }
    

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
                
        match msg {
            OnChange(name, value) => {
                info!("name:{} ---- value:{}", name, value);

                if name == "steps" {
                    self.steps = value.parse::<u8>().unwrap();
                } else if name == "guidance" {
                    self.guidance = value.parse::<f32>().unwrap();
                } else if name == "width" {
                    self.width = value.parse::<u16>().unwrap();
                } else if name == "height" {
                    self.height = value.parse::<u16>().unwrap();
                } else if name == "prompt" {
                    self.prompt = value;
                } else if name == "neg_prompt" {
                    self.neg_prompt = value;
                }
            }
            Clicked => {
                let prompt = self.prompt.clone();
                let neg_prompt = self.neg_prompt.clone();
                let steps = self.steps;
                let guidance = self.guidance;
                let width = self.width;
                let height = self.height;

                let link = ctx.link().clone();
                spawn_local(async move {
                    let resp = Request::post("/command")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(wasm_bindgen::JsValue::from_str(
                            &format!("prompt={}&neg_prompt={}&steps={}&guidance={}&width={}&height={}",
                            prompt.clone(),
                            neg_prompt.clone(),
                            steps,
                            guidance,
                            width,
                            height
                            )))
                        .send()
                        .await
                        .unwrap();
                    
                    assert_eq!(resp.status(), 200);
                    let cb = link.callback(|i:u8| Msg::ImageGenerated);
                    cb.emit(0);
                });
            }
            ImageGenerated => {
                self.rerender_image = !self.rerender_image;
                info!("c'est passé par image generated");
                return true;
            }
            /*GetImageNumber => {
                let request = Request::get("/lastimage");
                let callback =
                    ctx.link()
                        .callback(|response: Response| {
                            let data = response.text();
                            Msg::ImageNumberReceived(data)
                        });
            }
            ImageNumberReceived => {

            }*/


        }
        info!("update renvoie true");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Clicked);

        let link = ctx.link();

        let on_cautious_change = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::OnChange(input.name(), input.value()))
        });

        let on_cautious_input = link.batch_callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let event_target = event.target().unwrap();
            let target: Option<HtmlInputElement> = event_target.dyn_into().ok();
            target.map(|input| Msg::OnChange(input.name(), input.value()))
        });

        info!("ca passe dans view");


        html! {
            <div>
                <div class="col-6 col-s-6 menu">
                    <div>
                        <div>
                            <span>{"Prompt : "}</span> 
                            <input class="input-group-text test" type="textarea" name="prompt" oninput={on_cautious_input.clone()} onchange={on_cautious_change.clone()}/>
                        </div>
                        <br/>
                        <div>
                            <span >{"Negative prompt : "}</span> 
                            <input class="input-group-text test" type="textarea" name="neg_prompt" oninput={on_cautious_input} onchange={on_cautious_change.clone()}/>
                        </div>
                    </div>
                    <div>
                        <br/>
                        <div>
                            <Range value=15. text={"steps"} name={"steps"} min=1. max=150. step=1. on_change={on_cautious_change.clone()}></Range> <br/>
                            <Range value=7.5 text={"guidance"} name={"guidance"} min=1. max=25. step=0.1 on_change={on_cautious_change.clone()}></Range> <br/>
                            <Range value=512. text={"width"} name={"width"} min=256. max=1024. step=64. on_change={on_cautious_change.clone()}></Range> <br/>
                            <Range value=512. text={"height"} name={"height"} min=256. max=1024. step=64. on_change={on_cautious_change}></Range> <br/>
                        </div>
                        <br/>
                        <div class="button">
                            <button {onclick}>{"Envoyer le message"}</button>
                        </div>
                    </div>
                </div>
                <div class="col-6 col-s-6 menu">
                    <App render_switch={self.rerender_image}/>
                </div>
            </div>
        }
    }
}
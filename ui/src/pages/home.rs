use std::{fs, rc::Rc};

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
    IsConnected(String),
}

pub struct Home {
    prompt: String,
    neg_prompt: String,
    steps: u8,
    guidance: f32,
    width: u16,
    height: u16,
    iterations: u16,
    model: String,
    scheduler: String,
    rerender_image: bool,
    connected: bool,
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

    html! {
        <div>
            <img src={format!("data\\output\\{:0>6}-00.png", (*result))} alt={"Generated Image"}/>
        </div>
    }
}

impl Home {
    fn connectedModels(&self, ctx: &Context<Self>, cb: Callback<Event>) -> Html {
        if self.connected == true {
            html! {
                <>
                    <select name="model" id="model-select" onchange={cb}>
                        <option value="stable_diffusion_onnx" selected={true}>{"Stable Diffusion"}</option>
                        <option value="waifu-diffusion-diffusers-onnx-v1-3">{"Waifu Diffusion"}</option>
                        <option value="hassanblend_onnx">{"Hassanblend"}</option>
                    </select>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        }
    }

    fn connectedIterations(&self, ctx: &Context<Self>, cb: Callback<Event>) -> Html {
        if self.connected == true {
            html! {
                <>
                    <Range value=1. text={"iterations"} name={"iterations"} min=1. max=1000. step=1. on_change={cb}></Range> <br/>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        }
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        wasm_logger::init(wasm_logger::Config::default());

        let link = ctx.link().clone();

        wasm_bindgen_futures::spawn_local(async move {
            let fetched_data: String = Request::get("/connected")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
            
            let cb = link.callback(|data:String| Msg::IsConnected(data.clone()));
            cb.emit(fetched_data);
        });

        Home {
            prompt: String::from("test creation home"),
            neg_prompt: String::from(""),
            steps: 15,
            guidance: 7.5,
            width: 512,
            height: 512,
            iterations: 1,
            model: String::from("stable_diffusion_onnx"),
            scheduler: String::from("eulera"),
            rerender_image: false,
            connected: false,
        }


    }
    

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
        info!("Ca passe dans update");

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
                } else if name == "iterations" {
                    self.iterations = value.parse::<u16>().unwrap();
                } else if name == "model" {
                    self.model = value;
                } else if name == "scheduler" {
                    self.scheduler = value;
                }
            }
            Clicked => {
                let prompt = self.prompt.clone();
                let neg_prompt = self.neg_prompt.clone();
                let model = self.model.clone();
                let scheduler = self.scheduler.clone();
                let steps = self.steps;
                let guidance = self.guidance;
                let width = self.width;
                let height = self.height;
                let iterations = self.iterations;


                let link = ctx.link().clone();
                spawn_local(async move {
                    let resp = Request::post("/command")
                        .header("Content-Type", "application/x-www-form-urlencoded")
                        .body(wasm_bindgen::JsValue::from_str(
                            &format!("prompt={}&neg_prompt={}&model={}&scheduler={}&steps={}&guidance={}&width={}&height={}&iterations={}",
                            prompt.clone(),
                            neg_prompt.clone(),
                            model.clone(),
                            scheduler.clone(),
                            steps,
                            guidance,
                            width,
                            height,
                            iterations
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
                return true;
            }
            IsConnected(response) => {
                info!("response == {}", response);

                if response == "true" {
                    self.connected = true;
                }
            }
        }
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

        let on_cautious_change_select = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            input.map(|input| Msg::OnChange(input.name(), input.value()))
        });

        let on_cautious_change_textarea = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            input.map(|input| Msg::OnChange(input.name(), input.value()))
        });

        /*let on_cautious_input = link.batch_callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let event_target = event.target().unwrap();
            let target: Option<HtmlInputElement> = event_target.dyn_into().ok();
            target.map(|input| Msg::OnChange(input.name(), input.value()))
        });*/

        html! {
            <div>
                <div class="col-6 col-s-6 menu">
                    {self.connectedModels(ctx, on_cautious_change_select.clone())}
                    <select name="scheduler" id="scheduler-select" onchange={on_cautious_change_select.clone()}>
                        <option value="pndm">{"PNDM"}</option>
                        <option value="lms">{"LMS"}</option>
                        <option value="ddim">{"DDIM"}</option>
                        <option value="ddpm">{"DDPM"}</option>
                        <option value="euler">{"Euler"}</option>
                        <option value="eulera" selected={true}>{"EulerA"}</option>
                        <option value="dpms">{"DPMS"}</option>
                    </select>
                    <div>
                        <div>
                            <span>{"Prompt : "}</span> 
                            <textarea name="prompt" onchange={on_cautious_change_textarea.clone()}/>
                        </div>
                        <br/>
                        <div>
                            <span >{"Negative prompt : "}</span> 
                            <textarea name="neg_prompt" onchange={on_cautious_change_textarea}/>
                        </div>
                    </div>
                    <div>
                        <br/>
                        <div>
                            <Range value=15. text={"steps"} name={"steps"} min=1. max=100. step=1. on_change={on_cautious_change.clone()}></Range> <br/>
                            <Range value=7.5 text={"guidance"} name={"guidance"} min=1. max=25. step=0.1 on_change={on_cautious_change.clone()}></Range> <br/>
                            <Range value=512. text={"width"} name={"width"} min=256. max=1024. step=64. on_change={on_cautious_change.clone()}></Range> <br/>
                            <Range value=512. text={"height"} name={"height"} min=256. max=1024. step=64. on_change={on_cautious_change.clone()}></Range> <br/>
                            {self.connectedIterations(ctx, on_cautious_change.clone())}
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
use yew::prelude::*;
use reqwasm::http::*;
use wasm_bindgen_futures::spawn_local;

use crate::components;

use components::range::Range;


pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        wasm_logger::init(wasm_logger::Config::default());

        Home {}
    }

    //prompt=hahaha&neg_prompt=&steps=15&guidance=7.5&width=512&height=512
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        
        
        spawn_local(async {
            let resp = Request::post("/command")
                .body("prompt=bonjoure&neg_prompt=aurevoir&steps=7&guidance=7.5&width=512&height=512")
                .send()
                .await
                .unwrap();
            
            assert_eq!(resp.status(), 200);
        });


        


        html! {
            <div class="d-flex justify-content-center m-5">
                /*<h1>{"Building a Website in Rust"}</h1>*/
                <br/>
                <br/>
                <form action="/command" method="post">
                    <div class="input-group">
                        <span class="input-group-text">{"prompt"}</span>
                        <textarea class="form-control" aria-label="prompt" name="prompt"></textarea>
                    </div>
                    <div class="input-group">
                        <span class="input-group-text">{"negative prompt"}</span>
                        <textarea class="form-control" aria-label="negative prompt" name="neg_prompt"></textarea>
                    </div>
                    <div>
                        <Range value=15. text={"steps"} name={"steps"} min=1. max=150. step=1.></Range>
                        <Range value=7.5 text={"guidance"} name={"guidance"} min=1. max=25. step=0.1></Range>
                        <Range value=512. text={"width"} name={"width"} min=256. max=1024. step=64.></Range>
                        <Range value=512. text={"height"} name={"height"} min=256. max=1024. step=64.></Range>
                    </div>
                    <div class="button">
                        <button type="submit">{"Envoyer le message"}</button>
                    </div>
                </form>
            </div>
        }
    }
}
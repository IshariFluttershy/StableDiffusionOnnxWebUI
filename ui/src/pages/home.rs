use yew::prelude::*;

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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="d-flex justify-content-center m-5">
                <h1>{"Building a Website in Rust"}</h1>
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
                        <Range value=7.5 text={"guidance"} name={"guidance"} min=1. max=50. step=0.1></Range>
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
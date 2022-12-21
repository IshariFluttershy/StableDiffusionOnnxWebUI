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
                        <Range value=15 text={"steps"} name={"steps"}></Range>
                        <Range value=7 text={"guidance"} name={"guidance"}></Range>
                        <Range value=512 text={"width"} name={"width"}></Range>
                        <Range value=512 text={"height"} name={"height"}></Range>
                    </div>
                    <div class="button">
                        <button type="submit">{"Envoyer le message"}</button>
                    </div>
                </form>
            </div>
        }
    }
}
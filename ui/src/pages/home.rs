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
                        <span class="input-group-text">{"With textarea"}</span>
                        <textarea class="form-control" aria-label="With textarea" name="prompt"></textarea>
                    </div>
                    <div>
                        <Range value=50 text={"steps"}></Range>
                        <Range value=50 text={"steps"}></Range>
                        <Range value=50 text={"steps"}></Range>

                    </div>
                    <div class="button">
                        <button type="submit">{"Envoyer le message"}</button>
                    </div>
                </form>
            </div>
        }
    }
}
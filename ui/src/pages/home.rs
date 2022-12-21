use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
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
                        <label for="customRange3" class="form-label">{"Example range"}</label>
                        <input type="range" class="form-range" min="1" max="40" id="customRange3" name="steps"/>
                    </div>
                    <div class="button">
                        <button type="submit">{"Envoyer le message"}</button>
                    </div>
                </form>
            </div>
        }
    }
}
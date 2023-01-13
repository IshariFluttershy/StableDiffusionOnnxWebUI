use yew::prelude::*;

pub struct Admin;

impl Component for Admin {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Admin
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
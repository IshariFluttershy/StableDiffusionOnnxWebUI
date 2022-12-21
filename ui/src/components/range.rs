use yew::{prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use log::info;


#[derive(Properties, PartialEq)]
pub struct Range {
    pub value: u32,
    pub text: String,
}

pub enum Msg {
    OnChange(String),
}

impl Component for Range {
    type Message = Msg;
    type Properties = Range;

    fn create(ctx: &Context<Self>) -> Self {
        Range {
            value: ctx.props().value,
            text: ctx.props().text.clone(),
        }  
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
                
        match msg {
            OnChange(input) => {
                self.value = input.parse::<u32>().unwrap();
            }
        }
        true
      }


      
      fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        /*let on_cautious_change = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::OnChange(input.value()))
        });*/

        let on_cautious_input = link.batch_callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap();
            let event_target = event.target().unwrap();
            let target: Option<HtmlInputElement> = event_target.dyn_into().ok();
            target.map(|input| Msg::OnChange(input.value()))
        });

        html! {
            <div>
                <label for="customRange" class="form-label">{format!("{} : {}", self.text, self.value)}</label>
                <br/>
                <input type="range" min="1" max="150" step="1" class="range" id="myRange"
                    value={self.value.to_string()}
                    //onchange={on_cautious_change}
                    oninput={on_cautious_input}
                />
                
            </div>
        }
    }
}
use yew::{prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use log::info;


pub struct Range {
    pub value: f32,
}

#[derive(Properties, PartialEq)]
pub struct RangeProps {
    pub value: f32,
    pub text: String,
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub on_change: Callback<Event>,
}

pub enum Msg {
    OnChange(String),
}

impl Component for Range {
    type Message = Msg;
    type Properties = RangeProps;

    fn create(ctx: &Context<Self>) -> Self {
        Range {
            value: ctx.props().value,
        }  
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use Msg::*;
                
        match msg {
            OnChange(input) => {
                self.value = input.parse::<f32>().unwrap();
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
                <label for="customRange" class="form-label">{format!("{} : {}", ctx.props().text, self.value)}</label>
                <br/>
                <input type="range" min={ctx.props().min.to_string()} max={ctx.props().max.to_string()} step={ctx.props().step.to_string()} class="form-range" id="myRange" name={ctx.props().text.clone()}
                    value={self.value.to_string()}
                    onchange={ctx.props().on_change.clone()}
                    oninput={on_cautious_input}
                />
                
            </div>
        }
    }
}
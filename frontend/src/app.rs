use crate::name;
use yew::prelude::*;

pub struct Orca;

impl yew::Component for Orca {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <h1 class={ classes!("text-3xl", "font-bold", "underline") }>{ "Hello World" }</h1>
                <name::Name />
            </>
        }
    }
}

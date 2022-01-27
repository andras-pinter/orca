use reqwasm::http::Request;
use yew::prelude::*;

pub struct Name {
    names: Vec<String>,
}

pub enum Message {
    Add(String),
    Idle,
}

impl yew::Component for Name {
    type Message = Message;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { names: Vec::new() }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Add(name) => self.names.push(name),
            Message::Idle => return false,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress: Callback<yew::KeyboardEvent> = ctx.link().callback_future(|e: KeyboardEvent| async move {
            if e.key() == "Enter" {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                let name = common::IName { name: value };
                let resp = Request::post("/api/name")
                    .body(name)
                    .send()
                    .await
                    .unwrap()
                    .json::<common::IName>()
                    .await
                    .unwrap();

                Message::Add(resp.name)
            } else {
                Message::Idle
            }
        });

        html! {
            <>
                <h1 class={ classes!("bg-blue-100") }>{ "Hello there!" }</h1>
                <input {onkeypress} placeholder="Who is it?" />
                <hr/>
                {for self.names.iter().map(|name| html! { <h3>{name}</h3> })}
            </>
        }
    }
}

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::{html, html::Scope, Component, Context, Html, KeyboardEvent, TargetCast};

mod footer;
mod header;
mod todo;

use footer::Footer;
use header::Header;
use todo::Todo;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header></Header>
                <br/>
                <br/>
                <div class="container text-align-center" style="max-width: 600px">
                    <Todo></Todo>
                    <br/>
                    <br/>
                    <Footer></Footer>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}

use crate::{html, Component, Context, Html};

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Header
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar navbar-expand-sm bg-info navbar-dark justify-content-center">
                <a class="navbar-brand" href="/#"><span class="font-weight-bold">{ "Todo App" }</span></a>
            </nav>
        }
    }
}

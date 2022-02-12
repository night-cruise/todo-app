use crate::{html, Component, Context, Html};

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer>
                <p class="text-center">
                    <small>{ "© 2021 "}
                        <a href="https://github.com/night-cruise" title="Written by Night Cruising">{ "Night Cruising" }</a> { "-" }
                        <a href="https://github.com/night-cruise/todo-app" title="Fork me on GitHub">{ "GitHub" }</a>
                    </small>
                </p>
            </footer>
        }
    }
}

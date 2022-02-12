use crate::{
    html, Component, Context, Deserialize, Html, HtmlInputElement, KeyboardEvent, LocalStorage,
    Scope, Serialize, Storage, TargetCast,
};

const KEY: &str = "todo-app";

#[derive(PartialEq, Serialize, Deserialize)]
enum EntryState {
    Active,
    Completed,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    content: String,
    state: EntryState,
}

pub struct Todo {
    entries: Vec<Entry>,
}

pub enum Msg {
    AddOne(String),
    DeleteOne(usize),
    ToggleOne(usize),
    Clear,
}

impl Component for Todo {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let entries = LocalStorage::get(KEY).unwrap_or_else(|_| Vec::new());
        Todo { entries }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne(content) => {
                if !content.is_empty() {
                    self.entries.push(Entry {
                        content: content.trim().to_string(),
                        state: EntryState::Active,
                    })
                }
            }
            Msg::DeleteOne(idx) => {
                self.entries.remove(idx);
            }
            Msg::ToggleOne(idx) => match self.entries[idx].state {
                EntryState::Active => self.entries[idx].state = EntryState::Completed,
                EntryState::Completed => self.entries[idx].state = EntryState::Active,
            },
            Msg::Clear => {
                self.entries = self
                    .entries
                    .drain(..)
                    .filter(|val| val.state != EntryState::Completed)
                    .collect();
            }
        }
        LocalStorage::set(KEY, &self.entries).expect("Local storage failed");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <>
                { self.view_input(link) }
                <br/>
                <ul class="list-group">
                    { for self.entries.iter().enumerate().map(|(idx, val)| self.view_entry((idx, val), link)) }
                </ul>
            </>
        }
    }
}

impl Todo {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                let input = event.target_unchecked_into::<HtmlInputElement>();
                let value = input.value();
                input.set_value("");
                Some(Msg::AddOne(value))
            } else {
                None
            }
        });

        html! {
            <form>
                <div class="input-group">
                      <input type="text" class="form-control border border-info" placeholder="What needs to be done?" { onkeypress } />
                    <div class="input-group-append">
                          <button class="btn btn-info" onclick={ link.callback(move |_| Msg::Clear) }>{ "Clear" }</button>
                    </div>
                </div>
              </form>
        }
    }

    fn view_entry(&self, (idx, val): (usize, &Entry), link: &Scope<Self>) -> Html {
        let (checked, completed) = match val.state {
            EntryState::Completed => (true, "text-decoration:line-through"),
            EntryState::Active => (false, ""),
        };

        html! {
            <li class="list-group-item" style="margin:3px">
                <input type="checkbox" style="zoom:120%;vertical-align:middle;" checked={ checked } onclick={ link.callback(move |_| Msg::ToggleOne(idx)) } />
                {" "}
                <span style={ completed }>
                    { &val.content }
                </span>
                <button type="button" class="close" data-dismiss="alert" onclick={ link.callback(move |_| Msg::DeleteOne(idx)) }>{ "×" }</button>
            </li>
        }
    }
}

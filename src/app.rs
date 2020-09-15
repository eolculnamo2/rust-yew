use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

pub struct Model {
    link: ComponentLink<Self>,
    state: State,
}

impl Model {
    fn view_entry(&self, (_, item): (usize, &Item)) -> Html {
        html! {
            <div>{item.name.clone()}</div>
        }
    }
}

struct Item {
    name: String,
}

struct State {
    items: Vec<Item>,
    newItem: String,
}

pub enum Msg {
    Add,
    Update(String),
    None,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let init_state = State {
            items: vec![],
            newItem: String::from(""),
        };

        Self {
            link,
            state: init_state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let item = Item {
                    name: self.state.newItem.clone(),
                };
                self.state.items.push(item);
                self.state.newItem = String::from("");
            }
            Msg::Update(updated_value) => {
                self.state.newItem = updated_value;
            }
            Msg::None => {}
        }
        true
    }

    // return true if properties change (we're not using any)
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input class="new-todo"
                    placeholder="What needs to be done?"
                    value=&self.state.newItem
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Add } else { Msg::None }
                    })
                />
                <button onclick=self.link.callback(|_| Msg::Add)>{"Add Item"}</button>
                <ul class="todo-list">
                    { for self.state.items.iter()
                        .enumerate()
                        .map(|val| self.view_entry(val)) }
                </ul>
                <p>{ "Testing" }</p>
            </div>
        }
    }
}

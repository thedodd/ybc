#![recursion_limit = "512"]

use strum::IntoEnumIterator;
use yew::{
    events::KeyboardEvent, html, web_sys::HtmlInputElement as InputElement, Classes, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender,
};

mod entry;
mod filter;
mod state;

use crate::entry::Entry;
use crate::filter::Filter;
use crate::state::State;

pub enum AppMsg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
    Noop,
}

pub struct App {
    pub link: ComponentLink<Self>,
    pub focus_ref: NodeRef,
    pub state: State,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entries = Vec::new();
        let focus_ref = NodeRef::default();
        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
        };

        Self { link, focus_ref, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppMsg::Add => {
                let description = self.state.value.trim();
                if !description.is_empty() {
                    let entry = Entry {
                        description: description.to_string(),
                        completed: false,
                        editing: false,
                    };
                    self.state.entries.push(entry);
                }
                self.state.value = "".to_string();
            }
            AppMsg::Edit(idx) => {
                let val = self.state.edit_value.trim().to_string();
                self.state.complete_edit(idx, val);
                self.state.edit_value = "".to_string();
            }
            AppMsg::Update(val) => self.state.value = val,
            AppMsg::UpdateEdit(val) => self.state.edit_value = val,
            AppMsg::Remove(idx) => self.state.remove(idx),
            AppMsg::SetFilter(filter) => self.state.filter = filter,
            AppMsg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }
            AppMsg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
            }
            AppMsg::Toggle(idx) => self.state.toggle(idx),
            AppMsg::ClearCompleted => self.state.clear_completed(),
            AppMsg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
            AppMsg::Noop => (),
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let is_hidden = if self.state.entries.is_empty() { "hidden" } else { "" };

        html! {
        <>
            <ybc::Columns>
                <ybc::Column classes="is-half is-offset-one-quarter">
                    <ybc::Section>
                        <ybc::Card>
                            <ybc::CardHeader>
                                <p class="card-header-title">
                                    {"TodoMVC"}
                                </p>
                            </ybc::CardHeader>
                            <ybc::CardContent>
                                { self.view_input() }
                            </ybc::CardContent>
                            <ybc::CardFooter>
                                <p class="card-footer-item">
                                    <strong>{ self.state.total() }</strong>
                                    { " item(s) left" }
                                </p>
                            </ybc::CardFooter>
                        </ybc::Card>
                    </ybc::Section>
                    <ybc::Section classes=is_hidden>
                        {
                            for self.state.entries.iter()
                                .filter(|entry| self.state.filter.fits(entry))
                                .enumerate()
                                .map(|entry| self.view_entry(entry))
                        }
                        <hr />
                        <ybc::Level>
                            <ybc::LevelLeft>
                                {
                                    for Filter::iter()
                                    .map(|filter| self.view_filter(filter))
                                }
                            </ybc::LevelLeft>
                            <ybc::LevelRight>
                                <ybc::Button classes="card-footer-item is-danger"
                                    onclick=self.link.callback(|_| AppMsg::ClearCompleted)>
                                    {
                                        format!("Clear completed ({})", self.state.total_completed())
                                    }
                                </ybc::Button>
                            </ybc::LevelRight>
                        </ybc::Level>
                    </ybc::Section>
                </ybc::Column>
            </ybc::Columns>
            <ybc::Footer classes="has-text-centered">
                <p>{"Double-click to edit a todo"}</p>
                <p><a href="https://github.com/thedodd/ybc">{"❤️ YBC ❤️"}</a></p>
            </ybc::Footer>
        </>
        }
    }
}

impl App {
    fn view_filter(&self, filter: Filter) -> Html {
        let mut class = Classes::from("button");
        if self.state.filter == filter {
            class.push(" is-active");
        } else {
            class.push("");
        }
        html! {
            <a class=class href=filter.as_href()
                onclick=self.link.callback(move |_| AppMsg::SetFilter(filter))>
                { filter }
            </a>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            <input class="input"
                type="text"
                placeholder="What needs to be done?"
                value=&self.state.value
                oninput=self.link.callback(|ev: InputData| AppMsg::Update(ev.value))
                onkeypress=self.link.batch_callback(|event: KeyboardEvent| {
                    //@TODO: Change AppMsg to be returned as an Option once Yew releases
                    if event.key() == "Enter" { vec![AppMsg::Add] } else { vec![AppMsg::Noop] }
                })
            />
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        html! {
            <ybc::Media>
                <ybc::MediaLeft>
                    <ybc::Checkbox name="chkbx"
                        checked=entry.completed
                        update=self.link.callback(move |_| AppMsg::Toggle(idx))
                    >
                    </ybc::Checkbox>
                </ybc::MediaLeft>
                <ybc::MediaContent>
                    <label ondblclick=self.link.callback(move |_| AppMsg::ToggleEdit(idx))>
                        { &entry.description }
                    </label>
                    { self.view_entry_edit_input((idx, &entry))}
                </ybc::MediaContent>
                <ybc::MediaRight>
                    <a class="delete" onclick=self.link.callback(move |_| AppMsg::Remove(idx))>
                    </a>
                </ybc::MediaRight>
            </ybc::Media>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {

                <input name="edit-input"
                    class="input"
                    type="text"
                    ref=self.focus_ref.clone()
                    value=&self.state.edit_value
                    onmouseover=self.link.callback(|_| AppMsg::Focus)
                    oninput=self.link.callback(|ev: InputData| AppMsg::UpdateEdit(ev.value))
                    onblur=self.link.callback(move |_| AppMsg::Edit(idx))
                    onkeypress=self.link.batch_callback(move |ev: KeyboardEvent| {
                        //@TODO: Change AppMsg to be returned as an Option once Yew releases
                        if ev.key() == "Enter" { vec![AppMsg::Edit(idx)] } else { vec![AppMsg::Noop] }
                    })
                />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

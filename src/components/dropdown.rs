use yew::prelude::*;

use crate::elements::button::Button;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DropdownProps {
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Any additional classes to use for the trigger button.
    #[prop_or_default]
    pub button_classes: Option<Classes>,
    /// The content of the trigger button.
    #[prop_or_default]
    pub button_html: Html,
}

/// Dropdown actions.
pub enum DropdownMsg {
    Open,
    Close,
}

/// An interactive dropdown menu for discoverable content.
///
/// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
pub struct Dropdown {
    is_menu_active: bool,
}

impl Component for Dropdown {
    type Message = DropdownMsg;
    type Properties = DropdownProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_menu_active: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().hoverable {
            return false;
        }
        match msg {
            DropdownMsg::Open => self.is_menu_active = true,
            DropdownMsg::Close => self.is_menu_active = false,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut class = Classes::from("dropdown");
        class.push(&ctx.props().classes);
        let opencb = if ctx.props().hoverable {
            class.push("is-hoverable");
            Callback::noop()
        } else {
            ctx.link().callback(|_| DropdownMsg::Open)
        };
        let overlay = if self.is_menu_active {
            class.push("is-active");
            html! {<div onclick={ctx.link().callback(|_| DropdownMsg::Close)} style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"></div>}
        } else {
            html! {}
        };
        html! {
            <div {class}>
                {overlay}
                <div class="dropdown-trigger">
                    <Button classes={ctx.props().button_classes.clone()} onclick={opencb}>
                        {ctx.props().button_html.clone()}
                    </Button>
                </div>
                <div class="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                        {ctx.props().children.clone()}
                    </div>
                </div>
            </div>
        }
    }
}

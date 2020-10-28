use yew::prelude::*;
use yewtil::NeqAssign;

use crate::elements::button::Button;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DropdownProps {
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub id: String,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Any additional classes to use for the trigger button.
    #[prop_or_default]
    pub button_classes: Option<String>,
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
    link: ComponentLink<Self>,
    props: DropdownProps,
    is_menu_active: bool,
}

impl Component for Dropdown {
    type Message = DropdownMsg;
    type Properties = DropdownProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            is_menu_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.props.hoverable {
            return false;
        }
        match msg {
            DropdownMsg::Open => self.is_menu_active = true,
            DropdownMsg::Close => self.is_menu_active = false,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("dropdown");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let opencb = if self.props.hoverable {
            classes.push("is-hoverable");
            Callback::noop()
        } else {
            self.link.callback(|_| DropdownMsg::Open)
        };
        let overlay = if self.is_menu_active {
            classes.push("is-active");
            html! {<div onclick=self.link.callback(|_| DropdownMsg::Close) style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"></div>}
        } else {
            html! {}
        };
        let id = &self.props.id;
        html! {
            <div class=classes id=id>
                {overlay}
                <div class="dropdown-trigger">
                    <Button classes=self.props.button_classes.clone() onclick=opencb>
                        {self.props.button_html.clone()}
                    </Button>
                </div>
                <div class="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                        {self.props.children.clone()}
                    </div>
                </div>
            </div>
        }
    }
}

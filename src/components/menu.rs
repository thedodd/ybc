use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A simple menu, for any type of vertical navigation.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
        <aside class={classes!("menu", &props.classes)}>
            {props.children.clone()}
        </aside>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuListProps {
    /// The child `li` elements of this list.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for menu list `li` elements.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuList)]
pub fn menu_list(props: &MenuListProps) -> Html {
    html! {
        <ul class={classes!("menu-list", &props.classes)}>
            {props.children.clone()}
        </ul>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuLabelProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The text of the label.
    #[prop_or_default]
    pub text: String,
}

/// A label for a section of the menu.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
#[function_component(MenuLabel)]
pub fn menu_label(props: &MenuLabelProps) -> Html {
    html! {
        <p class={classes!("menu-label", &props.classes)}>
            {props.text.clone()}
        </p>
    }
}

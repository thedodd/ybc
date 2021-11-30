use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Align child columns vertically.
    #[prop_or_default]
    pub vcentered: bool,
    /// Allow for multiline rows.
    #[prop_or_default]
    pub multiline: bool,
    /// Center all child columns within their row.
    #[prop_or_default]
    pub centered: bool,
}

/// The container for a set of responsive columns.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
#[function_component(Columns)]
pub fn columns(props: &ColumnsProps) -> Html {
    let class = classes!(
        "columns",
        props.classes.clone(),
        props.vcentered.then(|| "is-vcentered"),
        props.multiline.then(|| "is-multiline"),
        props.centered.then(|| "is-centered"),
    );
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A flexbox-based responsive column.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
///
/// This component has a very large number of valid class combinations which users may want.
/// Modelling all of these is particularly for this component, so for now you are encouraged to
/// add classes to this Component manually via the `classes` prop.
#[function_component(Column)]
pub fn column(props: &ColumnProps) -> Html {
    html! {
        <div class={classes!("column", props.classes.clone())}>
            {props.children.clone()}
        </div>
    }
}

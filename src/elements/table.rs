use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: bool,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: bool,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: bool,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: bool,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop_or_default]
    pub scrollable: bool,
}

/// An HTML table component.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let class = classes!(
        "table",
        &props.classes,
        props.bordered.then(|| "is-bordered"),
        props.striped.then(|| "is-striped"),
        props.narrow.then(|| "is-narrow"),
        props.hoverable.then(|| "is-hoverable"),
        props.fullwidth.then(|| "is-fullwidth"),
    );
    if props.scrollable {
        html! {
            <div class="table-container">
                <table {class}>
                    {props.children.clone()}
                </table>
            </div>
        }
    } else {
        html! {
            <table {class}>
                {props.children.clone()}
            </table>
        }
    }
}

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
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
pub struct Table {
    props: TableProps,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("table");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if self.props.bordered {
            classes.push("is-bordered");
        }
        if self.props.striped {
            classes.push("is-striped");
        }
        if self.props.narrow {
            classes.push("is-narrow");
        }
        if self.props.hoverable {
            classes.push("is-hoverable");
        }
        if self.props.fullwidth {
            classes.push("is-fullwidth");
        }
        if self.props.scrollable {
            html! {
                <div class="table-container">
                    <table class=classes>
                        {self.props.children.clone()}
                    </table>
                </div>
            }
        } else {
            html! {
                <table class=classes>
                    {self.props.children.clone()}
                </table>
            }
        }
    }
}

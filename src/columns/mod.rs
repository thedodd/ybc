use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// The container for a set of responsive columns.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
pub struct Columns {
    props: ColumnsProps,
}

impl Component for Columns {
    type Message = ();
    type Properties = ColumnsProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("columns");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// A flexbox-based responsive column.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
///
/// This component has a very large number of valid class combinations which users may want.
/// Modelling all of these is particularly for this component, so for now you are encouraged to
/// add classes to this Component manually via the `classes` prop.
pub struct Column {
    props: ColumnProps,
}

impl Component for Column {
    type Message = ();
    type Properties = ColumnProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self{props}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("column");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        html!{
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

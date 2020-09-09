use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub with_container: bool,
}

pub struct Table {
    props: TableProps,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self{props}
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
        if self.props.with_container {
            html!{
                <div class="table-container">
                    <table class=classes>
                        {self.props.children.clone()}
                    </table>
                </div>
            }
        } else {
            html!{
                <table class=classes>
                    {self.props.children.clone()}
                </table>
            }
        }
    }
}

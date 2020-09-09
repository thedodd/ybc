use yew::prelude::*;
use yew::events::MouseEvent;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "button".into())]
    pub tag: String,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

pub struct Delete {
    props: DeleteProps,
}

impl Component for Delete {
    type Message = ();
    type Properties = DeleteProps;

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
        let mut classes = Classes::from("delete");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let tag = self.props.tag.clone();
        html!{
            <@{tag} class=classes onclick?=self.props.onclick.clone()>
                {self.props.children.clone()}
            </@>
        }
    }
}

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_else(|| "p".into())]
    pub tag: String,
}

pub struct Content {
    props: ContentProps,
}

impl Component for Content {
    type Message = ();
    type Properties = ContentProps;

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
        let mut classes = Classes::from("content");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        let tag = self.props.tag.clone();
        html!{
            <@{tag} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}

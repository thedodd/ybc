use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// An all-around flexible and composable component; this is the card container.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct Card {
    props: CardProps,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

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
        let mut classes = Classes::from("card");
        classes.push(&self.props.classes);
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for card header content; rendered as a horizontal bar with a shadow.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardHeader {
    props: CardHeaderProps,
}

impl Component for CardHeader {
    type Message = ();
    type Properties = CardHeaderProps;

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
        let mut classes = Classes::from("card-header");
        classes.push(&self.props.classes);
        html! {
            <header class=classes>
                {self.props.children.clone()}
            </header>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A fullwidth container for a responsive image.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardImage {
    props: CardImageProps,
}

impl Component for CardImage {
    type Message = ();
    type Properties = CardImageProps;

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
        let mut classes = Classes::from("card-image");
        classes.push(&self.props.classes);
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for any other content as the body of the card.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardContent {
    props: CardContentProps,
}

impl Component for CardContent {
    type Message = ();
    type Properties = CardContentProps;

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
        let mut classes = Classes::from("card-content");
        classes.push(&self.props.classes);
        html! {
            <div class=classes>
                {self.props.children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for card footer content; rendered as a horizontal list of controls.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
pub struct CardFooter {
    props: CardFooterProps,
}

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

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
        let mut classes = Classes::from("card-footer");
        classes.push(&self.props.classes);
        html! {
            <footer class=classes>
                {self.props.children.clone()}
            </footer>
        }
    }
}

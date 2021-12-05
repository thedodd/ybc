use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<ImageSize>,
}

/// A container for responsive images.
///
/// [https://bulma.io/documentation/elements/image/](https://bulma.io/documentation/elements/image/)
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let class = classes!("image", &props.classes, props.size.as_ref().map(|size| size.to_string()));
    html! {
        <figure {class}>
            {props.children.clone()}
        </figure>
    }
}

/// Available placeholder sizes for figures.
///
/// https://bulma.io/documentation/elements/image/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum ImageSize {
    #[display(fmt = "16x16")]
    Is16x16,
    #[display(fmt = "24x24")]
    Is24x24,
    #[display(fmt = "32x32")]
    Is32x32,
    #[display(fmt = "48x48")]
    Is48x48,
    #[display(fmt = "64x64")]
    Is64x64,
    #[display(fmt = "96x96")]
    Is96x96,
    #[display(fmt = "128x128")]
    Is128x128,
    #[display(fmt = "Square")]
    IsSquare,
    #[display(fmt = "1by1")]
    Is1by1,
    #[display(fmt = "5by4")]
    Is5by4,
    #[display(fmt = "4by3")]
    Is4by3,
    #[display(fmt = "3by2")]
    Is3by2,
    #[display(fmt = "5by3")]
    Is5by3,
    #[display(fmt = "16by9")]
    Is16by9,
    #[display(fmt = "2by1")]
    Is2by1,
    #[display(fmt = "3by1")]
    Is3by1,
    #[display(fmt = "4by5")]
    Is4by5,
    #[display(fmt = "3by4")]
    Is3by4,
    #[display(fmt = "2by3")]
    Is2by3,
    #[display(fmt = "3by5")]
    Is3by5,
    #[display(fmt = "9by16")]
    Is9by16,
    #[display(fmt = "1by2")]
    Is1by2,
    #[display(fmt = "1by3")]
    Is1by3,
}
